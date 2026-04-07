use ratatui::{
    widgets::{Paragraph, Block, Borders},
    style::{Style, Color},
    layout::{Alignment},
    Frame,
};

pub fn splash(frame: &mut Frame) {
    let ascii = r#"
    __                    __                  __           __  
   / /   __  ______ ___  / /_  ___  _____    / /___ ______/ /__
  / /   / / / / __ `__ \/ __ \/ _ \/ ___/_  / / __ `/ ___/ //_/
 / /___/ /_/ / / / / / / /_/ /  __/ /  / /_/ / /_/ / /__/ ,<   
/_____/\__,_/_/ /_/ /_/_.___/\___/_/   \____/\__,_/\___/_/|_|  
                                                               
"#;

    let paragraph = Paragraph::new(ascii)
        .style(
            Style::default().fg(Color::Blue) // you can mix later
        )
        .alignment(Alignment::Center)
        .block(Block::default().title("Lumberjack").borders(Borders::ALL));

    frame.render_widget(paragraph, frame.area());
}

