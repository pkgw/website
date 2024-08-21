// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! A very simple logger.
//!
//! Overkill for the deploytool, but copied from Cranko and I'm keeping the
//! API to make it easier to reuse other pieces of the codebase.

use lazy_static::lazy_static;
use log::{Level, Log};
use std::{
    fmt::Display,
    io::{self, Write},
};

/// A simple logger.
pub struct Logger {}

lazy_static! {
    static ref LOGGER: Logger = Logger {};
}

impl Logger {
    /// Set up this type as the global static logger.
    pub fn init() -> Result<(), log::SetLoggerError> {
        log::set_logger(&*LOGGER)
    }

    pub fn print_cause(err: &(dyn std::error::Error + 'static)) {
        eprintln!("caused by: {}", err);
    }

    pub fn print_err_note<T: Display>(msg: T) {
        eprintln!("note: {}", msg);
    }

    pub fn println_highlighted<T1: Display, T2: Display, T3: Display>(
        before: T1,
        highlight: T2,
        after: T3,
    ) {
        println!("{}{}{}", before, highlight, after);
    }
}

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        // Rely on `log::set_max_level()` for filtering
        true
    }

    fn log(&self, record: &log::Record) {
        match record.level() {
            Level::Trace => {
                eprintln!("trace: {}", record.args());
            }

            Level::Debug => {
                eprintln!("debug: {}", record.args());
            }

            Level::Info => {
                println!("info: {}", record.args());
            }

            Level::Warn => {
                eprintln!("warning: {}", record.args());
            }

            Level::Error => {
                eprintln!("error: {}", record.args());
            }
        }
    }

    fn flush(&self) {
        let _r = io::stdout().flush();
    }
}
