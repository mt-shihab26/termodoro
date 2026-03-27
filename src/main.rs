use std::collections::HashMap;
use std::io::{self, Result};

use termodoro::cmd::{Cmd, tui::Tui, version::Version};

fn main() -> Result<()> {
    let cmds: HashMap<&str, Box<dyn Cmd>> = HashMap::from([
        ("tui", Box::new(Tui) as Box<dyn Cmd>),
        ("version", Box::new(Version) as Box<dyn Cmd>),
    ]);

    let arg = std::env::args().nth(1);

    let key = match arg.as_deref() {
        None | Some("tui") => "tui",
        Some("version") | Some("--version") | Some("-V") => "version",
        Some(unknown) => {
            eprintln!("unknown command: {unknown}");
            eprintln!("available: {}", cmds.keys().cloned().collect::<Vec<_>>().join(", "));
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
    };

    cmds[key].run()
}
