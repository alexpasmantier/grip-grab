use ignore::{types::TypesBuilder, WalkBuilder};

use std::path::PathBuf;

pub fn walk_builder(
    paths: &[PathBuf],
    ignored_paths: &[PathBuf],
    // TODO: add ftype filtering
    //filetypes: &Vec<String>,
) -> WalkBuilder {
    let mut types_builder = TypesBuilder::new();
    types_builder.add_defaults();

    let mut walk_builder = WalkBuilder::new(&paths[0]);
    for path in paths.iter().skip(1) {
        walk_builder.add(path);
    }
    let ignored_paths = ignored_paths.to_vec();
    walk_builder.filter_entry(move |entry| {
        for ignore in ignored_paths.iter() {
            if entry.path() == ignore {
                return false;
            }
        }
        true
    });
    walk_builder.types(types_builder.build().unwrap());
    walk_builder
}
