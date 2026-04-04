use std::io::Result;

use super::Cmd;

/// Command that prints the global CLI help output.
pub struct Help {
    /// Help lines collected from all registered commands.
    lines: Vec<Vec<String>>,
}

impl Help {
    /// Creates a new `Help` command from the registered command help providers.
    pub fn new(helps: &[fn() -> &'static [&'static str]]) -> Self {
        let lines = helps
            .iter()
            .map(|help| help().iter().map(|s| s.to_string()).collect())
            .collect();

        Self { lines }
    }

    /// Formats one command help entry into a printable row.
    fn format(entries: &[&str]) -> String {
        let (description, names) = entries.split_last().unwrap_or((&"", &[]));
        format!("{:<30} {}", names.join(", "), description)
    }
}

impl Cmd for Help {
    /// Returns the CLI aliases and description for the help command.
    fn help() -> &'static [&'static str] {
        &["help", "--help", "-h", "Show help for all commands"]
    }

    /// Prints usage text followed by all registered command descriptions.
    fn run(self: Box<Self>) -> Result<()> {
        println!("Usage: {} <command>\n", env!("CARGO_PKG_NAME"));
        println!("Commands:");
        for entries in &self.lines {
            let strs: Vec<&str> = entries.iter().map(|s| s.as_str()).collect();
            println!("  {}", Self::format(&strs));
        }
        Ok(())
    }
}
