// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! The `size-report` action.

use crate::{app::AppSession, errors::Result, github::GitHubClient, Command};

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
}

impl Command for SizeReportArgs {
    /// ...
    fn execute(self) -> Result<i32> {
        let sess = AppSession::initialize_default()?;
        sess.ensure_ci_main_mode(self.force)?;

        let ghcl = GitHubClient::new()?;
        let mut http = ghcl.make_blocking_client()?;

        ghcl.create_comment(
            "pkgw/website",
            self.pr_number,
            "This is my test comment".to_owned(),
            &mut http,
        )?;

        Ok(0)
    }
}
