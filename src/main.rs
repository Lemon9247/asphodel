mod map;
mod entity;
mod fov;
mod game;
mod ui;
mod spawn;
mod screen;
mod ability;
mod shrine;
mod flavor;

use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use game::{Game, Action, GameState};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut state = GameState::Title;

    // Pre-create game so we don't need Option
    let mut game = Game::new();

    loop {
        match state {
            GameState::Title => {
                terminal.draw(|frame| {
                    let area = centered_rect(60, 22, frame.area());
                    frame.render_widget(screen::title_widget(area), area);
                })?;

                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                            break;
                        }
                        game = Game::new();
                        game.log("You awaken in the Asphodel Meadows.");
                        game.log("Grey flowers stretch endlessly. You remember nothing.");
                        game.log("You are a shade. You will not last.");
                        game.log("[1] Dash  [2] Spectral Scream  [3] Lethe's Touch  [e] Shrine");
                        state = GameState::Playing;
                    }
                }
            }

            GameState::Playing => {
                terminal.draw(|frame| ui::draw(frame, &game))?;

                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    // Check if at shrine prompt
                    if game.shrine_prompt.is_some() {
                        match key.code {
                            KeyCode::Char('a') => { game.choose_shrine_boon(true); }
                            KeyCode::Char('b') => { game.choose_shrine_boon(false); }
                            KeyCode::Esc => { game.shrine_prompt = None; game.log("You step away from the shrine."); }
                            _ => {}
                        }
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
                        KeyCode::Char('e') => Action::Interact,
                        KeyCode::Char('x') => Action::Look,
                        KeyCode::Char('1') => Action::UseAbility(0),
                        KeyCode::Char('2') => Action::UseAbility(1),
                        KeyCode::Char('3') => Action::UseAbility(2),
                        _ => continue,
                    };
                    game.player_action(action);

                    if game.game_over {
                        state = if game.victory { GameState::Victory } else { GameState::Dead };
                    }
                }
            }

            GameState::Dead => {
                terminal.draw(|frame| {
                    let area = centered_rect(60, 20, frame.area());
                    frame.render_widget(
                        screen::death_widget(game.depth, game.obols, game.turns, &game.killed_by),
                        area,
                    );
                })?;

                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        break;
                    }
                }
            }

            GameState::Victory => {
                terminal.draw(|frame| {
                    let area = centered_rect(60, 22, frame.area());
                    frame.render_widget(
                        screen::victory_widget(game.obols, game.turns),
                        area,
                    );
                })?;

                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        break;
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

/// Create a centered rectangle
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
