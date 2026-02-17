#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Tree,
    Config,
    Diff,
}

pub struct App {
    pub current_tab: Tab,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Tree,
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
}
