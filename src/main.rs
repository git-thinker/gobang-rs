#![allow(unused_imports)]
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod App;
fn main() -> Result<(), io::Error> {
    // 配置 Terminal
    enable_raw_mode()?; // 启动命令行的 raw 模式
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?; // 在一个新的界面上运行 UI，保存原终端内容，并开启鼠标捕获
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // 初始化 app 资源
    let app = App::App::new();
 	// 程序主要逻辑循环 …… //
    App::run_app(&mut terminal, app)?;
    // 恢复 Terminal
    disable_raw_mode()?;	// 禁用 raw 模式
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,	// 恢复到原来的命令行窗口
        DisableMouseCapture		// 禁用鼠标捕获
    )?;
    terminal.show_cursor()?; // 显示光标

    Ok(())
}