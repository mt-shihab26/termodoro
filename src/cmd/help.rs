use std::collections::HashMap;
use std::io::Result;

use super::Cmd;

pub struct Help {
    lines: Vec<String>,
}

impl Help {
    pub fn new(cmds: &HashMap<&str, Box<dyn Cmd>>) -> Self {
        let mut lines: Vec<String> = cmds.values().map(|c| c.help().to_string()).collect();
        lines.sort();
        Self { lines }
    }
}

impl Cmd for Help {
    fn help(&self) -> &str {
        "help    Show help for all commands"
    }

    fn run(&self) -> Result<()> {
        println!("Usage: termodoro <command>\n");
        println!("Commands:");
        for line in &self.lines {
            println!("  {line}");
        }
        println!("  {}", self.help());
        Ok(())
    }
}
