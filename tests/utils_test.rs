//! Tests for utils module

use std::fs;
use tempfile::TempDir;

use bce_tool::utils::project_detector::{get_bce_dir, get_index_file_path};

#[test]
fn test_get_bce_dir_creates_directory() {
    let temp_dir = TempDir::new().unwrap();
    let bce_dir = get_bce_dir(temp_dir.path());

    assert!(bce_dir.exists());
    assert!(bce_dir.is_dir());
    assert_eq!(bce_dir, temp_dir.path().join(".bce-tool"));
}

#[test]
fn test_get_bce_dir_idempotent() {
    let temp_dir = TempDir::new().unwrap();

    // Call twice
    let bce_dir1 = get_bce_dir(temp_dir.path());
    let bce_dir2 = get_bce_dir(temp_dir.path());

    assert_eq!(bce_dir1, bce_dir2);
    assert!(bce_dir1.exists());
}

#[test]
fn test_get_bce_dir_adds_to_gitignore_new_file() {
    let temp_dir = TempDir::new().unwrap();
    let gitignore_path = temp_dir.path().join(".gitignore");

    // No .gitignore initially
    assert!(!gitignore_path.exists());

    // Create .bce-tool dir
    get_bce_dir(temp_dir.path());

    // .gitignore should now exist with .bce-tool
    assert!(gitignore_path.exists());
    let content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(content.contains(".bce-tool/"));
}

#[test]
fn test_get_bce_dir_adds_to_existing_gitignore() {
    let temp_dir = TempDir::new().unwrap();
    let gitignore_path = temp_dir.path().join(".gitignore");

    // Create existing .gitignore
    fs::write(&gitignore_path, "node_modules/\n").unwrap();

    // Create .bce-tool dir
    get_bce_dir(temp_dir.path());

    // .gitignore should contain both
    let content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(content.contains("node_modules/"));
    assert!(content.contains(".bce-tool/"));
}

#[test]
fn test_get_bce_dir_does_not_duplicate_in_gitignore() {
    let temp_dir = TempDir::new().unwrap();
    let gitignore_path = temp_dir.path().join(".gitignore");

    // Create .gitignore that already has .bce-tool
    fs::write(&gitignore_path, "node_modules/\n.bce-tool/\n").unwrap();

    // Create .bce-tool dir
    get_bce_dir(temp_dir.path());

    // Should not have duplicate entries
    let content = fs::read_to_string(&gitignore_path).unwrap();
    let count = content.matches(".bce-tool").count();
    assert_eq!(count, 1);
}

#[test]
fn test_get_bce_dir_ignores_similar_gitignore_entries() {
    let temp_dir = TempDir::new().unwrap();
    let gitignore_path = temp_dir.path().join(".gitignore");

    // .bce-tooling should not block adding .bce-tool/
    fs::write(&gitignore_path, ".bce-tooling/\n").unwrap();

    get_bce_dir(temp_dir.path());

    let content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(content.contains(".bce-tool/"));
    let count = content
        .lines()
        .filter(|line| {
            let line = line.trim();
            line == ".bce-tool" || line == ".bce-tool/"
        })
        .count();
    assert_eq!(count, 1);
}

#[test]
fn test_get_bce_dir_handles_gitignore_without_trailing_newline() {
    let temp_dir = TempDir::new().unwrap();
    let gitignore_path = temp_dir.path().join(".gitignore");

    // Create .gitignore without trailing newline
    fs::write(&gitignore_path, "node_modules/").unwrap();

    // Create .bce-tool dir
    get_bce_dir(temp_dir.path());

    // Should add newline before .bce-tool
    let content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(content.contains("node_modules/\n.bce-tool/"));
}

#[test]
fn test_get_index_file_path() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = get_index_file_path(temp_dir.path());

    assert_eq!(
        index_path,
        temp_dir.path().join(".bce-tool").join("index.bin")
    );
    // The .bce-tool directory should have been created
    assert!(temp_dir.path().join(".bce-tool").exists());
}

#[test]
fn test_get_index_file_path_consistent() {
    let temp_dir = TempDir::new().unwrap();

    let path1 = get_index_file_path(temp_dir.path());
    let path2 = get_index_file_path(temp_dir.path());

    assert_eq!(path1, path2);
}
