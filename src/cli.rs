use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    /// Tick rate, i.e. number of ticks per second
    #[arg(short, long, value_name = "FLOAT", default_value_t = 4.0)]
    pub tick_rate: f64,

    /// Frame rate, i.e. number of frames per second
    #[arg(short, long, value_name = "FLOAT", default_value_t = 60.0)]
    pub frame_rate: f64,
}

impl Cli {
    pub fn new() -> Self {
        let args = Self::parse();

        Self {
            tick_rate: args.tick_rate,
            frame_rate: args.frame_rate,
        }
    }
}

pub fn version() -> String {
    let author = clap::crate_authors!();
    let version = env!("CARGO_PKG_VERSION");

    format!("{version}\n\nAuthors: {author}")
}
