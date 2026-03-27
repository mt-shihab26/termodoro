use std::io::Result;

use crate::cmd::Cmd;

pub struct Version;

impl Cmd for Version {
    fn run(&self) -> Result<()> {
        println!("{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
