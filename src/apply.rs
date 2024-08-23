// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! The `apply` action.

use anyhow::bail;
use log::warn;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};
use time::{
    format_description::well_known::iso8601::{Config, EncodedConfig, Iso8601, TimePrecision},
    OffsetDateTime,
};

use crate::{a_ok_or, app::AppSession, atry, errors::Result, Command};

const FMT_CONFIG: EncodedConfig = Config::DEFAULT
    .set_time_precision(TimePrecision::Second {
        decimal_digits: None,
    })
    .encode();

const TIME_FORMAT: Iso8601<FMT_CONFIG> = Iso8601;

#[derive(clap::Args, Debug)]
#[command()]
pub struct ApplyArgs {
    #[arg(
        short = 'f',
        long,
        help = "Force operation even in unexpected conditions"
    )]
    force: bool,
}

impl Command for ApplyArgs {
    /// For this command, we should be running in CI and on the `main` branch.
    /// We compare the contents of `main`/HEAD to the `deploy` branch, and apply
    /// publication metadata. A new date for any new files, an "updated" date
    /// for any files modified since the last deploy, and historical versions of
    /// these values as needed for everything else.
    fn execute(self) -> Result<i32> {
        let sess = AppSession::initialize_default()?;
        sess.ensure_fully_clean(self.force)?;
        sess.ensure_ci_main_mode(self.force)?;

        let tree = sess.repo.get_deploy_tree()?;
        let mut new_posts = vec![];
        let mut old_posts = vec![];

        let pubdate = time::OffsetDateTime::now_utc();

        sess.repo.scan_paths(|p| {
            if !p.starts_with("content/") || !p.ends_with(".md") {
                return Ok(());
            }

            let maybe_entry = match tree.get_path(p.as_path()) {
                Ok(ent) => Some(ent),
                Err(err) => {
                    if err.code() == git2::ErrorCode::NotFound {
                        None
                    } else {
                        return Err(err.into());
                    }
                }
            };

            if let Some(ent) = maybe_entry {
                // We appear to have a pre-existing file. Check whether we need to
                // rewrite the working-tree version, by scanning the deploy content
                // for a deploytool-assigned timestamp.

                let object = sess.repo.entry_to_object(&ent)?;
                let blob = a_ok_or!(
                    object.as_blob();
                    ["path `{}` should correspond to a Git blob but does not", p.escaped()]
                );

                let maybe_timestamp = atry!(
                    self.timestamp_from_markdown(blob.content());
                    ["failed to scan deploy branch file `{}` for metadata", p.escaped()]
                );

                if let Some(ts) = maybe_timestamp {
                    old_posts.push((p.to_owned(), ts));
                }
            } else {
                new_posts.push(p.to_owned());
            }

            Ok(())
        })?;

        println!("Detected {} new pages.", new_posts.len());

        for repopath in &new_posts {
            let fspath = sess.repo.resolve_workdir(repopath);

            atry!(
                self.apply_metadata_to_page(&fspath, pubdate);
                ["failed to timestamp `{}`", fspath.display()]
            );
        }

        for (repopath, pagedate) in &old_posts {
            let fspath = sess.repo.resolve_workdir(repopath);

            atry!(
                self.apply_metadata_to_page(&fspath, *pagedate);
                ["failed to timestamp `{}`", fspath.display()]
            );
        }

        println!("Updated {} existing pages.", old_posts.len());

        Ok(0)
    }
}

impl ApplyArgs {
    /// Given a Zola content file, obtain its date stamp. We scan the file for
    /// `date = ...` frontmatter with a ` # deploytool` comment on the line. If
    /// no such comment is present, we return Ok(None). If something unexpected
    /// happens, we return an error.
    fn timestamp_from_markdown<R: Read>(&self, stream: R) -> Result<Option<OffsetDateTime>> {
        let reader = BufReader::new(stream);

        enum State {
            Start,
            Frontmatter,
        }

        let mut state = State::Start;

        for line in reader.lines() {
            let line: String = atry!(
                line;
                ["failed to read from the file"]
            );

            match state {
                State::Start => {
                    if line == "+++" {
                        state = State::Frontmatter;
                    } else {
                        // We could potentially return Ok(None) here
                        bail!("file does not appear to be Zola content (no `+++`)");
                    }
                }

                State::Frontmatter => {
                    if line == "+++" {
                        // No `date`; assume this is fine
                        return Ok(None);
                    } else if let Some(date_text) = line.strip_prefix("date = ") {
                        // If the annotation isn't there, treat this as a legacy
                        // file that doesn't need rewriting. That's fine.
                        if !date_text.contains("deploytool") {
                            return Ok(None);
                        }

                        let mut words_iter = date_text.split_whitespace();

                        if let Some(word) = words_iter.next() {
                            return Ok(Some(atry!(
                                OffsetDateTime::parse(word, &Iso8601::DEFAULT);
                                ["failed to parse ISO8601 `{}`", word]
                            )));
                        } else {
                            bail!("file has malformatted `date` frontmatter item");
                        }
                    }
                }
            }
        }

        // If we got here, we saw the first `+++` but not the second.
        bail!("file appears to be truncated Zola content");
    }

    fn apply_metadata_to_page(&self, path: &Path, pubdate: OffsetDateTime) -> Result<()> {
        // Rewrite the frontmatter manually instead of using something like
        // toml_edit. It should be fine.

        let old_f = atry!(
            File::open(path);
            ["failed to open the file"]
        );

        let old_reader = BufReader::new(old_f);

        enum State {
            Start,
            Frontmatter,
            Main,
        }

        let mut state = State::Start;

        let new_af =
            atomicwrites::AtomicFile::new(path, atomicwrites::OverwriteBehavior::AllowOverwrite);

        let r = new_af.write(|new_f| {
            for line in old_reader.lines() {
                let line: String = atry!(
                    line;
                    ["failed to read from the file"]
                );

                match state {
                    State::Start => {
                        if line == "+++" {
                            state = State::Frontmatter;
                            atry!(
                                writeln!(new_f, "{line}");
                                ["failed to write to new tempfile"]
                            );
                        } else {
                            warn!(
                                "file `{}` does not appear to be Zola content (no `+++`)",
                                path.display()
                            );
                            return Ok(());
                        }
                    }

                    State::Frontmatter => {
                        if line == "+++" {
                            state = State::Main;
                            atry!(
                                writeln!(new_f, "{line}");
                                ["failed to write to new tempfile"]
                            );
                        } else if let Some(datestr) = line.strip_prefix("date = ") {
                            let devdate = atry!(
                                OffsetDateTime::parse(datestr, &Iso8601::DEFAULT);
                                ["failed to parse ISO8601 `{}`", datestr]
                            );

                            let deploydate = pubdate.to_offset(devdate.offset());

                            atry!(
                                writeln!(
                                    new_f,
                                    "date = {} # deploytool",
                                    deploydate.format(&TIME_FORMAT).unwrap()
                                );
                                ["failed to write to new tempfile"]
                            );
                        } else {
                            atry!(
                                writeln!(new_f, "{line}");
                                ["failed to write to new tempfile"]
                            );
                        }
                    }

                    State::Main => {
                        atry!(
                            writeln!(new_f, "{line}");
                            ["failed to write to new tempfile"]
                        );
                    }
                }
            }

            Ok(())
        });

        match r {
            Err(atomicwrites::Error::Internal(e)) => Err(e.into()),
            Err(atomicwrites::Error::User(e)) => Err(e),
            Ok(()) => Ok(()),
        }
    }
}
