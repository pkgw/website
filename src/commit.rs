// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! The `commit` action.

use crate::{app::AppSession, errors::Result, Command};

#[derive(clap::Args, Debug)]
#[command()]
pub struct CommitArgs {
    #[arg(
        short = 'f',
        long,
        help = "Force operation even in unexpected conditions"
    )]
    force: bool,
}

impl Command for CommitArgs {
    /// For this command, we should be running in CI and on the `main` branch.
    /// Changes will have been made and `git add`-ed. We generate a new commit
    /// on the `deploy` branch that merges `main` into `deploy`, using the
    /// current index as the merged tree.
    fn execute(self) -> Result<i32> {
        let mut sess = AppSession::initialize_default()?;
        sess.ensure_ci_main_mode(self.force)?;
        sess.repo.make_deploy_commit()?;
        Ok(0)
    }
}
