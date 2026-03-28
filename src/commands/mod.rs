pub mod help;
pub mod tui;
pub mod version;

use std::io::Result;

pub trait Command {
    fn run(&self) -> Result<()>;
    fn help(&self) -> &[&str];
}
