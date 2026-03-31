use std::env;
use std::io::{Error, ErrorKind, Result};

use orivo::cmds::{Cmd, help::Help, sync::Sync, tui::Tui, version::Version};
use orivo::config::Config;

fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        None | Some("tui") => Box::new(Tui::new(Config::load()?)).run(),
        Some("sync") => Box::new(Sync::new(Config::load()?)).run(),
        Some("version") | Some("--version") | Some("-V") => Box::new(Version::new()).run(),
        Some("help") | Some("--help") | Some("-h") => {
            let cmds: Vec<(&str, Box<dyn Cmd>)> = vec![
                ("tui", Box::new(Tui::new(Config::default())) as Box<dyn Cmd>),
                ("sync", Box::new(Sync::new(Config::default())) as Box<dyn Cmd>),
                ("version", Box::new(Version::new()) as Box<dyn Cmd>),
            ];
            Box::new(Help::new(&cmds)).run()
        }
        Some(unknown) => {
            eprintln!("unknown command: {unknown}");
            eprintln!("run `orivo help` for usage");
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }
}
