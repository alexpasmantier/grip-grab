use std::path::PathBuf;

use crate::{printer::PrintMode, utils};
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(name = "grip-grab")]
#[command(bin_name = "gg")]
#[command(version, about = "A somewhat faster, more lightweight, ripgrep-inspired alternative.", long_about = None, arg_required_else_help=true)]
pub struct Cli {
    /// a regex pattern to search for
    #[arg(num_args = 1)]
    pub pattern: Option<String>,

    /// you can specify multiple patterns using -e "pattern1" -e "pattern2" etc.
    #[arg(
        short = 'e',
        long,
        action = ArgAction::Append
    )]
    patterns: Vec<String>,

    /// path in which to search recursively
    #[arg(num_args = 1)]
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

impl Cli {
    pub fn validate(&mut self) {
        if self.patterns.is_empty() {
            // If no patterns are provided using -e, the positional argument should be treated as a
            // pattern
            if self.pattern.is_none() {
                eprintln!("error: the following required arguments were not provided: <PATTERN>");
                std::process::exit(1);
            }
        } else if self.pattern.is_some() && self.path.is_some() {
            // If patterns are provided using -e, and a pattern is provided as a positional argument
            // as well as a path, we should invalidate the pattern
            eprintln!(
                "error: the argument '[PATTERN]' cannot be used with '--patterns <PATTERNS>'"
            );
            std::process::exit(1);
        } else if self.pattern.is_some() {
            // If patterns are provided using -e, the positional argument (if there is one) should be interpreted as
            // a path
            self.path = self.pattern.take().map(PathBuf::from);
            self.pattern = None;
        } else {
            // If patterns are provided using -e and no positional arguments are provided, use
            // default path
            self.path = Some(PathBuf::from("."));
        }
    }
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

pub fn process_cli_args(mut cli: Cli) -> anyhow::Result<PostProcessedCli> {
    cli.validate();
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
