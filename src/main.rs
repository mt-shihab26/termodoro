use std::collections::HashMap;
use std::io::{self, Result};

use termodoro::cmd::{Cmd, help::Help, tui::Tui, version::Version};

fn main() -> Result<()> {
    let cmds: HashMap<&str, Box<dyn Cmd>> = HashMap::from([
        ("tui",     Box::new(Tui)     as Box<dyn Cmd>),
        ("version", Box::new(Version) as Box<dyn Cmd>),
    ]);

    let arg = std::env::args().nth(1);

    let key = match arg.as_deref() {
        None | Some("tui")                                => "tui",
        Some("version") | Some("--version") | Some("-V") => "version",
        Some("help")    | Some("--help")    | Some("-h")  => {
            return Help::new(&cmds).run();
        }
        Some(unknown) => {
            eprintln!("unknown command: {unknown}");
            eprintln!("run `termodoro help` for usage");
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
    };

    cmds[key].run()
}
