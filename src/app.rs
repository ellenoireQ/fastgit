use std::env::current_dir;
use std::path::{Path, PathBuf};

use git2::Status;

use crate::file_tree::FileTree;

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Tree,
    Config,
    Diff,
}

pub struct App {
    pub current_tab: Tab,
    pub cur_dir: String,
    pub has_git: bool,
    pub tree: FileTree,
    pub states: Vec<States>,
    pub branches: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct States {
    pub state: Status,
    pub tree: FileTree,
}

impl App {
    pub fn new() -> Self {
        let mut app_new = Self {
            current_tab: Tab::Tree,
            has_git: false,
            cur_dir: "".to_string(),
            tree: FileTree::new(std::path::PathBuf::from(".")),
            states: vec![],
            branches: vec![],
        };
        app_new.get_path();
        app_new.scan_git();

        if app_new.has_git {
            if let Ok(repo) = git2::Repository::open(&app_new.cur_dir) {
                if let Ok(statuses) = repo.statuses(None) {
                    let paths: Vec<std::path::PathBuf> = statuses
                        .iter()
                        .filter(|e| !e.status().contains(Status::IGNORED))
                        .filter_map(|e| e.path().map(|p| std::path::PathBuf::from(p)))
                        .collect();

                    app_new.tree.populate_from_paths(paths);

                    for entry in statuses.iter() {
                        let status = entry.status();
                        let vu = States {
                            state: status,
                            tree: app_new.tree.clone(),
                        };
                        app_new.states.push(vu);
                    }
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
