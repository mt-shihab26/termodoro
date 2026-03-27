use std::collections::HashMap;
use std::io::Result;

use super::Cmd;

pub struct Help {
    lines: Vec<Vec<String>>,
}

impl Help {
    pub fn new(cmds: &HashMap<&str, Box<dyn Cmd>>) -> Self {
        let mut lines: Vec<Vec<String>> = cmds
            .values()
            .map(|c| c.help().iter().map(|s| s.to_string()).collect())
            .collect();

        lines.sort_by(|a, b| a[0].cmp(&b[0]));

        Self { lines }
    }

    fn format(entries: &[&str]) -> String {
        let (description, names) = entries.split_last().unwrap_or((&"", &[]));
        format!("{:<30} {}", names.join(", "), description)
    }
}

impl Cmd for Help {
    fn help(&self) -> &[&str] {
        &["help", "--help", "-h", "Show help for all commands"]
    }

    fn run(&self) -> Result<()> {
        println!("Usage: termodoro <command>\n");
        println!("Commands:");
        for entries in &self.lines {
            let strs: Vec<&str> = entries.iter().map(|s| s.as_str()).collect();
            println!("  {}", Self::format(&strs));
        }
        println!("  {}", Self::format(self.help()));
        Ok(())
    }
}
