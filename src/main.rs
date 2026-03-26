use std::io::Result;

use termodoro::{app::App, cli::Cli};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::new();

    let mut app = App::new(&cli)?;

    app.run().await?;

    Ok(())
}
