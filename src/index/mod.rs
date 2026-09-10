//! Index module

mod manager;

pub use manager::{
    project_too_large_notice, Blob, FileEntry, IndexData, IndexManager, IndexResult, IndexStats,
    MAX_PROJECT_BYTES,
};
