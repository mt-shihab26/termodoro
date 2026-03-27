use std::io::Result;

use termodoro::app::App;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new(&mut terminal);

    let result = app.run();

    ratatui::restore();

    result
}
