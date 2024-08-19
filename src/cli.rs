use std::path::PathBuf;

use crate::{printer::PrintMode, utils};
use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(name = "grip-grab")]
#[command(bin_name = "gg")]
#[command(version, about = "A somewhat faster, more lightweight, ripgrep-inspired alternative.", long_about = None, arg_required_else_help=true)]
#[command(group(ArgGroup::new("pattern_group").args(&["pattern", "patterns"])))]
pub struct Cli {
    /// a regex pattern to search for
    #[arg(num_args = 1, group = "pattern_group")]
    pub pattern: Option<String>,

    /// you can specify multiple patterns using -e "pattern1" -e "pattern2" etc.
    #[arg(short = 'e', long, group = "pattern_group", num_args = 1)]
    patterns: Vec<String>,

    /// path in which to search recursively
    #[arg(num_args = 1, last = true)]
    pub path: Option<PathBuf>,

    /// paths to ignore when recursively walking target directory
    #[clap(short = 'I', long)]
    pub ignore_paths: Vec<PathBuf>,

    /// disregard .gitignore rules when recursively walking directory (defaults to false)
    #[clap(short = 'G', long, default_value_t = false)]
    pub disregard_gitignore: bool,

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

    /// disable colored output (colored by default)
    #[clap(short = 'C', long, default_value_t = false)]
    pub disable_colored_output: bool,

    /// filter on filetype (defaults to all filetypes)
    #[clap(short = 't', long)]
    pub filter_filetypes: Vec<String>,

    /// disable hyperlinks in output (defaults to false)
    #[clap(short = 'H', long, default_value_t = false)]
    pub disable_hyperlinks: bool,
}

#[derive(Debug)]
pub struct PostProcessedCli {
    pub patterns: Vec<String>,
    pub path: PathBuf,
    pub ignored_paths: Vec<PathBuf>,
    pub max_results: usize,
    pub n_threads: usize,
    pub disregard_gitignore: bool,
    pub multiline: bool,
    pub print_mode: PrintMode,
    pub absolute_paths: bool,
    pub colored_output: bool,
    pub filter_filetypes: Vec<String>,
    pub disable_hyperlinks: bool,
}

const DEFAULT_PATH: &str = ".";

pub fn process_cli_args(cli: Cli) -> anyhow::Result<PostProcessedCli> {
    Ok(PostProcessedCli {
        patterns: if !cli.patterns.is_empty() {
            cli.patterns
        } else {
            vec![cli.pattern.unwrap()]
        },
        path: utils::resolve_path(cli.path.unwrap_or(PathBuf::from(DEFAULT_PATH))),
        ignored_paths: utils::resolve_paths(cli.ignore_paths),
        max_results: cli.max_results,
        n_threads: cli.n_threads,
        disregard_gitignore: cli.disregard_gitignore,
        multiline: cli.multiline,
        print_mode: if cli.json {
            PrintMode::Json
        } else if cli.file_paths_only {
            PrintMode::Files
        } else {
            PrintMode::Text
        },
        absolute_paths: cli.absolute_paths,
        colored_output: !cli.disable_colored_output,
        filter_filetypes: cli.filter_filetypes,
        disable_hyperlinks: cli.disable_hyperlinks,
    })
}
