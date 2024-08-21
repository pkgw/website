// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! State for the deployment tool.

use anyhow::anyhow;
use log::warn;

use crate::{errors::Result, repository::Repository};

/// The main Cranko CLI application state structure.
pub struct AppSession {
    /// The backing repository.
    pub repo: Repository,

    /// Information about the CI environment that we may be running in.
    ci_info: ci_info::types::CiInfo,
}

impl AppSession {
    /// Create a new app session with totally default parameters
    pub fn initialize_default() -> Result<Self> {
        let repo = Repository::open_from_env()?;
        let ci_info = ci_info::get();

        Ok(AppSession { repo, ci_info })
    }

    /// Characterize the repository environment in which this process is
    /// running.
    pub fn execution_environment(&self) -> Result<ExecutionEnvironment> {
        if !self.ci_info.ci {
            Ok(ExecutionEnvironment::NotCi)
        } else {
            let maybe_pr = self.ci_info.pr;
            let maybe_ci_branch: Option<&str> =
                self.ci_info.branch_name.as_ref().map(|s| s.as_ref());

            if maybe_ci_branch.is_none() {
                warn!("cannot determine the triggering branch name in this CI environment");
            }

            if let Some(true) = maybe_pr {
                warn!("we seem to be running in a pull request; treating as non-CI for safety/convenience");
                return Ok(ExecutionEnvironment::NotCi);
            }

            Ok(ExecutionEnvironment::MainCi)
        }
    }

    /// Check that the current process is running in the "main" CI environment.
    pub fn ensure_ci_main_mode(&self) -> Result<()> {
        match self.execution_environment()? {
            ExecutionEnvironment::NotCi => Err(anyhow!(
                "no CI environment detected; this is unexpected for this command",
            )),

            ExecutionEnvironment::MainCi => Ok(()),
        }
    }

    /// Check that the working tree is completely clean. We allow untracked and
    /// ignored files but otherwise don't want any modifications, etc. Returns
    /// Ok if clean, an Err downcastable to DirtyRepositoryError if not. The
    /// error may have a different cause if, e.g., there is an I/O failure.
    pub fn ensure_fully_clean(&self) -> Result<()> {
        use crate::repository::DirtyRepositoryError;

        if let Some(changed_path) = self.repo.check_if_dirty()? {
            Err(DirtyRepositoryError(changed_path).into())
        } else {
            Ok(())
        }
    }
}

/// Different categorizations of the environment in which the program is
/// running.
pub enum ExecutionEnvironment {
    /// The program is running in a CI environment on an update to the "main"
    /// branch.
    MainCi,

    /// The program does not appear to be running in a CI environment. We infer
    /// that we're running in an individual development environment.
    NotCi,
}
