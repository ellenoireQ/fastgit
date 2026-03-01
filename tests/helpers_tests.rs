use fastgit::helper::helpers::{DialogType, Helper};
use git2::Status;
use ratatui::style::Color;

#[test]
fn get_txt_icon_index_new_is_staged() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::INDEX_NEW), "S");
}

#[test]
fn get_txt_icon_index_modified_is_staged() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::INDEX_MODIFIED), "S");
}

#[test]
fn get_txt_icon_index_deleted_is_staged() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::INDEX_DELETED), "S");
}

#[test]
fn get_txt_icon_index_renamed_is_staged() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::INDEX_RENAMED), "S");
}

#[test]
fn get_txt_icon_index_typechange_is_staged() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::INDEX_TYPECHANGE), "S");
}

#[test]
fn get_txt_icon_wt_modified() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::WT_MODIFIED), "M");
}

#[test]
fn get_txt_icon_wt_new() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::WT_NEW), "N");
}

#[test]
fn get_txt_icon_wt_deleted() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::WT_DELETED), "D");
}

#[test]
fn get_txt_icon_wt_renamed() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::WT_RENAMED), "R");
}

#[test]
fn get_txt_icon_wt_typechange() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::WT_TYPECHANGE), "T");
}

#[test]
fn get_txt_icon_unknown_returns_question_marks() {
    let h = Helper;
    assert_eq!(h.get_txt_icon(Status::CURRENT), "??");
}

#[test]
fn get_status_color_index_new_is_green() {
    let h = Helper;
    assert_eq!(h.get_status_color(Status::INDEX_NEW), Color::Green);
}

#[test]
fn get_status_color_index_modified_is_green() {
    let h = Helper;
    assert_eq!(h.get_status_color(Status::INDEX_MODIFIED), Color::Green);
}

#[test]
fn get_status_color_wt_new_is_green() {
    let h = Helper;
    assert_eq!(h.get_status_color(Status::WT_NEW), Color::Green);
}

#[test]
fn get_status_color_wt_modified_is_yellow() {
    let h = Helper;
    assert_eq!(h.get_status_color(Status::WT_MODIFIED), Color::Yellow);
}

#[test]
fn get_status_color_wt_deleted_is_red() {
    let h = Helper;
    assert_eq!(h.get_status_color(Status::WT_DELETED), Color::Red);
}

#[test]
fn get_status_color_wt_renamed_is_cyan() {
    let h = Helper;
    assert_eq!(h.get_status_color(Status::WT_RENAMED), Color::Cyan);
}

#[test]
fn get_status_color_unknown_is_magenta() {
    let h = Helper;
    assert_eq!(h.get_status_color(Status::CURRENT), Color::Magenta);
}

#[test]
fn dialog_type_partial_eq() {
    assert_eq!(DialogType::Warning, DialogType::Warning);
    assert_eq!(DialogType::Error, DialogType::Error);
    assert_eq!(DialogType::Info, DialogType::Info);
    assert_eq!(DialogType::Success, DialogType::Success);
    assert_ne!(DialogType::Warning, DialogType::Error);
    assert_ne!(DialogType::Info, DialogType::Success);
}
