// main app loop and ui drawing

use std::{io, time::Duration};
use tui::{
    backend::Backend,
    widgets::{
        Block, 
        Borders, 
        Paragraph
    },
    layout::{
        Layout, 
        Constraint, 
        Direction, 
        Alignment
    },
    Terminal,
    Frame, 
    style::{
        Style, 
        Color, 
        Modifier
    },
};
use crossterm::{
    event::{self, Event, KeyCode},
};
use crate::app::{
    App, 
    Status
};


// ui drawing
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // split heading and vertical layout
    let chunks = Layout::default() 
        .margin(1)
        .constraints({
            let mut v = vec![Constraint::Percentage(12)];
            v.extend(vec![Constraint::Length(3); app.size+1]);
            v
        }.as_ref()) 
        .direction(Direction::Vertical) 
        .split(f.size());
    // split horizontal layout in table 
    let mut chunkss = Vec::new();
    for i in 1..app.size+1{
        chunkss.push(Layout::default()
            .constraints(vec![Constraint::Length(3); app.size+1].as_ref())
            .direction(Direction::Horizontal)
            .split(chunks[i])
        );
    }
    // draw cells in accordance with app.matrix
    for i in 0..app.size{
        for j in 0..app.size{
            let paragraph = Paragraph::new(
                    match app.matrix[i][j] {
                        Status::X => "X",
                        Status::O => "O",
                        Status::Null => " ",
                    }
                )
                .style({
                        let mut style = Style::default();
                        if app.row == i && app.column == j {
                            style = style.fg(Color::Red).add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK | Modifier::REVERSED);
                        }else{
                            style = style.fg(Color::Black);
                        }
                        match app.matrix[i][j] {
                            Status::X => style = style.bg(Color::Green),
                            Status::O => style = style.bg(Color::LightYellow),
                            Status::Null => style = style.bg(Color::DarkGray),
                        }
                        style
                    }
                )
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);
            f.render_widget(paragraph, chunkss[i][j]);
        }
    }
    // add heading to heading area
    let paragraph = Paragraph::new(
        match app.winner{
            Status::Null => format!("Gobang IN PROGRESS----{}'s Turn\nQ to quit | R to restart", app.now_player).to_string(),
            Status::X => "X is the winner ~\nQ to quit | R to restar".to_string(),
            Status::O => "O is the winner ~\nQ to quit | R to restar".to_string(),
        }   
        )
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);
}

// app main loop
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        // check if game is over
        app.check();
        // render ui
        terminal.draw(|f| ui(f, &mut app))?;
        // key press event response
        if crossterm::event::poll(Duration::from_micros(1))? { 
            // check if there is a event with timeout = 1ms
            if let Event::Key(key) = event::read()? {
                // use read straight away may block the main thread
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Char('r') | KeyCode::Char('R')=> app = App::new(),
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => app.left(),
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => app.right(),
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => app.up(),
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => app.down(),
                    KeyCode::Enter => app.register(),
                    _ => {},
                }
            }
        }
        // 处理其他逻辑
    }
    Ok(())
}