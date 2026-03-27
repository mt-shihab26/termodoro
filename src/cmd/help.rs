use std::collections::HashMap;
use std::io::Result;

use super::Cmd;

pub struct Help<'a> {
    pub cmds: &'a HashMap<&'a str, Box<dyn Cmd>>,
}

impl<'a> Help<'a> {
    pub fn new(cmds: &'a HashMap<&'a str, Box<dyn Cmd>>) -> Self {
        Self { cmds }
    }
}

impl<'a> Cmd for Help<'a> {
    fn help(&self) -> &str {
        "help    Show help for all commands"
    }

    fn run(&self) -> Result<()> {
        println!("Usage: termodoro <command>\n");
        println!("Commands:");
        let mut lines: Vec<&str> = self.cmds.values().map(|c| c.help()).collect();
        lines.sort();
        for line in lines {
            println!("  {line}");
        }
        println!("  {}", self.help());
        Ok(())
    }
}
