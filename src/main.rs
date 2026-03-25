mod config;
mod timer;
mod ui;

use config::load_config;
use timer::Timer;

fn main() {
    let config = load_config();

    let timer = Timer::new(config);

    if let Err(e) = ui::run(timer) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
