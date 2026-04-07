/*
 * Just the TUI of the tool,
 */

use ratatui::{
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    style::{Color, Style},
    layout::{Layout, Constraint, Direction},
    Frame,
};
use crate::app::App;
use crate::parser::Severity;


fn get_color(severity: &Severity) -> Style {
    match severity {
        Severity::Critical => Style::default().fg(Color::Red),
        Severity::Warning => Style::default().fg(Color::Yellow),
        Severity::Info => Style::default().fg(Color::Cyan),
    }
}
pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // ASCII
            Constraint::Length(1),  // counts
            Constraint::Min(0),     // logs
        ])
        .split(frame.area());

    let ascii = r#"
    __                    __                  __           __  
   / /   __  ______ ___  / /_  ___  _____    / /___ ______/ /__
  / /   / / / / __ `__ \/ __ \/ _ \/ ___/_  / / __ `/ ___/ //_/
 / /___/ /_/ / / / / / / /_/ /  __/ /  / /_/ / /_/ / /__/ ,<   
/_____/\__,_/_/ /_/ /_/_.___/\___/_/   \____/\__,_/\___/_/|_|  
                                                               
"#;

    let header = Paragraph::new(ascii)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, chunks[0]);

    // COUNTS BAR
    let (crit, warn, info) = app.severity_counts();

    let counts = Paragraph::new(format!(
        "🔴 {} Critical | 🟡 {} Warnings | 🔵 {} Info",
        crit, warn, info
    ))
    .style(Style::default().fg(Color::Yellow));

    frame.render_widget(counts, chunks[1]);

    // LOGS / DETECTIONS
    let items: Vec<ListItem> = if !app.detections.is_empty() {
        app.detections.iter().map(|d| {
            let count = app.counts.get(&d.name).unwrap_or(&1);

            ListItem::new(format!(
                "{} (x{})\n{}\nFix: {}",
                d.name, count, d.message, d.fix
            ))
            .style(get_color(&d.severity))
        }).collect()
    } else {
        app.raw_logs.iter().map(|line| {
            ListItem::new(line.clone())
        }).collect()
    };

    let list = List::new(items)
        .block(Block::default().title("Logs").borders(Borders::ALL));

    let mut state = ListState::default();
    state.select(Some(app.selected));

    frame.render_stateful_widget(list, chunks[2], &mut state);
}
