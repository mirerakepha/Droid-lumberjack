/*
 * Just the TUI of the tool,
 */

use ratatui::{
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    layout::{Layout, Constraint, Direction},
    Frame,
};
use crate::app::App;
use crate::parser::Severity;

fn severity_color(s: &Severity) -> Color {
    match s {
        Severity::Critical => Color::Red,
        Severity::Warning  => Color::Yellow,
        Severity::Info     => Color::Cyan,
    }
}

fn severity_icon(s: &Severity) -> &'static str {
    match s {
        Severity::Critical => "✖ CRIT",
        Severity::Warning  => "⚠ WARN",
        Severity::Info     => "● INFO",
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    // Outer layout: header | body | footer
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header bar
            Constraint::Min(0),     // body
            Constraint::Length(1),  // keybind bar
        ])
        .split(frame.area());

    // Header 
    let (crit, warn, info) = app.severity_counts();
    let header_line = Line::from(vec![
        Span::styled(" ⬡ LUMBERJACK ", Style::default()
            .fg(Color::Black).bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled("✖ ", Style::default().fg(Color::Red)),
        Span::styled(format!("{crit} critical"), Style::default().fg(Color::Red)),
        Span::raw("   "),
        Span::styled("⚠ ", Style::default().fg(Color::Yellow)),
        Span::styled(format!("{warn} warning"), Style::default().fg(Color::Yellow)),
        Span::raw("   "),
        Span::styled("● ", Style::default().fg(Color::Cyan)),
        Span::styled(format!("{info} info"), Style::default().fg(Color::Cyan)),
    ]);
    frame.render_widget(Paragraph::new(header_line)
        .block(Block::default().borders(Borders::BOTTOM)), outer[0]);

    // Body: 3 columns
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35), // detection list
            Constraint::Percentage(40), // detail panel
            Constraint::Percentage(25), // raw logs
        ])
        .split(outer[1]);

    // Left: detection list
    let list_items: Vec<ListItem> = app.detections.iter().map(|d| {
        let count = app.counts.get(&d.name).cloned().unwrap_or(1);
        let icon  = severity_icon(&d.severity);
        let color = severity_color(&d.severity);
        ListItem::new(Line::from(vec![
            Span::styled(format!(" {icon} "), Style::default().fg(color)),
            Span::styled(d.name.clone(), Style::default()
                .fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" ×{count}"), Style::default().fg(Color::DarkGray)),
        ]))
    }).collect();

    let detection_list = List::new(list_items)
        .block(Block::default()
            .title(" Detections ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)))
        .highlight_style(Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected));
    frame.render_stateful_widget(detection_list, cols[0], &mut list_state);

    // Middle: detail panel for selected detection
    let detail_text = if let Some(d) = app.detections.get(app.selected) {
        let color = severity_color(&d.severity);
        vec![
            Line::from(Span::styled(
                format!(" {} ", severity_icon(&d.severity)),
                Style::default().fg(color).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("Rule:    ", Style::default().fg(Color::DarkGray)),
                Span::styled(d.name.clone(), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Issue:   ", Style::default().fg(Color::DarkGray)),
                Span::styled(d.message.clone(), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(Span::styled("Suggested fix:", Style::default().fg(Color::DarkGray))),
            Line::from(Span::styled(d.fix.clone(),
                Style::default().fg(Color::Green))),
        ]
    } else {
        vec![Line::from(Span::styled(
            " No detections yet — watching…",
            Style::default().fg(Color::DarkGray)))]
    };

    frame.render_widget(
        Paragraph::new(detail_text)
            .wrap(Wrap { trim: false })
            .block(Block::default()
                .title(" Detail ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))),
        cols[1]);

    // Right: raw log tail
    let raw_text: Vec<Line> = app.raw_logs.iter().map(|line| {
        Line::from(Span::styled(
                line.clone(),
                Style::default().fg(Color::DarkGray),
        ))
    }).collect();

    frame.render_widget(
        Paragraph::new(raw_text)
            .wrap(Wrap {trim: false})
            .block(Block::default()
                .title(" Raw logs ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))),
        cols[2]);

    // Footer keybind bar
    let footer = Line::from(vec![
        Span::styled(" ↑↓ ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" navigate   "),
        Span::styled(" Enter ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" expand   "),
        Span::styled(" q ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" quit "),
    ]);
    frame.render_widget(Paragraph::new(footer), outer[2]);
}
