
mod app;
mod adb;
mod input;
mod parser;
mod rules;
mod ui;

// in android studio you run -> adb logcat -T 1 | lumberjack

use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    execute,
    event::{self, Event, KeyCode}
}; 

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::ListState,
};

use app::App;
use parser::parse_line;
use rules::default_rules;

use std::io;
use std::env;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] != "start" {
        println!("Usage: lumberjack start");
        return Ok(());
    }

    run()
}




fn run() -> Result<(), io::Error> {
    let rules = default_rules();
    let mut app = App::new();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    for line in adb::spawn_logcat() {

        // keyboard input
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        // parse logs
        if let Some(detection) = parse_line(&line, &rules) {
            app.add_detection(detection);
        }

        // draw UI
        terminal.draw(|f| ui::draw(f, &app))?;
    }

    disable_raw_mode()?;
    Ok(())
}
