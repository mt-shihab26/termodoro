use std::io::Result;

use crate::tui::app::App;

fn run() -> Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new(&mut terminal);

    let result = app.run();

    ratatui::restore();

    result
}
