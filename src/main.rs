use std::env;
use std::io::{Error, ErrorKind, Result};

use orivo::cmds::{Cmd, help::Help, sync::Sync, tui::Tui, version::Version};

fn main() -> Result<()> {
    let cmds: Vec<(&str, Box<dyn Cmd>)> = vec![
        ("tui", Box::new(Tui::new()) as Box<dyn Cmd>),
        ("sync", Box::new(Sync::new()) as Box<dyn Cmd>),
        ("version", Box::new(Version::new()) as Box<dyn Cmd>),
        ("help", Box::new(Help::new(&vec![])) as Box<dyn Cmd>),
    ];

    let arg = env::args().nth(1);

    let key = match arg.as_deref() {
        None | Some("tui") => "tui",
        Some("sync") => "sync",
        Some("version") | Some("--version") | Some("-V") => "version",
        Some("help") | Some("--help") | Some("-h") => return Help::new(&cmds).run(),
        Some(unknown) => {
            eprintln!("unknown command: {unknown}");
            eprintln!("run `orivo help` for usage");
            return Err(Error::from(ErrorKind::InvalidInput));
        }
    };

    for (name, cmd) in &cmds {
        if *name == key {
            return cmd.run();
        }
    }

    Err(Error::from(ErrorKind::InvalidInput))
}
