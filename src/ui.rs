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
    app::{App, DiffLineKind, Tab},
    helper::helpers::{Dialog, DialogType, Helper},
};

const BORDER_STYLE: Style = Style::new().yellow().bold();
const BORDER_DEFAULT_STYLE: Style = Style::new().white().bold();

pub fn draw_ui(f: &mut Frame, app: &mut App) {
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

    draw_footer(vertical_chunks[1], app, f);
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
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
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

            let branches: Vec<ListItem> = app
                .branches
                .iter()
                .map(|b| ListItem::new(b.as_str()))
                .collect();

            if items.is_empty() {
                let empty = Paragraph::new("Working tree is clean")
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title("[1]-Tree"),
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
                            .title("[1]-Tree"),
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

            let branch_list = ratatui::widgets::List::new(branches)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("[2]-Branches"),
                )
                .style(if app.window_index == 1 {
                    BORDER_STYLE
                } else {
                    BORDER_DEFAULT_STYLE
                });

            f.render_widget(branch_list, top_cols[1]);

            let diff_title = match &app.selected_file {
                Some(p) => format!("[3]-Diff — {}", p.display()),
                None => "[3]-Diff — No file selected".to_string(),
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
                    .style(if app.window_index == 2 {
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
                    .style(if app.window_index == 2 {
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
            1 => "Branches",
            2 => "Diff",
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
            " master",
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        ),
    ]);

    f.render_widget(Paragraph::new(left_line), area);
}
