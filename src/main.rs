// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! A deployment support tool for my personal website.
//!
//! This is strongly derived from Cranko. If this tool were written from scratch
//! a bunch of the structure would probably be simpler and more reckless.

use clap::Parser;
use std::process;

mod app;
mod apply;
mod commit;
mod env;
mod errors;
mod github;
mod logger;
mod repository;

use apply::ApplyArgs;
use commit::CommitArgs;
use errors::Result;

#[derive(Debug, Parser)]
#[command(name = "deploytool")]
#[command(bin_name = "deploytool")]
enum DToolCli {
    Apply(ApplyArgs),
    Commit(CommitArgs),
}

trait Command {
    fn execute(self) -> Result<i32>;
}

impl Command for DToolCli {
    fn execute(self) -> Result<i32> {
        match self {
            Self::Apply(o) => o.execute(),
            Self::Commit(o) => o.execute(),
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
