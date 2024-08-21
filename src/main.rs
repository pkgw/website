// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! A deployment support tool for my personal website.
//!
//! This is strongly derived from Cranko. If this tool were written from scratch
//! a bunch of the structure would probably be simpler and more reckless.

use clap::Parser;
use std::process;

mod app;
mod errors;
mod logger;
mod repository;

#[derive(Debug, Parser)]
#[command(name = "deploytool")]
#[command(bin_name = "deploytool")]
enum DToolCli {
    Apply(ApplyArgs),
}

trait Command {
    fn execute(self) -> Result<i32, anyhow::Error>;
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
    fn execute(self) -> Result<i32, anyhow::Error> {
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
    fn execute(self) -> Result<i32, anyhow::Error> {
        let sess = app::AppSession::initialize_default()?;
        sess.ensure_fully_clean(self.force)?;
        sess.ensure_ci_main_mode(self.force)?;

        let tree = sess.repo.get_deploy_tree()?;
        let mut new_posts = vec![];

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

        if new_posts.is_empty() {
            return Ok(0);
        }

        Ok(0)
    }
}
