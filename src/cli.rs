use std::path::PathBuf;

use crate::utils;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(arg_required_else_help=true, version, about, long_about = None)]
pub struct Cli {
    /// pattern to search for
    pub pattern: String,

    /// paths in which to recursively search for dead files
    pub paths: Vec<PathBuf>,

    /// paths to ignore when searching for dead files
    #[clap(short = 'I', long)]
    pub ignore_paths: Vec<PathBuf>,

    /// max results
    #[clap(short = 'M', long, default_value_t = 1000)]
    pub max_results: usize,
}

#[derive(Debug)]
pub struct PostProcessedCli {
    pub pattern: String,
    pub paths: Vec<PathBuf>,
    pub ignored_paths: Vec<PathBuf>,
    pub max_results: usize,
}

pub fn process_cli_args(cli: Cli) -> anyhow::Result<PostProcessedCli> {
    Ok(PostProcessedCli {
        pattern: cli.pattern,
        paths: utils::resolve_paths(cli.paths),
        ignored_paths: utils::resolve_paths(cli.ignore_paths),
        max_results: cli.max_results,
    })
}
