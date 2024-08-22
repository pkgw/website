// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! A deployment support tool for my personal website.
//!
//! This is strongly derived from Cranko. If this tool were written from scratch
//! a bunch of the structure would probably be simpler and more reckless.

use clap::Parser;
use log::warn;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
    process,
};
use time::OffsetDateTime;

mod app;
mod errors;
mod logger;
mod repository;

use errors::Result;

#[derive(Debug, Parser)]
#[command(name = "deploytool")]
#[command(bin_name = "deploytool")]
enum DToolCli {
    Apply(ApplyArgs),
}

trait Command {
    fn execute(self) -> Result<i32>;
}

#[derive(clap::Args, Debug)]
#[command()]
struct ApplyArgs {
    #[arg(
        short = 'f',
        long,
        help = "Force operation even in unexpected conditions"
    )]
    force: bool,
}

impl Command for DToolCli {
    fn execute(self) -> Result<i32> {
        match self {
            Self::Apply(o) => o.execute(),
        }
    }
}

fn main() {
    let opts = DToolCli::parse();

    if let Err(e) = logger::Logger::init() {
        eprintln!("error: cannot initialize logging backend: {}", e);
        process::exit(1);
    }
    log::set_max_level(log::LevelFilter::Info);

    process::exit(errors::report(opts.execute()));
}

impl Command for ApplyArgs {
    /// For this command, we should be running in CI and on the `main` branch.
    /// We compare the contents of `main`/HEAD to the `deploy` branch, and apply
    /// publication metadata based on any differences: a new date for any new
    /// files, and an "updated" date for any files modified since the last
    /// deploy.
    fn execute(self) -> Result<i32> {
        let sess = app::AppSession::initialize_default()?;
        sess.ensure_fully_clean(self.force)?;
        sess.ensure_ci_main_mode(self.force)?;

        let tree = sess.repo.get_deploy_tree()?;
        let mut new_posts = vec![];

        let pubdate = time::OffsetDateTime::now_utc();

        sess.repo.scan_paths(|p| {
            if !p.starts_with("content/") || !p.ends_with(".md") {
                return Ok(());
            }

            let found = match tree.get_path(p.as_path()) {
                Ok(_) => true,
                Err(e) => {
                    if e.code() == git2::ErrorCode::NotFound {
                        false
                    } else {
                        return Err(e.into());
                    }
                }
            };

            if !found {
                new_posts.push(p.to_owned());
            }

            Ok(())
        })?;

        println!("Detected {} new pages.", new_posts.len());

        for repopath in &new_posts {
            let fspath = sess.repo.resolve_workdir(repopath);

            atry!(
                self.timestamp_new_page(&fspath, pubdate);
                ["failed to timestamp `{}`", fspath.display()]
            );
        }

        Ok(0)
    }
}

impl ApplyArgs {
    fn timestamp_new_page(&self, path: &Path, pubdate: OffsetDateTime) -> Result<()> {
        use time::format_description::well_known::iso8601::{
            Config, EncodedConfig, Iso8601, TimePrecision,
        };
        const FMT_CONFIG: EncodedConfig = Config::DEFAULT
            .set_time_precision(TimePrecision::Second {
                decimal_digits: None,
            })
            .encode();
        let fmt = Iso8601::<FMT_CONFIG>;

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

                            let deploydate = OffsetDateTime::new_in_offset(
                                pubdate.date(),
                                pubdate.time(),
                                devdate.offset(),
                            );

                            atry!(
                                writeln!(
                                    new_f,
                                    "date = {} # deploytool",
                                    deploydate.format(&fmt).unwrap()
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
