// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! A deployment support tool for my personal website.

use clap::Parser;
use std::process;

mod errors;
mod logger;

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
struct ApplyArgs {}

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
    fn execute(self) -> Result<i32, anyhow::Error> {
        println!("hello");
        Ok(0)
    }
}
