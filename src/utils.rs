use std::path::{Path, PathBuf};

pub fn resolve_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths.into_iter().map(|pb| resolve_path(&pb)).collect()
}

pub fn resolve_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap()
}
