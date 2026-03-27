use std::error::Error;

use termodoro::{app::App, cli::Cli};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cli = Cli::new();

    let mut app = App::new(&cli);

    app.run().await?;

    Ok(())
}
