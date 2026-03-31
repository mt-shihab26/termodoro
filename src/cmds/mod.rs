pub mod help;
pub mod sync;
pub mod tui;
pub mod version;

use std::io::Result;

pub trait Cmd {
    fn run(&self) -> Result<()>;
    fn help(&self) -> &[&str];
}
