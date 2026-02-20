use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, ListItem, Paragraph, Tabs},
};

use crate::{
    app::{App, Tab},
    helper::helpers::Helper,
};

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_tabs(f, chunks[0], app);
    draw_content(f, chunks[1], app);
    draw_footer(f, chunks[2]);
}

fn draw_tabs(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let titles = vec!["Tree", "Config", "Diff"];

    let selected = match app.current_tab {
        Tab::Tree => 0,
        Tab::Config => 1,
        Tab::Diff => 2,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("fastgit"))
        .select(selected)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn draw_content(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    match app.current_tab {
        Tab::Tree => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(area);

            let h = Helper::default();

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
                                .fg(Color::Blue)
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

            let branch_list = ratatui::widgets::List::new(branches)
                .block(Block::default().borders(Borders::ALL).title("Branches"));

            if items.is_empty() {
                let empty = Paragraph::new("Working tree is clean")
                    .block(Block::default().borders(Borders::ALL).title("Tree"));
                f.render_widget(empty, chunks[0]);
            } else {
                let list = ratatui::widgets::List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Tree"))
                    .highlight_style(
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("▶ ");

                f.render_stateful_widget(list, chunks[0], &mut app.tree.state);
            }

            f.render_widget(branch_list, chunks[1]);
        }
        Tab::Config => {
            let block = Block::default().borders(Borders::ALL).title("Config");
            f.render_widget(block, area);
        }
        Tab::Diff => {
            let block = Block::default().borders(Borders::ALL).title("Diff");
            f.render_widget(block, area);
        }
    }
}

fn draw_footer(f: &mut Frame, area: ratatui::layout::Rect) {
    let footer = Paragraph::new("q: Quit | Tab: Next | Shift+Tab: Prev")
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
