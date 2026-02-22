use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::{App, Tab},
    ui::draw_ui,
};
mod app;
mod file_tree;
mod helper;
mod ui;

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Tab => app.next_tab(),
                KeyCode::BackTab => app.prev_tab(),
                KeyCode::Char('s') => app.scan_git(),
                KeyCode::Enter => {
                    app.select_file();
                    app.focused = true
                }
                KeyCode::Up => {
                    if app.focused {
                        app.diff_scroll_up();
                    } else {
                        app.tree.previous();
                    }
                }
                KeyCode::Down => {
                    if app.focused {
                        app.diff_scroll_down();
                    } else {
                        app.tree.next();
                    }
                }
                KeyCode::Left => app.tree.collapse_or_parent(),
                KeyCode::Right => app.tree.toggle_expand(),
                KeyCode::Char('q') => break,
                _ => {}
            },
            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
