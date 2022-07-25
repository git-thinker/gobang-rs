use std::{io, time::Duration};

use crossterm::{event::{self, Event, KeyCode}};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal, text::Span, style::{Color, Modifier, Style},
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Status {
    X,
    O,
    Null,
}
pub struct App {
    pub url: String, // 存放一些数据或者 UI 状态
    pub row: u8,
    pub column: u8,
    pub matrix: [[Status; 3]; 3],
    pub winner: Status
}

impl App {
    fn up(&mut self){
        match self.row {
            0 => self.row = 2,
            _ => self.row -= 1,
        };
    }
    
    fn down(&mut self){
        match self.row {
            2 => self.row = 0,
            _ => self.row += 1,
        };
    }

    fn left(&mut self){
        match self.column {
            0 => self.column = 2,
            _ => self.column -= 1,
        };
    }

    fn right(&mut self){
        match self.column {
            2 => self.column = 0,
            _ => self.column += 1,
        };
    }
    
    fn x(&mut self){
        self.matrix[self.row as usize][self.column as usize] = Status::X;
    }

    fn o(&mut self){
        self.matrix[self.row as usize][self.column as usize] = Status::O;
    }

    fn check(&mut self){
        for i in 0..3{
            if self.matrix[i][0] == self.matrix[i][1] && self.matrix[i][1] == self.matrix[i][2]{
                self.winner = self.matrix[i][0];
                return ;
            }
            if self.matrix[0][i] == self.matrix[1][i] && self.matrix[1][i] == self.matrix[2][i]{
                self.winner = self.matrix[0][i];
                return ;
            }
        }
        if self.matrix[0][0] == self.matrix[1][1] && self.matrix[1][1] == self.matrix[2][2]{
            self.winner = self.matrix[0][0];
            return ;
        }
        if self.matrix[0][2] == self.matrix[1][1] && self.matrix[1][1] == self.matrix[2][0]{
            self.winner = self.matrix[1][1];
            return ;
        }
    }
    pub fn new() -> App{
        App {
            url: String::from(r"https://hellogithub.com/"),
            row: 0,
            column: 0,
            matrix: [[Status::Null;3];3],
            winner: Status::Null,
        }
    }
}
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    //
    let chunks = Layout::default() // 首先获取默认构造
        .margin(1)
        .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(30)
            ].as_ref()) // 按照 3 行 和 最小 3 行的规则分割区域
        .direction(Direction::Vertical) // 垂直分割
        .split(f.size()); // 分割整块 Terminal 区域
    let mut chunkss = Vec::new();
    for i in 1..4{
        chunkss.push(Layout::default()
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33)
                ].as_ref())
            .direction(Direction::Horizontal)
            .split(chunks[i])
        ); // 分割整块 Terminal 区域
    }

    for i in 0..3{
        for j in 0..3{
            let paragraph = Paragraph::new(
                    match app.matrix[i][j] {
                        Status::X => "X",
                        Status::O => "O",
                        Status::Null => " ",
                    }
                )
                .style({
                        let mut style = Style::default();
                        if app.row == i as u8 && app.column == j as u8 {
                            style = style.fg(Color::Red).add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK);
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
                .block(Block::default().borders(Borders::ALL).title(format!("{}-{}", i, j)))
                .alignment(Alignment::Center);
            f.render_widget(paragraph, chunkss[i][j]);
        }
    }

    let paragraph = Paragraph::new(
        match app.winner{
            Status::Null => "# CHESS IN PROGRESS",
            Status::X => "X is the winner ~",
            Status::O => "O is the winner ~",
        }   
        )
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);

}
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        app.check();
        // 渲染 UI
        terminal.draw(|f| ui(f, &mut app))?;
        // 处理按键事件
        if crossterm::event::poll(Duration::from_micros(1))? { // poll 方法非阻塞轮询
            if let Event::Key(key) = event::read()? { // 直接 read 如果没有事件到来则会阻塞等待
                match key.code { // 判断用户按键
                    KeyCode::Char('q') => {
                        break;
                    },
                    KeyCode::Char('r') => app = App::new(),
                    KeyCode::Left => app.left(),
                    KeyCode::Right => app.right(),
                    KeyCode::Up => app.up(),
                    KeyCode::Down => app.down(),
                    KeyCode::Enter => app.x(),
                    KeyCode::Char(' ') => app.o(),
                    _ => {},
                }
            }
        }
        // 处理其他逻辑
    }
    Ok(())
}