use std::env::current_dir;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Tree,
    Config,
    Diff,
}

pub struct App {
    pub current_tab: Tab,
    pub has_git: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Tree,
            has_git: false,
        }
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

    pub fn scan_git(&mut self) {
        if let Ok(act) = current_dir() {
            let dir = format!("{}/.git", act.display());
            if Path::new(dir.as_str()).is_dir() {
                return self.has_git = true;
            }
            self.has_git = false;
        }
    }
}
