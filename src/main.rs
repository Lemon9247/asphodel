mod map;
mod entity;
mod fov;
mod game;
mod ui;
mod spawn;

use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use game::{Game, Action};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut game = Game::new();
    game.log("You awaken in the Asphodel Meadows.");
    game.log("Grey flowers stretch endlessly. You remember nothing.");
    game.log("You are a shade. You will not last.");

    loop {
        terminal.draw(|frame| ui::draw(frame, &game))?;

        if game.is_over() {
            // Wait for any key to quit
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    break;
                }
            }
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            let action = match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Up    | KeyCode::Char('k') => Action::Move(0, -1),
                KeyCode::Down  | KeyCode::Char('j') => Action::Move(0, 1),
                KeyCode::Left  | KeyCode::Char('h') => Action::Move(-1, 0),
                KeyCode::Right | KeyCode::Char('l') => Action::Move(1, 0),
                KeyCode::Char('y') => Action::Move(-1, -1),
                KeyCode::Char('u') => Action::Move(1, -1),
                KeyCode::Char('b') => Action::Move(-1, 1),
                KeyCode::Char('n') => Action::Move(1, 1),
                KeyCode::Char('.') | KeyCode::Char('5') => Action::Wait,
                KeyCode::Char('g') => Action::Pickup,
                KeyCode::Char('d') => Action::Descend,
                _ => continue,
            };
            game.player_action(action);
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
