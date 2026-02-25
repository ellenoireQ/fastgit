// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Fitrian Musya

use std::path::PathBuf;

use git2::Status;
use ratatui::style::Color;

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
}
