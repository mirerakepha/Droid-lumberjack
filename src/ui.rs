/*
 * Just the TUI of the tool,
 */

use ratatui::{
    widgets::{Block, Borders, List, ListItem},
    Style::{Color, Style},
    Frame,
};
use crate app::App;
use crate parser::Severity;


fn get_color(severity: &Severity) -> Style {
    match severity {
        Severity::Critical => Style::default().fg(Color::Red),
        Severity::Warning => Style::default().fg(Color::Yellow),
        Severity::Info => Style::default().fg(Color::Cyan),
    }
}

pub fn draw(frame: &mut Frame, app: &App) {

    let (crit, warn, info) = app.severity_counts();

    let title = format!(
        "🔴 {} Critical | 🟡 {} Warnings | 🔵 {} Info",
        crit, warn, info
    );

    let items: Vec<ListItem> = app
        .detections
        .iter()
        .map(|d| {
            ListItems::new(format!(
                    "{} (x{})\n{}\nFix: {}",
                    d.name, count, d.message, d.fix
            ))
            .style(get_color(&d.severity))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("detections").borders(Borders::ALL));

    let mut state = ListState::default();
    state.select(Some(app.selected));

    frame.render_stateful_widget(list, frame.size(), &mut state);


}
