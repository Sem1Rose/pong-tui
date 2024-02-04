mod event_handler;
mod game;
mod tui;
mod drawer;
mod font;
mod app;

use std::io::{stdout, Result};
use ratatui::prelude::*;
use tui::Tui;
use app::App;

fn main() -> Result<()> {
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut tui = Tui::new(terminal);
    tui.init()?;
    let mut app = App::new();

    let result = tui.run(&mut app);

    tui.exit()?;
    result?;
    Ok(())
}