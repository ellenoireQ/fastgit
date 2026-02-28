// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Fitrian Musya

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{app::App, ui::draw_ui};
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

        app.check_push_result();

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if app.show_commit_dialog {
                    match key.code {
                        KeyCode::Esc => app.close_commit_dialog(),
                        KeyCode::Tab => app.toggle_commit_focus(),
                        KeyCode::Enter => {
                            if !app.commit_summary.is_empty() {
                                match app.commit() {
                                    Ok(_oid) => {
                                        app.scan_git();
                                        app.file_statuses.clear();
                                        app.tree = crate::file_tree::FileTree::new(
                                            std::path::PathBuf::from("."),
                                        );
                                        app.diff_content.clear();
                                        app.selected_file = None;

                                        if app.has_git {
                                            if let Ok(repo) = git2::Repository::open(&app.cur_dir) {
                                                let mut options = git2::StatusOptions::new();
                                                options.include_untracked(true);

                                                if let Ok(statuses) =
                                                    repo.statuses(Some(&mut options))
                                                {
                                                    let mut paths: Vec<std::path::PathBuf> =
                                                        Vec::new();

                                                    for entry in statuses.iter() {
                                                        if entry
                                                            .status()
                                                            .contains(git2::Status::IGNORED)
                                                        {
                                                            continue;
                                                        }
                                                        if let Some(p) = entry.path() {
                                                            let path = std::path::PathBuf::from(p);
                                                            app.file_statuses.insert(
                                                                path.clone(),
                                                                entry.status(),
                                                            );
                                                            paths.push(path);
                                                        }
                                                    }

                                                    app.tree.populate_from_paths(paths);
                                                }
                                            }
                                        }

                                        app.commit_success_open = true;
                                    }
                                    Err(_e) => {
                                        // TODO: Show error dialog
                                    }
                                }
                            }
                            app.close_commit_dialog();
                        }
                        KeyCode::Char(c) => app.commit_message_insert(c),
                        KeyCode::Backspace => app.commit_message_backspace(),
                        KeyCode::Delete => app.commit_message_delete(),
                        KeyCode::Left => app.commit_cursor_left(),
                        KeyCode::Right => app.commit_cursor_right(),
                        KeyCode::Home => app.commit_cursor_home(),
                        KeyCode::End => app.commit_cursor_end(),
                        _ => {}
                    }
                } else if app.commit_success_open {
                    app.commit_success_open = false;
                } else if app.push_success_open {
                    app.push_success_open = false;
                } else if app.push_error.is_some() {
                    app.push_error = None;
                } else if app.commit_warning_open {
                    if let KeyCode::Char('q') = key.code {
                        app.commit_warning_open = false;
                    }
                } else if app.checkout_success.is_some() {
                    app.checkout_success = None;
                } else if app.checkout_error.is_some() {
                    app.checkout_error = None;
                } else if app.show_help {
                    app.show_help = false;
                } else if app.show_new_branch_dialog {
                    match key.code {
                        KeyCode::Esc => app.close_new_branch_dialog(),
                        KeyCode::Enter => app.confirm_new_branch(),
                        KeyCode::Char(c) => app.new_branch_name.push(c),
                        KeyCode::Backspace => {
                            app.new_branch_name.pop();
                        }
                        _ => {}
                    }
                } else if app.show_add_remote_dialog {
                    match key.code {
                        KeyCode::Esc => app.close_add_remote_dialog(),
                        KeyCode::Tab => app.add_remote_focus_url = !app.add_remote_focus_url,
                        KeyCode::Enter => {
                            let _ = app.confirm_add_remote();
                        }
                        KeyCode::Char(c) => app.add_remote_input_push(c),
                        KeyCode::Backspace => app.add_remote_input_pop(),
                        _ => {}
                    }
                } else if app.branch_focused {
                    match key.code {
                        KeyCode::Up => {
                            if app.branch_tab == crate::app::BranchTab::Local {
                                app.branch_previous();
                            } else {
                                app.remote_previous();
                            }
                        }
                        KeyCode::Down => {
                            if app.branch_tab == crate::app::BranchTab::Local {
                                app.branch_next();
                            } else {
                                app.remote_next();
                            }
                        }
                        KeyCode::Left | KeyCode::Right => app.branch_tab_toggle(),
                        KeyCode::Enter => {
                            if app.branch_tab == crate::app::BranchTab::Local {
                                app.checkout_selected_branch();
                            }
                        }
                        KeyCode::Char('n') => {
                            if app.branch_tab == crate::app::BranchTab::Local {
                                app.open_new_branch_dialog();
                            }
                        }
                        KeyCode::Char('a') => {
                            if app.branch_tab == crate::app::BranchTab::Remote {
                                app.open_add_remote_dialog();
                            }
                        }
                        KeyCode::Char('d') => {
                            if app.branch_tab == crate::app::BranchTab::Remote {
                                let _ = app.remove_selected_remote();
                            }
                        }
                        KeyCode::Tab => {
                            app.branch_focused = false;
                            app.increase_window();
                        }
                        KeyCode::Char('?') => app.show_help = true,
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                } else {
                    // INFO: Main Terminal Key logic
                    match key.code {
                        KeyCode::Tab => app.increase_window(),
                        KeyCode::Char('s') => app.scan_git(),
                        KeyCode::Char('c') => {
                            if app.staged_count == 0 {
                                app.commit_warning_open = true;
                            } else {
                                app.open_commit_dialog()
                            }
                        }
                        KeyCode::Char('P') => {
                            app.start_push();
                        }
                        KeyCode::Enter => {
                            app.select_file();
                            app.focused = true
                        }
                        KeyCode::Up => {
                            if app.window_index == 1 {
                                app.commit_graph_previous();
                            } else if app.focused {
                                app.diff_scroll_up();
                            } else {
                                app.tree.previous();
                                app.select_file();
                            }
                        }
                        KeyCode::Down => {
                            if app.window_index == 1 {
                                app.commit_graph_next();
                            } else if app.focused {
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
                        KeyCode::Right => {
                            let selected_is_file = app
                                .tree
                                .state
                                .selected()
                                .and_then(|i| app.tree.items.get(i))
                                .map(|(_, _, is_dir)| !is_dir)
                                .unwrap_or(false);

                            if selected_is_file {
                                app.select_file();
                                app.focused = true;
                            } else {
                                app.tree.toggle_expand();
                            }
                        }
                        KeyCode::Char('u') => app.window_index += 1,

                        KeyCode::Char('?') => app.show_help = true,
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
