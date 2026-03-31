use std::env;
use std::io::{Error, ErrorKind, Result};

use orivo::cmds::{Cmd, help::Help, tui::Tui, version::Version};
use orivo::config::Config;
use orivo::utils::db;

fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        None | Some("tui") => Box::new(Tui::new(Config::load()?, db::connect()?)).run(),
        Some("version") | Some("--version") | Some("-V") => Box::new(Version::new()).run(),
        Some("help") | Some("--help") | Some("-h") => help(),
        Some(cmd) => unknown(cmd),
    }
}

fn help() -> Result<()> {
    let helps = [Tui::help, Version::help, Help::help];
    Box::new(Help::new(&helps)).run()
}

fn unknown(unknown: &str) -> Result<()> {
    eprintln!("unknown command: {unknown}");
    eprintln!("run `orivo help` for usage");
    Err(Error::from(ErrorKind::InvalidInput))
}
