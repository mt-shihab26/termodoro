//! Binary entrypoint for the Orivo CLI.
//!
//! This crate-level binary documentation exists separately from `lib.rs`
//! because the executable is responsible for command routing rather than the
//! application's internal architecture.
//!
//! The entry flow is intentionally small:
//!
//! - read the first CLI argument
//! - dispatch to the matching command implementation
//! - load config and connect the database only for commands that need them
//! - return an `InvalidInput` error for unknown commands
//!
//! The default command is `tui`, which launches the interactive terminal UI.
//! Additional commands such as `seed`, `version`, and `help` are thin wrappers
//! around the command types defined in the library crate.

use std::{
    env,
    io::{Error, ErrorKind, Result},
};

use orivo::{
    cmds::{Cmd, help::Help, tui::Tui, version::Version},
    config::Config,
    utils::db,
};

/// Parses the CLI argument list and dispatches to the selected command.
fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        None | Some("tui") => Box::new(Tui::new(Config::load()?, db::connect()?)).run(),
        #[cfg(debug_assertions)]
        Some("seed") => Box::new(orivo::cmds::seed::Seed::new()).run(),
        Some("version") | Some("--version") | Some("-V") => Box::new(Version::new()).run(),
        Some("help") | Some("--help") | Some("-h") => help(),
        Some(cmd) => unknown(cmd),
    }
}

/// Builds and runs the aggregated help command.
fn help() -> Result<()> {
    #[cfg(debug_assertions)]
    let helps = [Tui::help, orivo::cmds::seed::Seed::help, Version::help, Help::help];
    #[cfg(not(debug_assertions))]
    let helps = [Tui::help, Version::help, Help::help];
    Box::new(Help::new(&helps)).run()
}

/// Prints an error for an unknown command and returns `InvalidInput`.
fn unknown(unknown: &str) -> Result<()> {
    eprintln!("unknown command: {unknown}");
    eprintln!("run `orivo help` for usage");
    Err(Error::from(ErrorKind::InvalidInput))
}
