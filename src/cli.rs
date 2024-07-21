use std::path::PathBuf;

use crate::{printer::PrintMode, utils};
use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(name = "grip-grab")]
#[command(bin_name = "gg")]
#[command(version, about = "A faster, more lightweight, ripgrep alternative.", long_about = None)]
#[command(group(
    ArgGroup::new("pattern_group")
        .args(&["pattern", "patterns"])
        .required(true)
))]
pub struct Cli {
    /// a regex pattern to search for
    #[arg(index = 1, num_args = 0..=1, group = "pattern_group")]
    pub pattern: Option<String>,

    /// you can specify multiple patterns using -e "pattern1" -e "pattern2" etc.
    #[arg(short = 'e', long, group = "pattern_group")]
    patterns: Vec<String>,

    /// path in which to search recursively
    #[arg(index = 2)]
    pub path: PathBuf,

    /// paths to ignore when recursively walking target directory
    #[clap(short = 'I', long)]
    pub ignore_paths: Vec<PathBuf>,

    /// respect .gitignore when recursively walking directory
    #[clap(short = 'G', long, default_value_t = true)]
    pub respect_gitignore: bool,

    /// upper boundary for the number of results to expect (will panic if #results > max_results)
    #[clap(short = 'M', long, default_value_t = 1000)]
    pub max_results: usize,

    /// number of threads to use
    #[clap(short = 'T', long, default_value_t = 4)]
    pub n_threads: usize,

    /// enable multiline matching
    #[clap(short = 'U', long, default_value_t = false)]
    pub multiline: bool,

    /// output in JSON format
    #[clap(long, default_value_t = false)]
    pub json: bool,

    /// output file paths only
    #[clap(short = 'f', long, default_value_t = false)]
    pub file_paths_only: bool,

    /// output absolute paths (defaults to relative)
    #[clap(short = 'A', long, default_value_t = false)]
    pub absolute_paths: bool,

    /// toggle colored output (defaults to ON)
    #[clap(short = 'C', long, default_value_t = true)]
    pub colored_output: bool,
}

#[derive(Debug)]
pub struct PostProcessedCli {
    pub patterns: Vec<String>,
    pub path: PathBuf,
    pub ignored_paths: Vec<PathBuf>,
    pub max_results: usize,
    pub n_threads: usize,
    pub respect_gitignore: bool,
    pub multiline: bool,
    pub print_mode: PrintMode,
    pub absolute_paths: bool,
    pub colored_output: bool,
}

pub fn process_cli_args(cli: Cli) -> anyhow::Result<PostProcessedCli> {
    Ok(PostProcessedCli {
        patterns: if !cli.patterns.is_empty() {
            cli.patterns
        } else {
            vec![cli.pattern.unwrap()]
        },
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
        absolute_paths: cli.absolute_paths,
        colored_output: cli.colored_output,
    })
}
