use std::path::PathBuf;

use tui_input::Input;

use crate::cli::cli::DEFAULT_PATH;
use crate::search::search::FileResults;

#[derive(Clone, Copy, PartialEq)]
pub enum CurrentBlock {
    Search,
    Results,
    Preview,
}

static BLOCKS: [CurrentBlock; 3] = [
    CurrentBlock::Search,
    CurrentBlock::Results,
    CurrentBlock::Preview,
];

pub struct Result {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
}

impl Result {
    pub fn to_string(&self) -> String {
        format!("{}:{}:", self.path.to_string_lossy(), self.line_number)
    }
}

pub fn file_results_to_ui_results(file_result: FileResults) -> Vec<Result> {
    let path = file_result.path;
    file_result
        .results
        .iter()
        .map(|r| Result {
            path: path.clone(),
            line_number: r.line_number as usize,
            line: r.line.clone(),
        })
        .collect()
}

pub struct App {
    pub target_path: PathBuf,
    pub pattern: Input,
    pub current_block: CurrentBlock,
    pub should_quit: bool,
    pub results: Vec<Result>,
    pub selected_result: Option<Result>,
}

impl App {
    pub fn new(target_path: PathBuf, current_pattern: String) -> Self {
        Self {
            target_path,
            pattern: Input::new(current_pattern),
            current_block: CurrentBlock::Search,
            should_quit: false,
            results: Vec::new(),
            selected_result: None,
        }
    }

    pub fn next_block(&mut self) {
        let current_index = BLOCKS
            .iter()
            .position(|&block| block == self.current_block)
            .unwrap();
        let next_index = (current_index + 1) % BLOCKS.len();
        self.current_block = BLOCKS[next_index];
    }

    pub fn previous_block(&mut self) {
        let current_index = BLOCKS
            .iter()
            .position(|&b| b == self.current_block)
            .unwrap();
        let previous_index = if current_index == 0 {
            BLOCKS.len() - 1
        } else {
            current_index - 1
        };
        self.current_block = BLOCKS[previous_index];
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            target_path: PathBuf::from(DEFAULT_PATH),
            pattern: Input::new(String::new()),
            current_block: CurrentBlock::Search,
            should_quit: false,
            results: Vec::new(),
            selected_result: None,
        }
    }
}
