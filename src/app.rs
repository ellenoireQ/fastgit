// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Fitrian Musya

use std::collections::HashMap;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

use git2::*;
use ratatui::widgets::ListState;

use crate::file_tree::FileTree;

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Tree,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffLineKind {
    Add,
    Delete,
    Context,
    Header,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub content: String,
}

pub struct App {
    pub current_tab: Tab,
    pub cur_dir: String,
    pub has_git: bool,
    pub current_branch: String,
    pub tree: FileTree,
    pub file_statuses: HashMap<PathBuf, Status>,
    pub branches: Vec<String>,
    pub branch_state: ListState,
    pub selected_file: Option<PathBuf>,
    pub diff_content: Vec<DiffLine>,
    pub diff_scroll: usize,
    pub focused: bool,
    pub window_index: u32,
    pub show_commit_dialog: bool,
    pub commit_summary: String,
    pub commit_description: String,
    pub commit_focus_description: bool,
    pub commit_warning_open: bool,
    pub commit_success_open: bool,
    pub staged_count: u32,
    pub push_error: Option<String>,
    pub push_success_open: bool,
    pub push_in_progress: bool,
    pub push_result_rx: Option<mpsc::Receiver<Result<(), String>>>,
    pub branch_focused: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app_new = Self {
            current_tab: Tab::Tree,
            has_git: false,
            cur_dir: "".to_string(),
            current_branch: "-".to_string(),
            tree: FileTree::new(std::path::PathBuf::from(".")),
            file_statuses: HashMap::new(),
            branches: vec![],
            branch_state: ListState::default(),
            selected_file: None,
            diff_content: vec![],
            diff_scroll: 0,
            focused: false,

            // 0 => Tree
            // 1 => Branch
            // 2 => Diff
            window_index: 0,
            show_commit_dialog: false,
            commit_summary: String::new(),
            commit_description: String::new(),
            commit_focus_description: false,
            commit_warning_open: false,
            commit_success_open: false,
            staged_count: 0,
            push_error: None,
            push_success_open: false,
            push_in_progress: false,
            push_result_rx: None,
            branch_focused: false,
        };
        app_new.get_path();
        app_new.scan_git();

        if app_new.has_git
            && let Ok(repo) = Repository::open(&app_new.cur_dir)
            && let Ok(statuses) = repo.statuses(None)
        {
            let mut paths: Vec<std::path::PathBuf> = Vec::new();

            for entry in statuses.iter() {
                if entry.status().contains(Status::IGNORED) {
                    continue;
                }
                if let Some(p) = entry.path() {
                    let path = std::path::PathBuf::from(p);
                    app_new.file_statuses.insert(path.clone(), entry.status());
                    paths.push(path);
                }
            }

            let mut options = git2::StatusOptions::new();
            options.include_untracked(true);

            if let Ok(statuses) = repo.statuses(Some(&mut options)) {
                for entry in statuses.iter() {
                    if entry.status().contains(Status::WT_NEW) {
                        if let Some(p) = entry.path() {
                            let path = std::path::PathBuf::from(p);
                            if !paths.contains(&path) {
                                app_new.file_statuses.insert(path.clone(), entry.status());
                                paths.push(path);
                            }
                        }
                    }
                }
            }

            app_new.tree.populate_from_paths(paths);

            if let Ok(branches) = repo.branches(None) {
                for branch in branches {
                    if let Ok((branch, _)) = branch
                        && let Ok(Some(name)) = branch.name()
                    {
                        app_new.branches.push(name.to_string());
                    }
                }
                if !app_new.branches.is_empty() {
                    app_new.branch_state.select(Some(0));
                }
            }
        }

        app_new
    }

    pub fn select_file(&mut self) {
        if let Some(i) = self.tree.state.selected()
            && let Some((path, _, is_dir)) = self.tree.items.get(i)
            && !is_dir
        {
            let file_path = path.strip_prefix(".").unwrap_or(path).to_path_buf();
            self.selected_file = Some(file_path);
            self.diff_scroll = 0;
            self.load_diff();
        }
    }

    pub fn load_diff(&mut self) {
        self.diff_content.clear();

        let file_path = match &self.selected_file {
            Some(p) => p.clone(),
            None => return,
        };

        if !self.has_git {
            return;
        }

        let repo = match Repository::open(&self.cur_dir) {
            Ok(r) => r,
            Err(_) => return,
        };

        let file_str = match file_path.to_str() {
            Some(s) => s.to_string(),
            None => return,
        };

        if let Some(&status) = self.file_statuses.get(&file_path) {
            if status.contains(Status::WT_NEW) && !status.contains(Status::INDEX_NEW) {
                let full_path = Path::new(&self.cur_dir).join(&file_path);
                if let Ok(content) = std::fs::read_to_string(&full_path) {
                    let lines: Vec<DiffLine> = content
                        .lines()
                        .map(|line| DiffLine {
                            kind: DiffLineKind::Add,
                            content: line.to_string(),
                        })
                        .collect();
                    self.diff_content = lines;
                }
                return;
            }
        }

        let mut opts = DiffOptions::new();
        opts.pathspec(&file_str);
        opts.context_lines(3);

        let head_tree = repo.head().ok().and_then(|h| h.peel_to_tree().ok());

        let diff = repo.diff_tree_to_workdir_with_index(head_tree.as_ref(), Some(&mut opts));

        if let Ok(diff) = diff {
            let mut lines: Vec<DiffLine> = Vec::new();

            let _ = diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
                let content = String::from_utf8_lossy(line.content()).to_string();
                let kind = match line.origin() {
                    '+' => DiffLineKind::Add,
                    '-' => DiffLineKind::Delete,
                    'H' | 'F' => DiffLineKind::Header,
                    _ => DiffLineKind::Context,
                };

                let kind = if content.starts_with("@@") {
                    DiffLineKind::Header
                } else {
                    kind
                };

                lines.push(DiffLine {
                    kind,
                    content: content.trim_end_matches('\n').to_string(),
                });
                true
            });

            self.diff_content = lines;
        }
    }

    /// Toggle the staged status of a file.
    /// If the file is currently staged, it will be unstaged,
    /// and vice versa.
    pub fn toggle_stage(&mut self, path: &Path) -> Result<(), Error> {
        let repo = Repository::open(&self.cur_dir)?;
        let mut index = repo.index()?;
        let staged_mask = Status::INDEX_NEW
            | Status::INDEX_MODIFIED
            | Status::INDEX_DELETED
            | Status::INDEX_RENAMED
            | Status::INDEX_TYPECHANGE;
        let status = repo.status_file(path).unwrap_or(Status::WT_NEW);

        if status.intersects(staged_mask) {
            if let Ok(head) = repo.head().and_then(|h| h.peel(ObjectType::Any)) {
                if let Some(path_str) = path.to_str() {
                    repo.reset_default(Some(&head), Some(path_str))?;
                } else {
                    index.remove_path(path)?;
                }
            } else {
                index.remove_path(path)?;
            }
        } else {
            index.add_path(path)?;
            self.staged_count += 1;
        }

        index.write()?;

        let updated_status = repo.status_file(path).unwrap_or(Status::CURRENT);
        let key = path.to_path_buf();
        if updated_status == Status::CURRENT {
            self.file_statuses.remove(&key);
        } else {
            self.file_statuses.insert(key, updated_status);
        }

        Ok(())
    }

    pub fn diff_scroll_down(&mut self) {
        if self.diff_scroll < self.diff_content.len().saturating_sub(1) {
            self.diff_scroll += 1;
        }
    }

    pub fn diff_scroll_up(&mut self) {
        self.diff_scroll = self.diff_scroll.saturating_sub(1);
    }

    pub fn branch_next(&mut self) {
        if self.branches.is_empty() {
            self.branch_state.select(None);
            return;
        }

        let next = match self.branch_state.selected() {
            Some(index) if index + 1 < self.branches.len() => index + 1,
            _ => 0,
        };

        self.branch_state.select(Some(next));
    }

    pub fn branch_previous(&mut self) {
        if self.branches.is_empty() {
            self.branch_state.select(None);
            return;
        }

        let prev = match self.branch_state.selected() {
            Some(0) | None => self.branches.len() - 1,
            Some(index) => index - 1,
        };

        self.branch_state.select(Some(prev));
    }

    pub fn increase_window(&mut self) {
        if self.window_index == 2 {
            return self.window_index = 0;
        }
        self.window_index += 1;
    }

    pub fn get_path(&mut self) {
        if let Ok(ok) = current_dir() {
            self.cur_dir = ok.display().to_string();
        }
    }

    pub fn scan_git(&mut self) {
        let dir = format!("{}/.git", self.cur_dir);
        if Path::new(dir.as_str()).is_dir() {
            self.has_git = true;
            self.refresh_current_branch();
            return;
        }
        self.has_git = false;
        self.current_branch = "-".to_string();
    }

    pub fn refresh_current_branch(&mut self) {
        if !self.has_git {
            self.current_branch = "-".to_string();
            return;
        }

        let branch = if let Ok(repo) = Repository::open(&self.cur_dir) {
            if let Ok(head) = repo.head() {
                head.shorthand().map(|name| name.to_string())
            } else {
                None
            }
        } else {
            None
        };

        self.current_branch = branch.unwrap_or_else(|| "detached".to_string());
    }

    pub fn open_commit_dialog(&mut self) {
        self.show_commit_dialog = true;
        self.commit_summary.clear();
        self.commit_description.clear();
        self.commit_focus_description = false;
    }

    pub fn close_commit_dialog(&mut self) {
        self.show_commit_dialog = false;
        self.commit_summary.clear();
        self.commit_description.clear();
        self.commit_focus_description = false;
    }

    pub fn commit_message_push(&mut self, c: char) {
        if self.commit_focus_description {
            self.commit_description.push(c);
        } else {
            self.commit_summary.push(c);
        }
    }

    pub fn commit_message_pop(&mut self) {
        if self.commit_focus_description {
            self.commit_description.pop();
        } else {
            self.commit_summary.pop();
        }
    }

    pub fn toggle_commit_focus(&mut self) {
        self.commit_focus_description = !self.commit_focus_description;
    }

    pub fn commit(&mut self) -> Result<Oid, Error> {
        let repo = Repository::open(&self.cur_dir)?;
        let mut index = repo.index()?;

        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;

        let signature = repo.signature()?;

        let message = if self.commit_description.is_empty() {
            self.commit_summary.clone()
        } else {
            format!("{}\n\n{}", self.commit_summary, self.commit_description)
        };

        let mut parents = Vec::new();
        if let Ok(head) = repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                parents.push(commit);
            }
        }

        let parent_refs: Vec<&Commit> = parents.iter().collect();

        let oid = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &message,
            &tree,
            &parent_refs,
        )?;

        self.staged_count = 0;

        Ok(oid)
    }

    pub fn start_push(&mut self) {
        if self.push_in_progress {
            return;
        }

        self.push_in_progress = true;

        let cur_dir = self.cur_dir.clone();
        let (tx, rx) = mpsc::channel();
        self.push_result_rx = Some(rx);

        std::thread::spawn(move || {
            let result = Self::push_repo_sync(&cur_dir);
            let _ = tx.send(result.map_err(|e| e.to_string()));
        });
    }

    pub fn check_push_result(&mut self) {
        if let Some(rx) = &self.push_result_rx {
            if let Ok(result) = rx.try_recv() {
                self.push_in_progress = false;
                self.push_result_rx = None;
                match result {
                    Ok(_) => self.push_success_open = true,
                    Err(err) => self.push_error = Some(err),
                }
            }
        }
    }

    fn push_repo_sync(cur_dir: &str) -> Result<(), Error> {
        let output = std::process::Command::new("git")
            .arg("push")
            .current_dir(cur_dir)
            .output()
            .map_err(|e| Error::from_str(&e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::from_str(stderr.trim()));
        }

        Ok(())
    }
}
