// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Fitrian Musya

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::{
    app::{App, BranchTab, DiffLineKind, Tab},
    helper::helpers::{Dialog, DialogType, Helper},
};

const BORDER_STYLE: Style = Style::new().yellow().bold();
const BORDER_DEFAULT_STYLE: Style = Style::new().white().bold();

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    app.refresh_current_branch();

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    draw_content(f, vertical_chunks[0], app);

    if app.show_commit_dialog {
        draw_commit_dialog(f, app);
    }
    if app.commit_warning_open {
        let h = Helper;
        h.draw_dialog(
            f, // Frame
            Dialog {
                dialog_type: DialogType::Warning,
                title: "Commit Failed".to_string(),
                content: vec![
                    Line::from("No staged files found"),
                    Line::from("Please stage files first"),
                ],
                width: 60,
                height: 10,
            },
        );
    }
    if app.commit_success_open {
        let h = Helper;
        h.draw_dialog(
            f,
            Dialog {
                dialog_type: DialogType::Success,
                title: "Commit Successful".to_string(),
                content: vec![
                    Line::from("Your changes have been committed"),
                    Line::from(""),
                    Line::from("Press any key to continue"),
                ],
                width: 60,
                height: 10,
            },
        );
    }
    if app.push_in_progress {
        let h = Helper;
        h.draw_dialog(
            f,
            Dialog {
                dialog_type: DialogType::Success,
                title: "Pushing...".to_string(),
                content: vec![Line::from("Pushing to remote, please wait...")],
                width: 60,
                height: 8,
            },
        );
    }
    if app.push_success_open {
        let h = Helper;
        h.draw_dialog(
            f,
            Dialog {
                dialog_type: DialogType::Success,
                title: "Push Successful".to_string(),
                content: vec![
                    Line::from("Your changes have been pushed to remote"),
                    Line::from(""),
                    Line::from("Press any key to continue"),
                ],
                width: 60,
                height: 10,
            },
        );
    }
    if let Some(err_msg) = &app.push_error {
        let h = Helper;
        h.draw_dialog(
            f,
            Dialog {
                dialog_type: DialogType::Warning,
                title: "Push Failed".to_string(),
                content: vec![
                    Line::from(err_msg.clone()),
                    Line::from(""),
                    Line::from("Press any key to continue"),
                ],
                width: 70,
                height: 10,
            },
        );
    }
    app.branch_focused = app.window_index == 2;
    draw_footer(vertical_chunks[1], app, f);

    if app.show_add_remote_dialog {
        draw_add_remote_dialog(f, app);
    }
}

fn draw_content(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    match app.current_tab {
        Tab::Tree => {
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            let top_cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(45),
                    Constraint::Percentage(35),
                    Constraint::Percentage(20),
                ])
                .split(rows[0]);

            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(rows[1]);

            let h = Helper;

            let items: Vec<ListItem> = app
                .tree
                .items
                .iter()
                .map(|(path, depth, is_dir)| {
                    let indent = "  ".repeat(*depth);
                    let name = path.file_name().unwrap_or_default().to_string_lossy();

                    if *is_dir {
                        let text = format!("{}{}/", indent, name);
                        ListItem::new(text).style(
                            Style::default()
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        let lookup_path = path.strip_prefix(".").unwrap_or(path);
                        let (icon, color) = if let Some(status) = app.file_statuses.get(lookup_path)
                        {
                            (h.get_txt_icon(*status), h.get_status_color(*status))
                        } else {
                            ("??", Color::White)
                        };
                        let text = format!("{}{} {}", indent, icon, name);
                        ListItem::new(text).style(Style::default().fg(color))
                    }
                })
                .collect();

            if items.is_empty() {
                let empty = Paragraph::new("Working tree is clean")
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title("Tree"),
                    )
                    .style(if app.window_index == 0 {
                        BORDER_STYLE
                    } else {
                        BORDER_DEFAULT_STYLE
                    });

                f.render_widget(empty, top_cols[0]);
            } else {
                let list = ratatui::widgets::List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title("Tree"),
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("▶ ")
                    .style(if app.window_index == 0 {
                        BORDER_STYLE
                    } else {
                        BORDER_DEFAULT_STYLE
                    });

                f.render_stateful_widget(list, top_cols[0], &mut app.tree.state);
            }

            draw_commit_graph_panel(f, top_cols[1], app);

            let branch_outer_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(if app.window_index == 2 {
                    BORDER_STYLE
                } else {
                    BORDER_DEFAULT_STYLE
                });

            let branch_inner_area = branch_outer_block.inner(top_cols[2]);
            f.render_widget(branch_outer_block, top_cols[2]);

            let branch_inner_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(2), Constraint::Min(0)])
                .split(branch_inner_area);

            let local_style = if app.branch_tab == BranchTab::Local {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let remote_style = if app.branch_tab == BranchTab::Remote {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let tab_line = Line::from(vec![
                Span::styled(" Local ", local_style),
                Span::raw(" "),
                Span::styled(" Remote ", remote_style),
            ]);
            let separator = Line::from("─".repeat(branch_inner_area.width as usize));
            let tab_para = Paragraph::new(vec![tab_line, separator]);
            f.render_widget(tab_para, branch_inner_chunks[0]);

            match app.branch_tab {
                BranchTab::Local => {
                    let branches: Vec<ListItem> = app
                        .branches
                        .iter()
                        .map(|b| ListItem::new(b.as_str()))
                        .collect();

                    let branch_list = ratatui::widgets::List::new(branches)
                        .highlight_style(
                            Style::default()
                                .bg(Color::DarkGray)
                                .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol("▶ ");

                    f.render_stateful_widget(
                        branch_list,
                        branch_inner_chunks[1],
                        &mut app.branch_state,
                    );
                }
                BranchTab::Remote => {
                    let remote_items: Vec<ListItem> = app
                        .remotes
                        .iter()
                        .map(|(name, url)| {
                            ListItem::new(Line::from(vec![
                                Span::styled(
                                    format!("{} ", name),
                                    Style::default()
                                        .fg(Color::Cyan)
                                        .add_modifier(Modifier::BOLD),
                                ),
                                Span::styled(url.clone(), Style::default().fg(Color::Gray)),
                            ]))
                        })
                        .collect();

                    if remote_items.is_empty() {
                        let empty = Paragraph::new("No remotes  |a| Add  |d| Delete")
                            .style(Style::default().fg(Color::DarkGray));
                        f.render_widget(empty, branch_inner_chunks[1]);
                    } else {
                        let remote_list = ratatui::widgets::List::new(remote_items)
                            .highlight_style(
                                Style::default()
                                    .bg(Color::DarkGray)
                                    .add_modifier(Modifier::BOLD),
                            )
                            .highlight_symbol("▶ ");

                        f.render_stateful_widget(
                            remote_list,
                            branch_inner_chunks[1],
                            &mut app.remote_state,
                        );
                    }
                }
            }

            let diff_title = match &app.selected_file {
                Some(p) => format!("Diff — {}", p.display()),
                None => "Diff — No file selected".to_string(),
            };

            if app.diff_content.is_empty() {
                let msg = if app.selected_file.is_some() {
                    "No changes detected for this file"
                } else {
                    "Select a file and press Enter"
                };
                let empty = Paragraph::new(msg)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title(diff_title),
                    )
                    .style(if app.window_index == 3 {
                        BORDER_STYLE
                    } else {
                        BORDER_DEFAULT_STYLE
                    });
                f.render_widget(empty, bottom_chunks[0]);
                draw_recipe(f, bottom_chunks[1]);
            } else {
                let visible_lines: Vec<ListItem> = app
                    .diff_content
                    .iter()
                    .skip(app.diff_scroll)
                    .map(|dl| {
                        let color = match dl.kind {
                            DiffLineKind::Add => Color::Green,
                            DiffLineKind::Delete => Color::Red,
                            DiffLineKind::Header => Color::Yellow,
                            DiffLineKind::Context => Color::White,
                        };
                        let prefix = match dl.kind {
                            DiffLineKind::Add => "+ ",
                            DiffLineKind::Delete => "- ",
                            DiffLineKind::Header => "",
                            DiffLineKind::Context => "  ",
                        };
                        ListItem::new(Line::from(Span::styled(
                            format!("{}{}", prefix, dl.content),
                            Style::default().fg(color),
                        )))
                    })
                    .collect();

                let diff_list = List::new(visible_lines)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title(diff_title),
                    )
                    .style(if app.window_index == 3 {
                        BORDER_STYLE
                    } else {
                        BORDER_DEFAULT_STYLE
                    });

                f.render_widget(diff_list, bottom_chunks[0]);
                draw_recipe(f, bottom_chunks[1]);
            }
        }
    }
}

fn draw_recipe(f: &mut Frame, area: ratatui::layout::Rect) {
    let content = Paragraph::new(vec![
        Line::from("🕮  Book").style(Style::default().cyan()),
        Line::from("A brief guide to using this application."),
        Line::from(""),
        Line::from("|⇥| Switching window"),
        Line::from("|⏎| Select"),
        Line::from("|c| Commit"),
        Line::from("|P| Pushing"),
        Line::from("|a| Add remote"),
        Line::from("|d| Del remote"),
        Line::from("|q| Quit"),
    ])
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .add_modifier(Modifier::BOLD);

    f.render_widget(content, area);
}

fn draw_commit_graph_panel(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let graph_lines: Vec<Line> = if app.commit_graph.is_empty() {
        vec![Line::from("No commits found")]
    } else {
        app.commit_graph
            .iter()
            .skip(app.commit_graph_scroll)
            .map(|line| Line::from(line.as_str()))
            .collect()
    };

    let content = Paragraph::new(graph_lines)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Commit Graph"),
        )
        .style(if app.window_index == 1 {
            BORDER_STYLE
        } else {
            BORDER_DEFAULT_STYLE
        });

    f.render_widget(content, area);
}

fn draw_commit_dialog(f: &mut Frame, app: &App) {
    let area = f.area();
    let dialog_width = 70;
    let dialog_height = 16;

    let x = (area.width.saturating_sub(dialog_width)) / 2;
    let y = (area.height.saturating_sub(dialog_height)) / 2;

    let dialog_area = ratatui::layout::Rect {
        x,
        y,
        width: dialog_width,
        height: dialog_height,
    };

    f.render_widget(Clear, dialog_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(dialog_area);

    let summary_border_style = if !app.commit_focus_description {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let summary = Paragraph::new(app.commit_summary.as_str())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Commit Summary ")
                .border_style(summary_border_style),
        );

    f.render_widget(summary, chunks[0]);

    let description_border_style = if app.commit_focus_description {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let description = Paragraph::new(app.commit_description.as_str())
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Commit Description ")
                .border_style(description_border_style),
        );

    f.render_widget(description, chunks[1]);
}

fn draw_footer(area: Rect, app: &App, f: &mut Frame) {
    let focused_panel = if app.focused {
        "Focused"
    } else {
        match app.window_index {
            0 => "Tree",
            1 => "Commit Graph",
            2 => "Branches",
            3 => "Diff",
            _ => "Tree",
        }
    };

    let left_line = Line::from(vec![
        Span::styled(
            format!(" {} ", focused_panel),
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            format!(" {}", app.current_branch),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    f.render_widget(Paragraph::new(left_line), area);
}

fn draw_add_remote_dialog(f: &mut Frame, app: &App) {
    let area = f.area();
    let dialog_width = 70u16;
    let dialog_height = 10u16;

    let x = (area.width.saturating_sub(dialog_width)) / 2;
    let y = (area.height.saturating_sub(dialog_height)) / 2;

    let dialog_area = ratatui::layout::Rect {
        x,
        y,
        width: dialog_width,
        height: dialog_height,
    };

    f.render_widget(Clear, dialog_area);

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Add Remote ")
        .border_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let inner = outer_block.inner(dialog_area);
    f.render_widget(outer_block, dialog_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .split(inner);

    let name_border = if !app.add_remote_focus_url {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };
    let url_border = if app.add_remote_focus_url {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let name_input = Paragraph::new(app.add_remote_name.as_str())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Remote Name ")
                .border_style(name_border),
        );
    f.render_widget(name_input, chunks[0]);

    let url_input = Paragraph::new(app.add_remote_url.as_str())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Remote URL ")
                .border_style(url_border),
        );
    f.render_widget(url_input, chunks[1]);
}
