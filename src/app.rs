#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Commit,
    Config,
    Diff,
}

pub struct App {
    pub current_tab: Tab,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Commit,
        }
    }
    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Commit => Tab::Config,
            Tab::Config => Tab::Diff,
            Tab::Diff => Tab::Commit,
        };
    }

    pub fn prev_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Commit => Tab::Diff,
            Tab::Config => Tab::Commit,
            Tab::Diff => Tab::Config,
        };
    }
}
