use fastgit::app::{App, BranchTab, DiffLine, DiffLineKind, Tab};

#[test]
fn tab_enum_equality() {
    assert_eq!(Tab::Tree, Tab::Tree);
}

#[test]
fn branch_tab_enum_equality() {
    assert_eq!(BranchTab::Local, BranchTab::Local);
    assert_ne!(BranchTab::Local, BranchTab::Remote);
}

#[test]
fn diff_line_kind_equality() {
    assert_eq!(DiffLineKind::Add, DiffLineKind::Add);
    assert_ne!(DiffLineKind::Add, DiffLineKind::Delete);
    assert_ne!(DiffLineKind::Context, DiffLineKind::Header);
}

#[test]
fn diff_line_stores_kind_and_content() {
    let dl = DiffLine {
        kind: DiffLineKind::Delete,
        content: "-old line".to_string(),
    };
    assert_eq!(dl.kind, DiffLineKind::Delete);
    assert_eq!(dl.content, "-old line");
}

#[test]
fn diff_scroll_down_increments() {
    let mut app = App::new();
    app.diff_content = vec![
        DiffLine { kind: DiffLineKind::Add, content: "a".to_string() },
        DiffLine { kind: DiffLineKind::Add, content: "b".to_string() },
        DiffLine { kind: DiffLineKind::Add, content: "c".to_string() },
    ];
    app.diff_scroll = 0;
    app.diff_scroll_down();
    assert_eq!(app.diff_scroll, 1);
    app.diff_scroll_down();
    assert_eq!(app.diff_scroll, 2);
}

#[test]
fn diff_scroll_down_stops_at_last() {
    let mut app = App::new();
    app.diff_content = vec![
        DiffLine { kind: DiffLineKind::Add, content: "a".to_string() },
        DiffLine { kind: DiffLineKind::Add, content: "b".to_string() },
    ];
    app.diff_scroll = 1;
    app.diff_scroll_down();
    assert_eq!(app.diff_scroll, 1);
}

#[test]
fn diff_scroll_down_empty_stays_zero() {
    let mut app = App::new();
    app.diff_content.clear();
    app.diff_scroll = 0;
    app.diff_scroll_down();
    assert_eq!(app.diff_scroll, 0);
}

#[test]
fn diff_scroll_up_decrements() {
    let mut app = App::new();
    app.diff_scroll = 3;
    app.diff_scroll_up();
    assert_eq!(app.diff_scroll, 2);
}

#[test]
fn diff_scroll_up_no_underflow() {
    let mut app = App::new();
    app.diff_scroll = 0;
    app.diff_scroll_up();
    assert_eq!(app.diff_scroll, 0);
}

#[test]
fn increase_window_cycles_0_through_3() {
    let mut app = App::new();
    app.window_index = 0;
    app.increase_window();
    assert_eq!(app.window_index, 1);
    app.increase_window();
    assert_eq!(app.window_index, 2);
    app.increase_window();
    assert_eq!(app.window_index, 3);
    app.increase_window();
    assert_eq!(app.window_index, 0);
}

#[test]
fn open_commit_dialog_sets_flag_and_clears_fields() {
    let mut app = App::new();
    app.commit_summary = "dirty".to_string();
    app.commit_description = "dirty".to_string();
    app.commit_focus_description = true;
    app.commit_summary_cursor = 5;
    app.open_commit_dialog();
    assert!(app.show_commit_dialog);
    assert!(app.commit_summary.is_empty());
    assert!(app.commit_description.is_empty());
    assert!(!app.commit_focus_description);
    assert_eq!(app.commit_summary_cursor, 0);
    assert_eq!(app.commit_description_cursor, 0);
    assert_eq!(app.commit_summary_scroll, 0);
    assert_eq!(app.commit_description_scroll, 0);
}

#[test]
fn close_commit_dialog_clears_flag_and_fields() {
    let mut app = App::new();
    app.show_commit_dialog = true;
    app.commit_summary = "msg".to_string();
    app.commit_description = "desc".to_string();
    app.commit_summary_cursor = 3;
    app.close_commit_dialog();
    assert!(!app.show_commit_dialog);
    assert!(app.commit_summary.is_empty());
    assert!(app.commit_description.is_empty());
    assert_eq!(app.commit_summary_cursor, 0);
}

#[test]
fn commit_message_insert_into_summary() {
    let mut app = App::new();
    app.commit_focus_description = false;
    app.commit_summary.clear();
    app.commit_summary_cursor = 0;
    app.commit_message_insert('H');
    app.commit_message_insert('i');
    assert_eq!(app.commit_summary, "Hi");
    assert_eq!(app.commit_summary_cursor, 2);
}

#[test]
fn commit_message_insert_into_description() {
    let mut app = App::new();
    app.commit_focus_description = true;
    app.commit_description.clear();
    app.commit_description_cursor = 0;
    app.commit_message_insert('O');
    app.commit_message_insert('k');
    assert_eq!(app.commit_description, "Ok");
    assert_eq!(app.commit_description_cursor, 2);
}

#[test]
fn commit_message_insert_at_middle() {
    let mut app = App::new();
    app.commit_focus_description = false;
    app.commit_summary = "ac".to_string();
    app.commit_summary_cursor = 1;
    app.commit_message_insert('b');
    assert_eq!(app.commit_summary, "abc");
    assert_eq!(app.commit_summary_cursor, 2);
}

#[test]
fn commit_message_backspace_removes_char() {
    let mut app = App::new();
    app.commit_focus_description = false;
    app.commit_summary = "Hi".to_string();
    app.commit_summary_cursor = 2;
    app.commit_message_backspace();
    assert_eq!(app.commit_summary, "H");
    assert_eq!(app.commit_summary_cursor, 1);
}

#[test]
fn commit_message_backspace_at_start_is_noop() {
    let mut app = App::new();
    app.commit_focus_description = false;
    app.commit_summary = "Hi".to_string();
    app.commit_summary_cursor = 0;
    app.commit_message_backspace();
    assert_eq!(app.commit_summary, "Hi");
    assert_eq!(app.commit_summary_cursor, 0);
}

#[test]
fn commit_message_delete_removes_at_cursor() {
    let mut app = App::new();
    app.commit_focus_description = false;
    app.commit_summary = "Hi".to_string();
    app.commit_summary_cursor = 0;
    app.commit_message_delete();
    assert_eq!(app.commit_summary, "i");
}

#[test]
fn commit_message_delete_at_end_is_noop() {
    let mut app = App::new();
    app.commit_focus_description = false;
    app.commit_summary = "Hi".to_string();
    app.commit_summary_cursor = 2;
    app.commit_message_delete();
    assert_eq!(app.commit_summary, "Hi");
}

#[test]
fn toggle_commit_focus_toggles_field() {
    let mut app = App::new();
    assert!(!app.commit_focus_description);
    app.toggle_commit_focus();
    assert!(app.commit_focus_description);
    app.toggle_commit_focus();
    assert!(!app.commit_focus_description);
}

#[test]
fn scan_git_non_git_dir_sets_false() {
    let mut app = App::new();
    app.cur_dir = "/tmp".to_string();
    app.scan_git();
    assert!(!app.has_git);
}

#[test]
fn branch_tab_toggle_local_to_remote() {
    let mut app = App::new();
    assert_eq!(app.branch_tab, BranchTab::Local);
    app.branch_tab_toggle();
    assert_eq!(app.branch_tab, BranchTab::Remote);
}

#[test]
fn branch_tab_toggle_remote_to_local() {
    let mut app = App::new();
    app.branch_tab = BranchTab::Remote;
    app.branch_tab_toggle();
    assert_eq!(app.branch_tab, BranchTab::Local);
}

#[test]
fn branch_next_advances_selection() {
    let mut app = App::new();
    app.branches = vec!["main".to_string(), "dev".to_string(), "feat".to_string()];
    app.branch_state.select(Some(0));
    app.branch_next();
    assert_eq!(app.branch_state.selected(), Some(1));
    app.branch_next();
    assert_eq!(app.branch_state.selected(), Some(2));
}

#[test]
fn branch_next_wraps_to_start() {
    let mut app = App::new();
    app.branches = vec!["main".to_string(), "dev".to_string()];
    app.branch_state.select(Some(1));
    app.branch_next();
    assert_eq!(app.branch_state.selected(), Some(0));
}

#[test]
fn branch_next_empty_selects_none() {
    let mut app = App::new();
    app.branches.clear();
    app.branch_next();
    assert_eq!(app.branch_state.selected(), None);
}

#[test]
fn branch_previous_wraps_to_last() {
    let mut app = App::new();
    app.branches = vec!["main".to_string(), "dev".to_string(), "feat".to_string()];
    app.branch_state.select(Some(0));
    app.branch_previous();
    assert_eq!(app.branch_state.selected(), Some(2));
}

#[test]
fn branch_previous_decrements() {
    let mut app = App::new();
    app.branches = vec!["main".to_string(), "dev".to_string()];
    app.branch_state.select(Some(1));
    app.branch_previous();
    assert_eq!(app.branch_state.selected(), Some(0));
}

#[test]
fn branch_previous_empty_selects_none() {
    let mut app = App::new();
    app.branches.clear();
    app.branch_previous();
    assert_eq!(app.branch_state.selected(), None);
}

#[test]
fn open_add_remote_dialog_sets_state() {
    let mut app = App::new();
    app.open_add_remote_dialog();
    assert!(app.show_add_remote_dialog);
    assert!(app.add_remote_name.is_empty());
    assert!(app.add_remote_url.is_empty());
    assert!(!app.add_remote_focus_url);
}

#[test]
fn close_add_remote_dialog_resets_state() {
    let mut app = App::new();
    app.show_add_remote_dialog = true;
    app.add_remote_name = "origin".to_string();
    app.add_remote_url = "https://example.com".to_string();
    app.close_add_remote_dialog();
    assert!(!app.show_add_remote_dialog);
    assert!(app.add_remote_name.is_empty());
    assert!(app.add_remote_url.is_empty());
}

#[test]
fn add_remote_input_push_name() {
    let mut app = App::new();
    app.add_remote_focus_url = false;
    app.add_remote_name.clear();
    app.add_remote_input_push('o');
    app.add_remote_input_push('r');
    assert_eq!(app.add_remote_name, "or");
}

#[test]
fn add_remote_input_push_url() {
    let mut app = App::new();
    app.add_remote_focus_url = true;
    app.add_remote_url.clear();
    app.add_remote_input_push('h');
    app.add_remote_input_push('t');
    assert_eq!(app.add_remote_url, "ht");
}

#[test]
fn add_remote_input_pop_name() {
    let mut app = App::new();
    app.add_remote_focus_url = false;
    app.add_remote_name = "origin".to_string();
    app.add_remote_input_pop();
    assert_eq!(app.add_remote_name, "origi");
}

#[test]
fn add_remote_input_pop_url() {
    let mut app = App::new();
    app.add_remote_focus_url = true;
    app.add_remote_url = "https://x.com".to_string();
    app.add_remote_input_pop();
    assert_eq!(app.add_remote_url, "https://x.co");
}

#[test]
fn open_new_branch_dialog_sets_flag() {
    let mut app = App::new();
    app.new_branch_name = "old".to_string();
    app.open_new_branch_dialog();
    assert!(app.show_new_branch_dialog);
    assert!(app.new_branch_name.is_empty());
}

#[test]
fn close_new_branch_dialog_resets_state() {
    let mut app = App::new();
    app.show_new_branch_dialog = true;
    app.new_branch_name = "feature".to_string();
    app.close_new_branch_dialog();
    assert!(!app.show_new_branch_dialog);
    assert!(app.new_branch_name.is_empty());
}

#[test]
fn remote_next_wraps_to_start() {
    let mut app = App::new();
    app.remotes = vec![
        ("origin".to_string(), "https://a.com".to_string()),
        ("upstream".to_string(), "https://b.com".to_string()),
    ];
    app.remote_state.select(Some(1));
    app.remote_next();
    assert_eq!(app.remote_state.selected(), Some(0));
}

#[test]
fn remote_previous_wraps_to_last() {
    let mut app = App::new();
    app.remotes = vec![
        ("origin".to_string(), "https://a.com".to_string()),
        ("upstream".to_string(), "https://b.com".to_string()),
    ];
    app.remote_state.select(Some(0));
    app.remote_previous();
    assert_eq!(app.remote_state.selected(), Some(1));
}

#[test]
fn remote_next_empty_selects_none() {
    let mut app = App::new();
    app.remotes.clear();
    app.remote_next();
    assert_eq!(app.remote_state.selected(), None);
}
