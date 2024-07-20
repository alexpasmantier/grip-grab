use clap::Parser;
use cli::process_cli_args;
use std::io;
use std::io::Write;

use crossbeam::queue::ArrayQueue;
use fs::walk_builder;
use ignore::DirEntry;
use search::{build_matcher, search_file};

use crate::cli::Cli;

mod cli;
mod fs;
mod search;
mod utils;

pub fn main() -> anyhow::Result<()> {
    let cli_args = process_cli_args(Cli::parse())?;
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut writer = io::BufWriter::new(lock);

    let queue: ArrayQueue<String> = ArrayQueue::new(cli_args.max_results);

    let matcher = build_matcher(&cli_args.pattern)?;
    let haystack_builder = walk_builder(&cli_args.paths, &cli_args.ignored_paths);
    // PERF: is this tweakable ?
    haystack_builder.build_parallel().run(|| {
        Box::new(|entry: Result<DirEntry, ignore::Error>| match entry {
            Ok(entry) => {
                let file_type = entry.file_type().unwrap();
                if !file_type.is_dir() {
                    let path = entry.path().to_path_buf();
                    match search_file(&path, &matcher) {
                        Ok(file_results) => {
                            if !file_results.is_empty() {
                                let path_str: String = path.to_str().unwrap().to_string();
                                queue.push(path_str).unwrap();
                                //println!("{}", entry.path().to_string_lossy())
                            }
                        }
                        Err(_err) => (),
                    }
                }
                ignore::WalkState::Continue
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                ignore::WalkState::Continue
            }
        })
    });
    queue
        .into_iter()
        .for_each(|path| writeln!(writer, "{}", path).unwrap());

    writer.flush()?;

    Ok(())
}
