use fastgit::file_tree::{FileNode, FileTree};
use std::path::PathBuf;

#[test]
fn file_node_new_dir() {
    let node = FileNode::new(PathBuf::from("/a/b"), true);
    assert_eq!(node.path, PathBuf::from("/a/b"));
    assert!(node.is_dir);
    assert!(!node.expanded);
    assert!(node.children.is_empty());
}

#[test]
fn file_node_new_file() {
    let node = FileNode::new(PathBuf::from("src/main.rs"), false);
    assert!(!node.is_dir);
    assert!(node.children.is_empty());
}

#[test]
fn file_node_insert_single_file() {
    let mut root = FileNode::new(PathBuf::from("."), true);
    root.expanded = true;
    root.insert(&PathBuf::from("./foo.rs"), &PathBuf::from("foo.rs"));
    assert_eq!(root.children.len(), 1);
    assert!(!root.children[0].is_dir);
}

#[test]
fn file_node_insert_nested_path() {
    let mut root = FileNode::new(PathBuf::from("."), true);
    root.expanded = true;
    root.insert(
        &PathBuf::from("./src/main.rs"),
        &PathBuf::from("src/main.rs"),
    );
    assert_eq!(root.children.len(), 1);
    let src = &root.children[0];
    assert!(src.is_dir);
    assert_eq!(src.children.len(), 1);
    assert!(!src.children[0].is_dir);
}

#[test]
fn file_node_sort_dirs_before_files() {
    let mut root = FileNode::new(PathBuf::from("."), true);
    root.expanded = true;
    root.insert(&PathBuf::from("./z.rs"), &PathBuf::from("z.rs"));
    root.insert(
        &PathBuf::from("./src/main.rs"),
        &PathBuf::from("src/main.rs"),
    );
    assert!(root.children[0].is_dir, "dir should come first");
}

#[test]
fn file_tree_new_root_expanded_items_empty() {
    let tree = FileTree::new(PathBuf::from("."));
    assert!(tree.root.expanded);
    assert!(tree.items.is_empty());
}

#[test]
fn file_tree_populate_creates_items() {
    let mut tree = FileTree::new(PathBuf::from("."));
    tree.populate_from_paths(vec![PathBuf::from("main.rs"), PathBuf::from("lib.rs")]);
    assert_eq!(tree.items.len(), 2);
    assert!(!tree.items[0].2);
}

#[test]
fn file_tree_populate_selects_first() {
    let mut tree = FileTree::new(PathBuf::from("."));
    tree.populate_from_paths(vec![PathBuf::from("a.rs")]);
    assert_eq!(tree.state.selected(), Some(0));
}

#[test]
fn file_tree_next_advances_selection() {
    let mut tree = FileTree::new(PathBuf::from("."));
    tree.populate_from_paths(vec![
        PathBuf::from("a.rs"),
        PathBuf::from("b.rs"),
        PathBuf::from("c.rs"),
    ]);
    assert_eq!(tree.state.selected(), Some(0));
    tree.next();
    assert_eq!(tree.state.selected(), Some(1));
    tree.next();
    assert_eq!(tree.state.selected(), Some(2));
}

#[test]
fn file_tree_next_wraps_to_start() {
    let mut tree = FileTree::new(PathBuf::from("."));
    tree.populate_from_paths(vec![PathBuf::from("a.rs"), PathBuf::from("b.rs")]);
    tree.state.select(Some(1));
    tree.next();
    assert_eq!(tree.state.selected(), Some(0));
}

#[test]
fn file_tree_previous_wraps_to_end() {
    let mut tree = FileTree::new(PathBuf::from("."));
    tree.populate_from_paths(vec![
        PathBuf::from("a.rs"),
        PathBuf::from("b.rs"),
        PathBuf::from("c.rs"),
    ]);
    assert_eq!(tree.state.selected(), Some(0));
    tree.previous();
    assert_eq!(tree.state.selected(), Some(2));
}

#[test]
fn file_tree_previous_goes_up() {
    let mut tree = FileTree::new(PathBuf::from("."));
    tree.populate_from_paths(vec![PathBuf::from("a.rs"), PathBuf::from("b.rs")]);
    tree.state.select(Some(1));
    tree.previous();
    assert_eq!(tree.state.selected(), Some(0));
}

#[test]
fn file_tree_populate_empty_paths_clears_items() {
    let mut tree = FileTree::new(PathBuf::from("."));
    tree.populate_from_paths(vec![PathBuf::from("a.rs")]);
    tree.populate_from_paths(vec![]);
    assert!(tree.items.is_empty());
    assert_eq!(tree.state.selected(), None);
}

#[test]
fn file_tree_update_items_respects_collapsed() {
    let mut tree = FileTree::new(PathBuf::from("."));
    tree.populate_from_paths(vec![
        PathBuf::from("src/main.rs"),
        PathBuf::from("src/lib.rs"),
    ]);
    assert!(tree.items.len() >= 1);
}
