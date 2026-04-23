
mod app;
mod adb;
mod input;
mod parser;
mod rules;
mod ui;
mod splash;
mod filter;

use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
    event::{self, Event, KeyCode, KeyModifiers, KeyEventKind},
};
use ratatui::{Terminal, backend::CrosstermBackend};
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
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let rules = default_rules();
    let mut app = App::new();

    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        for line in adb::spawn_logcat() {
            if tx.send(line).is_err() {
                break;
            }
        }
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| splash::splash(f))?;
    thread::sleep(Duration::from_millis(1500));

    'main: loop {
    
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break 'main,
                        KeyCode::Char('q') => break 'main,
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break 'main,
                        KeyCode::Enter => app.toggle_expand(),
                        KeyCode::Down  => app.next(),
                        KeyCode::Up    => app.previous(),
                        _ => {}
                    }
                }
            }
        }

        // Drain up to 50 log lines per frame
        for _ in 0..50 {
            match rx.try_recv() {
                Ok(line) => {
                    //if !filter::should_keep(&line) { continue; }
                    //println!("LOG: {}", line);
                    if let Some(detection) = parse_line(&line, &rules) {
                        app.add_detection(detection);
                    } else {
                        app.add_raw(line);
                    }
                }
                Err(_) => break,
            }
        }

        terminal.draw(|f| ui::draw(f, &app))?;
    }

    // Always restore terminal, even on q
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
