//! Command line entry point for `kestrel`.

use std::process::ExitCode;

use clap::Parser;
use kestrel::prelude::*;
use tracing::Level;

/// Runs the crate and maps the outcome to a process exit code.
fn main() -> ExitCode {
    let args = Args::parse();
    let level = match args.verbose {
        0 => Level::WARN, // default: warnings + errors only
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE, // -vvv and beyond
    };
    tracing_subscriber::fmt().with_max_level(level).init();
    match kestrel::run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}: {err}", version());
            ExitCode::FAILURE
        }
    }
}
