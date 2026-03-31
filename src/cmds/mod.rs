pub mod help;
pub mod sync;
pub mod tui;
pub mod version;

use std::io::Result;

use self::{help::Help, sync::Sync, tui::Tui, version::Version};

pub trait Cmd {
    fn run(self: Box<Self>) -> Result<()>;
    fn help() -> &'static [&'static str];
}
