//! Project root detection utilities

use std::fs;
use std::path::{Path, PathBuf};

/// Get the .bce-tool directory path for a project
/// Creates the directory if it doesn't exist
pub fn get_bce_dir(project_root: &Path) -> PathBuf {
    let bce_dir = project_root.join(".bce-tool");

    if !bce_dir.exists() {
        if let Err(e) = fs::create_dir_all(&bce_dir) {
            tracing::warn!("Failed to create .bce-tool directory: {}", e);
        } else {
            // Try to add .bce-tool to .gitignore
            add_to_gitignore(project_root);
        }
    }

    bce_dir
}

/// Add .bce-tool to .gitignore
fn add_to_gitignore(project_root: &Path) {
    let gitignore_path = project_root.join(".gitignore");

    let content = if gitignore_path.exists() {
        match fs::read_to_string(&gitignore_path) {
            Ok(c) => c,
            Err(_) => return,
        }
    } else {
        String::new()
    };

    // Check if already included
    if gitignore_has_bce_tool(&content) {
        return;
    }

    // Add .bce-tool to .gitignore
    let new_content = if content.ends_with('\n') || content.is_empty() {
        format!("{}.bce-tool/\n", content)
    } else {
        format!("{}\n.bce-tool/\n", content)
    };

    if let Err(e) = fs::write(&gitignore_path, new_content) {
        tracing::warn!("Failed to update .gitignore: {}", e);
    }
}

fn gitignore_has_bce_tool(content: &str) -> bool {
    content.lines().any(|line| {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return false;
        }
        let entry = line.split('#').next().unwrap_or(line).trim();
        entry == ".bce-tool" || entry == ".bce-tool/"
    })
}

/// Get index file path
pub fn get_index_file_path(project_root: &Path) -> PathBuf {
    let bce_dir = get_bce_dir(project_root);
    bce_dir.join("index.bin")
}

/// Check whether a directory is inside a Git repository.
/// Walks up from `path` through its ancestors looking for a `.git` entry.
/// `.git` may be a directory (normal repo) or a file (worktree / submodule).
pub fn is_inside_git_repo(path: &Path) -> bool {
    path.ancestors().any(|p| p.join(".git").exists())
}

/// AI-facing notice returned by MCP tools when the target directory is not
/// inside a Git repository, so the model understands why the tool is
/// unavailable and can explain it to the user instead of retrying blindly.
pub fn non_git_repo_notice(path: &str) -> String {
    format!(
        "TOOL UNAVAILABLE: '{}' is not managed by Git (no .git found in this directory or any parent).\n\
         bce-tool only works inside Git repositories, to avoid indexing personal folders (e.g. Downloads or Desktop).\n\
         Please inform the user: the current folder is not a Git repository, so codebase search and prompt enhancement cannot be used here.\n\
         Suggested fixes: run 'git init' in the project root if this is a real project, or reopen a Git-managed project directory.\n\
         Do not retry this tool on the same directory.",
        path
    )
}
