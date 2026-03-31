use std::env;
use std::io::{Error, ErrorKind, Result};

use orivo::cmds::{Cmd, help::Help, sync::Sync, tui::Tui, version::Version};
use orivo::config::Config;

fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        None | Some("tui") => Box::new(Tui::new(Config::load()?)).run(),
        Some("sync") => Box::new(Sync::new(Config::load()?)).run(),
        Some("version") | Some("--version") | Some("-V") => Box::new(Version::new()).run(),
        Some("help") | Some("--help") | Some("-h") => Box::new(Help::new(&helps())).run(),
        Some(cmd) => unknown(cmd),
    }
}

fn helps() -> [&'static [&'static str]; 4] {
    [Tui, Sync, Version, Help]
}

fn unknown(unknown: &str) -> Result<()> {
    eprintln!("unknown command: {unknown}");
    eprintln!("run `orivo help` for usage");
    Err(Error::from(ErrorKind::InvalidInput))
}
