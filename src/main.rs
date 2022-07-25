use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, 
        EnableMouseCapture
    },
    execute,
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    },
};

mod app;
mod ui;

fn main() -> Result<(), io::Error> {
    // initialise terminal
    enable_raw_mode()?; 
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?; 
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // initialise app info
    let app = app::App::new();
    
    // main app loop
    ui::run_app(&mut terminal, app)?;

    disable_raw_mode()?;	
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,	
        DisableMouseCapture		
    )?;
    terminal.show_cursor()?;

    Ok(())
}