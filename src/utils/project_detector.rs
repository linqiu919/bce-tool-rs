//! Project root detection utilities

use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;

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

/// Git facts about a checkout, read straight from `.git` (no git binary).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitWorkspace {
    /// Root of this checkout: the directory holding `.git`.
    pub root: PathBuf,
    /// Folder name of the repository this checkout belongs to. For a linked
    /// worktree that is the main repository (`.git/worktrees/<x>` points
    /// back to it); otherwise the checkout itself.
    pub repo_name: String,
    /// Checked-out branch, or "HEAD" when detached.
    pub branch: String,
    /// True for a linked `git worktree` checkout (`.git` is a file carrying
    /// a `commondir` pointer); false for the main checkout and submodules.
    pub worktree: bool,
}

/// Locate the checkout containing `path` and read its branch / worktree
/// state. None outside git.
pub fn detect_git_workspace(path: &Path) -> Option<GitWorkspace> {
    let root = path.ancestors().find(|p| p.join(".git").exists())?;
    let dot_git = root.join(".git");
    let (git_dir, worktree, repo_root) = if dot_git.is_dir() {
        (dot_git, false, root.to_path_buf())
    } else {
        // `.git` file: "gitdir: <path>" — a linked worktree
        // (.git/worktrees/<x>) or a submodule (.git/modules/<x>). Only
        // worktrees carry a `commondir` pointer back to the main repository.
        let text = fs::read_to_string(&dot_git).ok()?;
        let target = text.trim().strip_prefix("gitdir:")?.trim();
        let git_dir = resolve_from(root, target);
        match fs::read_to_string(git_dir.join("commondir")) {
            Ok(common) => {
                let common_dir = resolve_from(&git_dir, common.trim());
                let main_root =
                    repo_root_of_git_dir(&common_dir).unwrap_or_else(|| root.to_path_buf());
                (git_dir, true, main_root)
            }
            Err(_) => (git_dir, false, root.to_path_buf()),
        }
    };
    let branch = fs::read_to_string(git_dir.join("HEAD"))
        .ok()
        .and_then(|head| {
            head.trim()
                .strip_prefix("ref: refs/heads/")
                .map(str::to_string)
        })
        .unwrap_or_else(|| "HEAD".to_string());
    let mut repo_name = repo_root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    if let Some(stem) = repo_name.strip_suffix(".git") {
        repo_name = stem.to_string();
    }
    Some(GitWorkspace {
        root: root.to_path_buf(),
        repo_name,
        branch,
        worktree,
    })
}

/// Working tree a git dir belongs to: `<root>/.git` → `<root>`; a bare
/// repository (`repo.git`) is its own root.
fn repo_root_of_git_dir(git_dir: &Path) -> Option<PathBuf> {
    if git_dir.file_name().is_some_and(|n| n == ".git") {
        git_dir.parent().map(Path::to_path_buf)
    } else {
        Some(git_dir.to_path_buf())
    }
}

/// Resolve a possibly relative path against `base`, folding `.` and `..`
/// lexically (git writes `gitdir` / `commondir` pointers as relative paths).
fn resolve_from(base: &Path, target: &str) -> PathBuf {
    let target = Path::new(target);
    let joined = if target.is_absolute() {
        target.to_path_buf()
    } else {
        base.join(target)
    };
    let mut out = PathBuf::new();
    for component in joined.components() {
        match component {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Git block attached to server requests. Its presence (not just its values)
/// tells the server the branch is tracked, so sibling branches and worktrees
/// of one repository are archived as separate console projects.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GitMeta {
    pub branch: String,
    pub worktree: bool,
}

/// Workspace identity a request reports: the project name the console lists
/// it under and, inside git, the git block.
#[derive(Debug, Clone, Default)]
pub struct WorkspaceReport {
    pub project_name: Option<String>,
    pub git: Option<GitMeta>,
}

/// Servers older than the workspace fields reject unknown JSON fields with
/// 400; once that is seen the fields are dropped for the rest of the process
/// and the request is retried without them.
static LEGACY_SERVER: AtomicBool = AtomicBool::new(false);

pub fn note_legacy_server() {
    LEGACY_SERVER.store(true, Ordering::Relaxed);
}

pub fn is_unknown_field_rejection(status: u16, body: &str) -> bool {
    status == 400 && body.contains("unknown field")
}

/// Compute what a request from `project_root` reports about its workspace.
/// A linked worktree is listed under the repository it belongs to rather
/// than its own (often throwaway) folder name; a sub-directory of a checkout
/// keeps its own folder name, as before.
pub fn workspace_report(project_root: &Path) -> WorkspaceReport {
    if LEGACY_SERVER.load(Ordering::Relaxed) {
        return WorkspaceReport::default();
    }
    let folder = project_root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let nonempty = |s: String| if s.is_empty() { None } else { Some(s) };
    match detect_git_workspace(project_root) {
        Some(git) => {
            let name = if git.worktree && git.root.as_path() == project_root {
                git.repo_name.clone()
            } else {
                folder
            };
            WorkspaceReport {
                project_name: nonempty(name),
                git: Some(GitMeta {
                    branch: git.branch,
                    worktree: git.worktree,
                }),
            }
        }
        None => WorkspaceReport {
            project_name: nonempty(folder),
            git: None,
        },
    }
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
