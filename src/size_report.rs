// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! The `size-report` action.
//!
//! This measures the filesystem size of the rendered website content and
//! compares that value to the previously deployed version. The result is
//! reported as a comment on the associated GitHub pull request.
//!
//! This action is pretty much just a demonstration of how you can use "layered
//! metadata" approaches to construct diffs of build *outputs*, not just inputs,
//! in a GitHub pull request. There is a significant limitation, though, that to
//! work with untrusted PRs I think you'll need to put together a whole GitHub
//! App to safely handle permissions.

use byte_unit::{Byte, UnitType};
use log::info;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::{a_ok_or, app::AppSession, errors::Result, github::GitHubClient, Command};

// Hardcoding this due to laziness.
const REPO_SLUG: &str = "pkgw/website";

#[derive(clap::Args, Debug)]
#[command()]
pub struct SizeReportArgs {
    #[arg(
        short = 'f',
        long,
        help = "Force operation even in unexpected conditions"
    )]
    force: bool,

    #[arg(long, help = "The number of this pull request")]
    pr_number: usize,

    #[arg(long, help = "The location of the on-disk rendered website content")]
    content_path: PathBuf,

    #[arg(long, help = "The path to save the resulting metadata")]
    result_path: PathBuf,
}

impl Command for SizeReportArgs {
    fn execute(self) -> Result<i32> {
        let sess = AppSession::initialize_default()?;

        // Get the size recorded in the most recent deployment

        let path = "_output_treesize.txt";
        let tree = sess.repo.get_deploy_tree()?;

        let maybe_entry = match tree.get_path(Path::new(path)) {
            Ok(ent) => Some(ent),
            Err(err) => {
                if err.code() == git2::ErrorCode::NotFound {
                    None
                } else {
                    return Err(err.into());
                }
            }
        };

        let prev_size = if let Some(ent) = maybe_entry {
            let object = sess.repo.entry_to_object(&ent)?;
            let blob = a_ok_or!(
                object.as_blob();
                ["path `{}` should correspond to a Git blob but does not", path]
            );
            let content = std::str::from_utf8(blob.content())?;
            str::parse::<u64>(content)?
        } else {
            info!("no file `{}` in the deploy tree; defaulting to 0", path);
            0
        };

        // Get the current size

        let mut cur_size = 0;

        for entry in WalkDir::new(self.content_path) {
            let entry = entry?;

            if !entry.file_type().is_file() {
                continue;
            }

            let md = entry.metadata()?;
            cur_size += md.len();
        }

        // Report as a GitHub comment.

        let prev_unit = Byte::from_u64(prev_size).get_appropriate_unit(UnitType::Binary);
        let cur_unit = Byte::from_u64(cur_size).get_appropriate_unit(UnitType::Binary);

        let size_cmp = if cur_size > prev_size {
            format!(
                "increase of {}",
                Byte::from_u64(cur_size - prev_size).get_appropriate_unit(UnitType::Binary)
            )
        } else if cur_size < prev_size {
            format!(
                "decrease of {}",
                Byte::from_u64(prev_size - cur_size).get_appropriate_unit(UnitType::Binary)
            )
        } else {
            "no change".to_owned()
        };

        let markdown = format!(
            "\
            ### Size Report (automated comment)
\n\
            Size of the on-disk website content:
\n\
            | When  |  Size |\n\
            | ----  |  ---- |\n\
            | Previous | {} |\n\
            | Current | {} |\n\
            | Change | **{}** |\n\
        ",
            prev_unit, cur_unit, size_cmp
        );

        let ghcl = GitHubClient::new()?;
        let mut http = ghcl.make_blocking_client()?;

        ghcl.create_comment(REPO_SLUG, self.pr_number, markdown, &mut http)?;

        // Save the new metadata. We don't write directly into the local
        // repository since it is possible that a variety of pieces of metadata
        // will need to be assembled and committed.

        let mut f_report = File::create(self.result_path)?;
        writeln!(f_report, "{}", cur_size)?;

        Ok(0)
    }
}
