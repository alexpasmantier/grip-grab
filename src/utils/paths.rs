use std::path::PathBuf;

pub fn resolve_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths.into_iter().map(|path| resolve_path(path)).collect()
}

pub fn resolve_path(path: PathBuf) -> PathBuf {
    path.canonicalize().unwrap()
}
