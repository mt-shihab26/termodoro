use std::env;
use std::io::{Error, ErrorKind, Result};

use orivo::cmds::{Cmd, help::Help, sync::Sync, tui::Tui, version::Version};
use orivo::config::Config;

fn help_lines() -> Vec<Vec<String>> {
    vec![
        Tui::new(Config::default())
            .help()
            .iter()
            .map(|s| s.to_string())
            .collect(),
        Sync::new(Config::default())
            .help()
            .iter()
            .map(|s| s.to_string())
            .collect(),
        Version::new().help().iter().map(|s| s.to_string()).collect(),
        Help::new(&vec![]).help().iter().map(|s| s.to_string()).collect(),
    ]
}

fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        None | Some("tui") => Box::new(Tui::new(Config::load()?)).run(),
        Some("sync") => Box::new(Sync::new(Config::load()?)).run(),
        Some("version") | Some("--version") | Some("-V") => Box::new(Version::new()).run(),
        Some("help") | Some("--help") | Some("-h") => Box::new(Help::new(&help_lines())).run(),
        Some(unknown) => {
            eprintln!("unknown command: {unknown}");
            eprintln!("run `orivo help` for usage");
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }
}
