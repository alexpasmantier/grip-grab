use clap::Parser;

use crossbeam::queue::ArrayQueue;
use ignore::DirEntry;
use printer::PrinterConfig;

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
    let queue: ArrayQueue<FileResults> = ArrayQueue::new(cli_args.max_results);

    let matcher = build_matcher(&cli_args.patterns)?;
    let haystack_builder = walk_builder(
        &cli_args.path,
        &cli_args.ignored_paths,
        cli_args.n_threads,
        cli_args.respect_gitignore,
        cli_args.filter_filetypes,
    );
    haystack_builder.build_parallel().run(|| {
        Box::new(|entry: Result<DirEntry, ignore::Error>| match entry {
            Ok(entry) => {
                let file_type = entry.file_type().unwrap();
                if !file_type.is_dir() {
                    let path = entry.path().to_path_buf();
                    match search_file(path, &matcher, cli_args.multiline) {
                        Ok(file_results) => {
                            if !file_results.is_empty() {
                                queue.push(file_results).unwrap();
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
        ..Default::default()
    };
    let mut printer = Printer::new(printer_config);
    queue
        .into_iter()
        .for_each(|file_results| printer.write(file_results).unwrap());

    printer.print()?;

    Ok(())
}
