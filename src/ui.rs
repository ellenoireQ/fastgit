// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Fitrian Musya

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs},
};

use crate::{
    app::{App, DiffLineKind, Tab},
    helper::helpers::Helper,
};

const BORDER_STYLE: Style = Style::new().yellow().bold();
const BORDER_DEFAULT_STYLE: Style = Style::new().white().bold();

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(vertical_chunks[1]);

    draw_content(f, vertical_chunks[0], app);
    draw_footer(f, bottom_chunks[1]);
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
                f.render_widget(empty, rows[1]);
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

                let diff_list = List::new(visible_lines).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title(diff_title),
                );

                f.render_widget(diff_list, rows[1]);
            }
        }
        Tab::Config => {
            let block = Block::default().borders(Borders::ALL).title("Config");
            f.render_widget(block, area);
        }
    }
}

fn draw_footer(f: &mut Frame, area: ratatui::layout::Rect) {
    let footer = Paragraph::new("b | Open Book").block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    f.render_widget(footer, area);
}
