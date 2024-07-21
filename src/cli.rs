use std::path::PathBuf;

use crate::{printer::PrintMode, utils};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "git")]
#[command(arg_required_else_help=true, version, about = "A grep-like utility", long_about = None)]
pub struct Cli {
    /// pattern to search for
    pub pattern: String,

    /// path in which to search recursively
    pub path: PathBuf,

    /// paths to ignore when walking directory
    #[clap(short = 'I', long)]
    pub ignore_paths: Vec<PathBuf>,

    /// respect .gitignore when walking directory
    #[clap(short = 'G', long, default_value_t = true)]
    pub respect_gitignore: bool,

    /// upper boundary for the number of results to expect (will panic if #results > max_results)
    #[clap(short = 'M', long, default_value_t = 1000)]
    pub max_results: usize,

    /// number of threads to use
    #[clap(short = 'T', long, default_value_t = 4)]
    pub n_threads: usize,

    /// enable multiline matching (off by default)
    #[clap(short = 'U', long, default_value_t = false)]
    pub multiline: bool,

    /// output in JSON format
    #[clap(long, default_value_t = false)]
    pub json: bool,

    /// output file paths only
    #[clap(short = 'f', long, default_value_t = false)]
    pub file_paths_only: bool,
}

#[derive(Debug)]
pub struct PostProcessedCli {
    pub pattern: String,
    pub path: PathBuf,
    pub ignored_paths: Vec<PathBuf>,
    pub max_results: usize,
    pub n_threads: usize,
    pub respect_gitignore: bool,
    pub multiline: bool,
    pub print_mode: PrintMode,
}

pub fn process_cli_args(cli: Cli) -> anyhow::Result<PostProcessedCli> {
    Ok(PostProcessedCli {
        pattern: cli.pattern,
        path: utils::resolve_path(cli.path),
        ignored_paths: utils::resolve_paths(cli.ignore_paths),
        max_results: cli.max_results,
        n_threads: cli.n_threads,
        respect_gitignore: cli.respect_gitignore,
        multiline: cli.multiline,
        print_mode: if cli.json {
            PrintMode::Json
        } else if cli.file_paths_only {
            PrintMode::Files
        } else {
            PrintMode::Text
        },
    })
}
