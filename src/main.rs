use std::sync::Arc;

use clap::Parser;

use crossbeam::queue::SegQueue;
use grep::regex::RegexMatcher;
use ignore::DirEntry;
use printer::PrinterConfig;
use search::build_searcher;

use crate::cli::{process_cli_args, Cli};
use crate::fs::walk_builder;
use crate::printer::Printer;
use crate::search::{build_matcher, search_file, FileResults};

mod cli;
mod fs;
mod printer;
mod search;
mod utils;

pub fn main() -> anyhow::Result<()> {
    let cli_args = process_cli_args(Cli::parse())?;
    let queue: Arc<SegQueue<FileResults>> = Arc::new(SegQueue::new());

    let haystack_builder = walk_builder(
        cli_args.paths.iter().map(|p| p.as_path()).collect(),
        &cli_args.ignored_paths,
        cli_args.n_threads,
        !cli_args.disregard_gitignore,
        cli_args.filter_filetypes,
    );
    let matcher: Arc<RegexMatcher> = Arc::new(build_matcher(&cli_args.patterns)?);
    haystack_builder.build_parallel().run(|| {
        let matcher = Arc::clone(&matcher);
        let mut searcher = build_searcher(cli_args.multiline);
        let queue = Arc::clone(&queue);
        Box::new(move |entry: Result<DirEntry, ignore::Error>| match entry {
            Ok(entry) => {
                let file_type = entry.file_type().unwrap();
                if !file_type.is_dir() {
                    let path = entry.path().to_path_buf();
                    match search_file(path, &matcher, &mut searcher) {
                        Ok(file_results) => {
                            if !file_results.is_empty() {
                                queue.push(file_results);
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

    let printer_config = PrinterConfig {
        mode: cli_args.print_mode,
        absolute_paths: cli_args.absolute_paths,
        colored_output: cli_args.colored_output,
        disable_hyperlinks: cli_args.disable_hyperlinks,
        ..Default::default()
    };
    let mut printer = Printer::new(printer_config);
    let printer_queue = Arc::into_inner(queue).unwrap();
    printer_queue
        .into_iter()
        .for_each(|file_results| printer.write(file_results).unwrap());

    printer.print()?;
    Ok(())
}
