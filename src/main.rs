use clap::Parser;
use termodoro::{app::App, cli::Cli};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}
