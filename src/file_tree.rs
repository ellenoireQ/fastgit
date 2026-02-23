// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Fitrian Musya

use ratatui::widgets::ListState;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub expanded: bool,
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
        Self {
            path,
            is_dir,
            expanded: false,
            children: vec![],
        }
    }

    pub fn insert(&mut self, full_path: &Path, relative_path: &Path) {
        let mut components = relative_path.components();
        if let Some(component) = components.next() {
            let component_path = PathBuf::from(component.as_os_str());
            let current_full_path = self.path.join(&component_path);

            let mut found_idx = None;
            for (idx, child) in self.children.iter().enumerate() {
                if child.path == current_full_path {
                    found_idx = Some(idx);
                    break;
                }
            }

            let child_node = if let Some(idx) = found_idx {
                &mut self.children[idx]
            } else {
                let is_dir = components.as_path().components().next().is_some();
                let new_node = FileNode::new(current_full_path, is_dir);
                self.children.push(new_node);
                self.children.last_mut().unwrap()
            };

            child_node.insert(full_path, components.as_path());
        } else {
            self.is_dir = false;
        }

        self.sort_children();
    }

    fn sort_children(&mut self) {
        self.children.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir)
                .then(a.path.file_name().cmp(&b.path.file_name()))
        });
    }
}

#[derive(Debug, Clone)]
pub struct FileTree {
    pub root: FileNode,
    pub state: ListState,
    pub items: Vec<(PathBuf, usize, bool)>,
}

impl FileTree {
    pub fn new(root_path: PathBuf) -> Self {
        let mut root = FileNode::new(root_path, true);
        root.expanded = true;

        Self {
            root,
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn populate_from_paths(&mut self, paths: Vec<PathBuf>) {
        self.root.children.clear();
        let root_path = self.root.path.clone();

        for path in paths {
            let relative_path = if path.is_absolute() {
                path.strip_prefix(&root_path).unwrap_or(&path)
            } else {
                &path
            };

            self.root
                .insert(&root_path.join(relative_path), relative_path);
        }
        self.update_items();
        if !self.items.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn update_items(&mut self) {
        self.items.clear();
        if self.root.expanded {
            for child in &self.root.children {
                Self::flatten_recursive(child, 0, &mut self.items);
            }
        }
    }

    fn flatten_recursive(node: &FileNode, depth: usize, items: &mut Vec<(PathBuf, usize, bool)>) {
        items.push((node.path.clone(), depth, node.is_dir));
        if node.expanded {
            for child in &node.children {
                Self::flatten_recursive(child, depth + 1, items);
            }
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn toggle_expand(&mut self) {
        if let Some(i) = self.state.selected() {
            if let Some((path, _, is_dir)) = self.items.get(i).cloned() {
                if is_dir {
                    if let Some(node) = Self::find_node_recursive(&mut self.root, &path) {
                        node.expanded = !node.expanded;
                    }
                    self.update_items();
                }
            }
        }
    }

    pub fn collapse_or_parent(&mut self) {
        if let Some(i) = self.state.selected() {
            if let Some((path, depth, is_dir)) = self.items.get(i).cloned() {
                if is_dir {
                    if let Some(node) = Self::find_node_recursive(&mut self.root, &path) {
                        if node.expanded {
                            node.expanded = false;
                            self.update_items();
                            return;
                        }
                    }
                }

                if depth > 0 {
                    let mut parent_idx = i;
                    while parent_idx > 0 {
                        parent_idx -= 1;
                        if let Some((_, d, _)) = self.items.get(parent_idx) {
                            if *d < depth {
                                self.state.select(Some(parent_idx));
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    fn find_node_recursive<'a>(node: &'a mut FileNode, path: &Path) -> Option<&'a mut FileNode> {
        if node.path == path {
            return Some(node);
        }
        for child in &mut node.children {
            if path.starts_with(&child.path) {
                if let Some(found) = Self::find_node_recursive(child, path) {
                    return Some(found);
                }
            }
        }
        None
    }
}
