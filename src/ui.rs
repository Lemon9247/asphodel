use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Gauge};
use crate::game::Game;
use crate::map::{MAP_W, MAP_H, Tile};

pub fn draw(frame: &mut Frame, game: &Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),      // map
            Constraint::Length(3),    // stats bar
            Constraint::Length(10),   // log
        ])
        .split(frame.area());

    draw_map(frame, chunks[0], game);
    draw_stats(frame, chunks[1], game);
    draw_log(frame, chunks[2], game);
}

fn draw_map(frame: &mut Frame, area: Rect, game: &Game) {
    let map = &game.map;

    // Camera: center on player
    let cam_x = (game.player.x as i32) - (area.width as i32 / 2);
    let cam_y = (game.player.y as i32) - (area.height as i32 / 2);

    let buf = frame.buffer_mut();

    for dy in 0..area.height {
        for dx in 0..area.width {
            let mx = cam_x + dx as i32;
            let my = cam_y + dy as i32;
            let cell_x = area.x + dx;
            let cell_y = area.y + dy;

            if mx < 0 || my < 0 || mx >= MAP_W as i32 || my >= MAP_H as i32 {
                buf[(cell_x, cell_y)].set_char(' ').set_bg(Color::Black);
                continue;
            }

            let ux = mx as usize;
            let uy = my as usize;

            if map.visible[uy][ux] {
                // Check for entities here
                if game.player.x == mx && game.player.y == my {
                    buf[(cell_x, cell_y)]
                        .set_char('@')
                        .set_fg(Color::Yellow)
                        .set_bg(Color::Black);
                } else if let Some(ent) = game.entities.iter().find(|e| e.alive && e.x == mx && e.y == my) {
                    buf[(cell_x, cell_y)]
                        .set_char(ent.kind.glyph())
                        .set_fg(ent.kind.color())
                        .set_bg(Color::Black);
                } else {
                    let tile = map.tiles[uy][ux];
                    let (ch, fg) = tile_visible(tile);
                    buf[(cell_x, cell_y)]
                        .set_char(ch)
                        .set_fg(fg)
                        .set_bg(Color::Black);
                }
            } else if map.revealed[uy][ux] {
                let tile = map.tiles[uy][ux];
                let (ch, _) = tile_visible(tile);
                buf[(cell_x, cell_y)]
                    .set_char(ch)
                    .set_fg(Color::DarkGray)
                    .set_bg(Color::Black);
            } else {
                buf[(cell_x, cell_y)].set_char(' ').set_bg(Color::Black);
            }
        }
    }
}

fn tile_visible(tile: Tile) -> (char, Color) {
    match tile {
        Tile::Wall => ('█', Color::Rgb(60, 60, 70)),
        Tile::Floor => ('·', Color::Rgb(80, 80, 80)),
        Tile::Stair => ('▼', Color::Cyan),
        Tile::Asphodel => ('✿', Color::Rgb(140, 130, 100)),
    }
}

fn draw_stats(frame: &mut Frame, area: Rect, game: &Game) {
    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            format!(" ASPHODEL — Depth {} ", game.depth),
            Style::default().fg(Color::Rgb(140, 130, 100)).bold(),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Split stats line
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20), // HP
            Constraint::Length(20), // HP bar
            Constraint::Length(15), // ATK
            Constraint::Length(15), // DEF
            Constraint::Length(15), // Obols
            Constraint::Min(0),    // rest
        ])
        .split(inner);

    let hp_text = format!(" HP: {}/{}", game.player.hp, game.player.max_hp);
    let hp_color = if game.player.hp <= 10 { Color::Red }
        else if game.player.hp <= 20 { Color::Yellow }
        else { Color::Green };

    frame.render_widget(
        Paragraph::new(hp_text).style(Style::default().fg(hp_color)),
        chunks[0],
    );

    let hp_ratio = (game.player.hp as f64 / game.player.max_hp as f64).clamp(0.0, 1.0);
    frame.render_widget(
        Gauge::default()
            .ratio(hp_ratio)
            .gauge_style(Style::default().fg(hp_color).bg(Color::DarkGray))
            .label(""),
        chunks[1],
    );

    let atk_str = if game.player.strength_turns > 0 {
        format!(" ATK: {}✦", game.player.effective_attack())
    } else {
        format!(" ATK: {}", game.player.attack)
    };
    frame.render_widget(
        Paragraph::new(atk_str).style(Style::default().fg(Color::White)),
        chunks[2],
    );

    frame.render_widget(
        Paragraph::new(format!(" DEF: {}", game.player.defense))
            .style(Style::default().fg(Color::White)),
        chunks[3],
    );

    frame.render_widget(
        Paragraph::new(format!(" Obols: {}", game.obols))
            .style(Style::default().fg(Color::Yellow)),
        chunks[4],
    );

    if game.victory {
        frame.render_widget(
            Paragraph::new(" ✦ ELYSIUM REACHED ✦")
                .style(Style::default().fg(Color::Yellow).bold()),
            chunks[5],
        );
    }
}

fn draw_log(frame: &mut Frame, area: Rect, game: &Game) {
    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(" Log ", Style::default().fg(Color::DarkGray)));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let log_lines: Vec<Line> = game.log.iter().enumerate().map(|(i, msg)| {
        let age = game.log.len() - 1 - i;
        let color = match age {
            0 => Color::White,
            1 => Color::Rgb(180, 180, 180),
            2 => Color::Rgb(140, 140, 140),
            _ => Color::Rgb(100, 100, 100),
        };
        Line::from(Span::styled(format!(" {}", msg), Style::default().fg(color)))
    }).collect();

    frame.render_widget(Paragraph::new(log_lines), inner);
}
