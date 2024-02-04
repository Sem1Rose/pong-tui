use crate::{app::App, drawer, event_handler};
use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::Backend, prelude::*};
use std::io::{stdout, Result};
use std::time::{Duration, Instant};

pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    frame_time: Duration,
    now: Instant,
    last_update: Instant,
}

impl<B: Backend> Tui<B> {
    pub fn new(_terminal: Terminal<B>) -> Self {
        Self {
            terminal: _terminal,
            frame_time: Duration::from_secs_f32(1.0 / 60.0),
            now: Instant::now(),
            last_update: Instant::now(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn run(&mut self, app: &mut App) -> Result<()> {
        loop {
            self.now = Instant::now();
            if app.should_quit {
                return Ok(());
            }

            if app.game_started && !app.game.game_paused {
                self.frame_time =
                    Duration::from_secs_f32(1.0 / (app.difficulty as u8 * 4 + 10) as f32);
            } else {
                self.frame_time = Duration::from_secs_f32(1.0 / 60.0);
            }

            let mut update_b = false;
            self.handle_input(app)?;
            app.game.check_game_over();

            if self.now.duration_since(self.last_update) >= self.frame_time {
                self.last_update = Instant::now();
                update_b = true;
            }
            self.draw(app, update_b)?;
        }
    }

    pub fn exit(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn handle_input(&mut self, app: &mut App) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(0))? {
            return event_handler::handle(event::read()?, app);
        }
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App, update_b: bool) -> Result<CompletedFrame> {
        self.terminal.draw(|frame| drawer::ui(frame, app, update_b))
    }
}
