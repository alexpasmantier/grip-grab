use anyhow::Result;
use ignore::{types::TypesBuilder, WalkBuilder};

use std::path::{Path, PathBuf};

pub fn walk_builder(
    path: &Path,
    ignored_paths: &[PathBuf],
    n_threads: usize,
    respect_gitignore: bool,
    filter_filetypes: Vec<String>,
) -> WalkBuilder {
    let mut builder = WalkBuilder::new(path);

    // ft-based filtering
    let mut types_builder = TypesBuilder::new();
    types_builder.add_defaults();
    add_custom_filetypes(&mut types_builder).unwrap();
    filter_filetypes.iter().for_each(|ft| {
        types_builder.select(ft);
    });
    builder.types(types_builder.build().unwrap());

    // path-based filtering
    let ignored_paths = ignored_paths.to_vec();
    builder.filter_entry(move |entry| {
        for ignore in ignored_paths.iter() {
            if entry.path() == ignore {
                return false;
            }
        }
        true
    });

    // .gitignore filtering
    builder.git_ignore(respect_gitignore);

    builder.threads(n_threads);
    builder
}

fn add_custom_filetypes(types_builder: &mut TypesBuilder) -> Result<()> {
    Ok(types_builder.add("pystrict", "*.py")?)
}
