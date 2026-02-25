// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Fitrian Musya

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
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if app.show_commit_dialog {
                    match key.code {
                        KeyCode::Esc => app.close_commit_dialog(),
                        KeyCode::Tab => app.toggle_commit_focus(),
                        KeyCode::Enter => {
                            // TODO: implement commit logic
                            app.close_commit_dialog();
                        }
                        KeyCode::Char(c) => app.commit_message_push(c),
                        KeyCode::Backspace => app.commit_message_pop(),
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Tab => app.increase_window(),
                        KeyCode::BackTab => app.prev_tab(),
                        KeyCode::Char('s') => app.scan_git(),
                        KeyCode::Char('c') => app.open_commit_dialog(),
                        KeyCode::Enter => {
                            app.select_file();
                            app.focused = true
                        }
                        KeyCode::Up => {
                            if app.focused {
                                app.diff_scroll_up();
                            } else {
                                app.tree.previous();
                                app.select_file();
                            }
                        }
                        KeyCode::Down => {
                            if app.focused {
                                app.diff_scroll_down();
                            } else {
                                app.tree.next();
                                app.select_file();
                            }
                        }
                        KeyCode::Esc => {
                            if app.focused {
                                app.focused = false;
                            }
                        }
                        KeyCode::Char(' ') => {
                            if let Some(path) = &app.selected_file
                                && let Err(err) = app.toggle_stage(&path.clone())
                            {
                                eprintln!("{}", err);
                            };
                        }
                        KeyCode::Left => app.tree.collapse_or_parent(),
                        KeyCode::Right => app.tree.toggle_expand(),
                        // For testing purpose
                        KeyCode::Char('u') => app.window_index += 1,

                        KeyCode::Char('q') => break,

                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
