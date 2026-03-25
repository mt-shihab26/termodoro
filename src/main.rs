mod config;
mod logger;
mod state;
mod timer;
mod ui;

use config::load_config;
use state::load_state;
use timer::Timer;

fn main() {
    let config = load_config();
    let state = load_state();

    let timer = Timer::new(config, state);

    if let Err(e) = ui::run(timer) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
