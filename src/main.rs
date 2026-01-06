//! TEXTFILES.COM Browser

mod browser;
mod fetcher;
mod parser;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = ui::App::new();
    let res = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        eprintln!("Error: {e}");
    }
    Ok(())
}

fn run<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut ui::App) -> Result<()> {
    app.load_home()?;

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Dismiss error
                if app.error.is_some() {
                    app.error = None;
                    continue;
                }

                // Quit
                if key.code == KeyCode::Char('q')
                    || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
                {
                    return Ok(());
                }

                match app.mode {
                    ui::Mode::Browser => match key.code {
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::PageUp => app.page_up(),
                        KeyCode::PageDown => app.page_down(),
                        KeyCode::Home | KeyCode::Char('g') => app.home(),
                        KeyCode::End | KeyCode::Char('G') => app.end(),
                        KeyCode::Enter => { app.select()?; }
                        KeyCode::Backspace | KeyCode::Left | KeyCode::Esc => { app.go_back()?; }
                        KeyCode::Char('r') => { app.refresh()?; }
                        _ => {}
                    },
                    ui::Mode::Viewer => match key.code {
                        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(1),
                        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(1),
                        KeyCode::PageUp | KeyCode::Char('b') => app.scroll_up(20),
                        KeyCode::PageDown | KeyCode::Char(' ') => app.scroll_down(20),
                        KeyCode::Home | KeyCode::Char('g') => app.scroll_home(),
                        KeyCode::End | KeyCode::Char('G') => app.scroll_end(),
                        KeyCode::Backspace | KeyCode::Left | KeyCode::Esc | KeyCode::Char('q') => { app.go_back()?; }
                        _ => {}
                    },
                }
            }
        }
        app.tick();
    }
}
