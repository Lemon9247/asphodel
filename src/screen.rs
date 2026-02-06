use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

const TITLE_ART: &str = r#"

              ░█▀█░█▀▀░█▀█░█░█░█▀█░█▀▄░█▀▀░█░░
              ░█▀█░▀▀█░█▀▀░█▀█░█░█░█░█░█▀▀░█░░
              ░▀░▀░▀▀▀░▀░░░▀░▀░▀▀▀░▀▀░░▀▀▀░▀▀▀


                     ✿  ·  ✿  ·  ✿

          You are a shade in the Asphodel Meadows.
        The grey afterlife, where ordinary souls drift.

        You were nobody special in life.
        You won't be special in death either.

                     But you can try.


                 ✿  Press any key  ✿
"#;

const DEATH_ART: &str = r#"

                    ·  ·  ·  ✿  ·  ·  ·


                    You have dissolved.

               Another shade, lost to the meadows.
              The flowers grow a little taller here.


"#;

const VICTORY_ART: &str = r#"

                    ☀  ☀  ☀  ☀  ☀  ☀  ☀


                    You reached Elysium.

            The river Lethe gleams below.
            You could drink and forget everything.

                 Instead, you remember.
                 You choose to remember.


"#;

pub fn title_widget(_area: Rect) -> Paragraph<'static> {
    let lines: Vec<Line> = TITLE_ART
        .lines()
        .map(|l| {
            Line::from(Span::styled(
                l.to_string(),
                Style::default().fg(Color::Rgb(140, 130, 100)),
            ))
        })
        .collect();

    Paragraph::new(lines).alignment(Alignment::Left)
}

pub fn death_widget(depth: u32, obols: u32, turns: u32, killed_by: &str) -> Paragraph<'static> {
    let mut lines: Vec<Line> = DEATH_ART
        .lines()
        .map(|l| {
            Line::from(Span::styled(
                l.to_string(),
                Style::default().fg(Color::Rgb(120, 80, 80)),
            ))
        })
        .collect();

    let stats_color = Color::Rgb(100, 100, 100);
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("              Depth: {}  |  Obols: {}  |  Turns: {}", depth, obols, turns),
        Style::default().fg(stats_color),
    )));
    lines.push(Line::from(Span::styled(
        format!("              Killed by: {}", killed_by),
        Style::default().fg(stats_color),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "                     [Press any key to quit]",
        Style::default().fg(Color::DarkGray),
    )));

    Paragraph::new(lines).alignment(Alignment::Left)
}

pub fn victory_widget(obols: u32, turns: u32) -> Paragraph<'static> {
    let mut lines: Vec<Line> = VICTORY_ART
        .lines()
        .map(|l| {
            Line::from(Span::styled(
                l.to_string(),
                Style::default().fg(Color::Rgb(200, 180, 100)),
            ))
        })
        .collect();

    let stats_color = Color::Rgb(160, 150, 100);
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("              Obols: {}  |  Turns: {}", obols, turns),
        Style::default().fg(stats_color),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "                 ☀  You were nobody special.  ☀",
        Style::default().fg(Color::Rgb(200, 180, 100)).bold(),
    )));
    lines.push(Line::from(Span::styled(
        "                 ☀  You are nobody special.   ☀",
        Style::default().fg(Color::Rgb(200, 180, 100)).bold(),
    )));
    lines.push(Line::from(Span::styled(
        "                 ☀  But you made it anyway.   ☀",
        Style::default().fg(Color::Rgb(200, 180, 100)).bold(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "                     [Press any key to quit]",
        Style::default().fg(Color::DarkGray),
    )));

    Paragraph::new(lines).alignment(Alignment::Left)
}
