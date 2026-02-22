use std::collections::HashMap;
use std::env::current_dir;
use std::path::{Path, PathBuf};

use git2::{DiffOptions, Status};

use crate::file_tree::FileTree;

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Tree,
    Config,
    Diff,
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
    pub tree: FileTree,
    pub file_statuses: HashMap<PathBuf, Status>,
    pub branches: Vec<String>,
    pub selected_file: Option<PathBuf>,
    pub diff_content: Vec<DiffLine>,
    pub diff_scroll: usize,
    pub focused: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app_new = Self {
            current_tab: Tab::Tree,
            has_git: false,
            cur_dir: "".to_string(),
            tree: FileTree::new(std::path::PathBuf::from(".")),
            file_statuses: HashMap::new(),
            branches: vec![],
            selected_file: None,
            diff_content: vec![],
            diff_scroll: 0,
            focused: false,
        };
        app_new.get_path();
        app_new.scan_git();

        if app_new.has_git {
            if let Ok(repo) = git2::Repository::open(&app_new.cur_dir) {
                if let Ok(statuses) = repo.statuses(None) {
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

                    app_new.tree.populate_from_paths(paths);
                }

                if let Ok(branches) = repo.branches(None) {
                    for branch in branches {
                        if let Ok((branch, _)) = branch {
                            if let Ok(Some(name)) = branch.name() {
                                app_new.branches.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        app_new
    }

    pub fn select_file(&mut self) {
        if let Some(i) = self.tree.state.selected() {
            if let Some((path, _, is_dir)) = self.tree.items.get(i) {
                if !is_dir {
                    let file_path = path.strip_prefix(".").unwrap_or(path).to_path_buf();
                    self.selected_file = Some(file_path);
                    self.diff_scroll = 0;
                    self.load_diff();
                }
            }
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

        let repo = match git2::Repository::open(&self.cur_dir) {
            Ok(r) => r,
            Err(_) => return,
        };

        let file_str = match file_path.to_str() {
            Some(s) => s.to_string(),
            None => return,
        };

        let mut opts = DiffOptions::new();
        opts.pathspec(&file_str);
        opts.context_lines(3);

        let head_tree = repo.head().ok().and_then(|h| h.peel_to_tree().ok());

        let diff = repo.diff_tree_to_workdir_with_index(head_tree.as_ref(), Some(&mut opts));

        if let Ok(diff) = diff {
            let mut lines: Vec<DiffLine> = Vec::new();

            let _ = diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
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

    pub fn diff_scroll_down(&mut self) {
        if self.diff_scroll < self.diff_content.len().saturating_sub(1) {
            self.diff_scroll += 1;
        }
    }

    pub fn diff_scroll_up(&mut self) {
        self.diff_scroll = self.diff_scroll.saturating_sub(1);
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Tree => Tab::Config,
            Tab::Config => Tab::Diff,
            Tab::Diff => Tab::Tree,
        };
    }

    pub fn prev_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Tree => Tab::Diff,
            Tab::Config => Tab::Tree,
            Tab::Diff => Tab::Config,
        };
    }

    pub fn get_path(&mut self) {
        if let Ok(ok) = current_dir() {
            self.cur_dir = ok.display().to_string();
        }
    }

    pub fn scan_git(&mut self) {
        let dir = format!("{}/.git", self.cur_dir);
        if Path::new(dir.as_str()).is_dir() {
            return self.has_git = true;
        }
        self.has_git = false;
    }
}
