// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Fitrian Musya

use std::path::PathBuf;

use git2::Status;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogType {
    Warning,
    Error,
    Info,
    Success,
}

#[derive(Default)]
pub struct Helper;

impl Helper {
    pub fn get_txt_icon(&self, st: Status) -> &'static str {
        if st.contains(Status::WT_MODIFIED) || st.contains(Status::INDEX_MODIFIED) {
            "M"
        } else if st.contains(Status::WT_NEW) || st.contains(Status::INDEX_NEW) {
            "N"
        } else if st.contains(Status::WT_DELETED) || st.contains(Status::INDEX_DELETED) {
            "D"
        } else if st.contains(Status::WT_RENAMED) || st.contains(Status::INDEX_RENAMED) {
            "R"
        } else if st.contains(Status::WT_TYPECHANGE) || st.contains(Status::INDEX_TYPECHANGE) {
            "T"
        } else {
            "??"
        }
    }

    pub fn get_status_color(&self, st: Status) -> Color {
        if st.contains(Status::WT_NEW) || st.contains(Status::INDEX_NEW) {
            Color::Green
        } else if st.contains(Status::WT_MODIFIED) || st.contains(Status::INDEX_MODIFIED) {
            Color::Yellow
        } else if st.contains(Status::WT_DELETED) || st.contains(Status::INDEX_DELETED) {
            Color::Red
        } else if st.contains(Status::WT_RENAMED) || st.contains(Status::INDEX_RENAMED) {
            Color::Cyan
        } else {
            Color::Magenta
        }
    }

    /// Draw a centered dialog box with customizable type, title, and content
    ///
    /// # Arguments
    /// * `f` - The frame to render on
    /// * `dialog_type` - Type of dialog (Warning, Error, Info, Success)
    /// * `title` - Dialog title
    /// * `exit_hint` - Optional exit hint text (e.g., "<q> for exit")
    /// * `content` - Content lines to display
    /// * `width` - Dialog width (default: 60)
    /// * `height` - Dialog height (default: 10)
    pub fn draw_dialog(
        &self,
        f: &mut Frame,
        dialog_type: DialogType,
        title: &str,
        content: Vec<Line>,
        width: u16,
        height: u16,
    ) {
        let area = f.area();

        let x = (area.width.saturating_sub(width)) / 2;
        let y = (area.height.saturating_sub(height)) / 2;

        let dialog_area = ratatui::layout::Rect {
            x,
            y,
            width,
            height,
        };

        f.render_widget(Clear, dialog_area);

        let (color, icon) = match dialog_type {
            DialogType::Warning => (Color::Yellow, "⚠ "),
            DialogType::Error => (Color::Red, "✖ "),
            DialogType::Info => (Color::Cyan, "ℹ "),
            DialogType::Success => (Color::Green, "✓ "),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(" {} {} ", icon, title))
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD));

        let inner = block.inner(dialog_area);
        f.render_widget(block, dialog_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(inner);

        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[0]);

        let hint_text = Paragraph::new("<q> Quit")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Right);
        f.render_widget(hint_text, chunks[1]);
    }
}
