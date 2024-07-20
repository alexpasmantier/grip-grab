use std::path::PathBuf;

pub fn resolve_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths
        .into_iter()
        .map(|path| path.canonicalize().unwrap())
        .collect()
}
