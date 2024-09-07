use std::collections::HashMap;
use std::path::PathBuf;
use std::u16;

use ratatui::widgets::ListState;
use syntect::highlighting::Style;

use crate::cli::cli::DEFAULT_PATH;
use crate::int::input::Input;
use crate::search::search::{FileResults, MatchRange, SearchResult};

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

#[derive(Clone)]
pub struct Result {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
    pub matches: Vec<MatchRange>,
}

pub fn file_results_to_ui_results(file_result: FileResults) -> Vec<Result> {
    let path = file_result.path;
    file_result
        .results
        .iter()
        .map(|r: &SearchResult| Result {
            path: path.clone(),
            line_number: r.line_number as usize,
            line: r.line.clone(),
            matches: r.matches.clone(),
        })
        .collect()
}

pub struct ResultsList {
    pub results: Vec<Result>,
    pub state: ListState,
}

impl Default for ResultsList {
    fn default() -> Self {
        Self {
            results: Vec::new(),
            state: ListState::default(),
        }
    }
}

#[derive(Clone)]
pub struct PreviewState {
    pub scroll: (u16, u16),
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub highlighted_lines: Vec<Vec<(Style, String)>>,
}

pub struct App {
    pub target_path: PathBuf,
    pub input: Input,
    pub pattern: String,
    pub current_block: CurrentBlock,
    pub should_quit: bool,
    pub results_list: ResultsList,
    pub preview_state: PreviewState,
    pub preview_cache: HashMap<PathBuf, Vec<Vec<(Style, String)>>>,
    pub preview_pane_height: u16,
    pub selected_result: Option<Result>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            target_path: PathBuf::from(DEFAULT_PATH),
            input: Input::new(String::new()),
            pattern: String::new(),
            current_block: CurrentBlock::Search,
            should_quit: false,
            results_list: ResultsList::default(),
            preview_state: PreviewState {
                scroll: (0, 0),
                file_name: None,
                file_type: None,
                highlighted_lines: Vec::new(),
            },
            preview_cache: HashMap::new(),
            preview_pane_height: 0,
            selected_result: None,
        }
    }
}

impl App {
    fn get_current_block_index(&self) -> usize {
        BLOCKS
            .iter()
            .position(|&block| block == self.current_block)
            .unwrap()
    }

    pub fn next_block(&mut self) {
        let current_index = self.get_current_block_index();
        let next_index = (current_index + 1) % BLOCKS.len();
        self.current_block = BLOCKS[next_index];
    }

    pub fn previous_block(&mut self) {
        let current_index = self.get_current_block_index();
        let previous_index = if current_index == 0 {
            BLOCKS.len() - 1
        } else {
            current_index - 1
        };
        self.current_block = BLOCKS[previous_index];
    }

    /// ┌───────────────────┐┌─────────────┐
    /// │ Results           ││ Preview     │
    /// │                   ││             │
    /// │                   ││             │
    /// │                   ││             │
    /// └───────────────────┘│             │
    /// ┌───────────────────┐│             │
    /// │ Search          x ││             │
    /// └───────────────────┘└─────────────┘
    pub fn move_to_block_on_top(&mut self) {
        match self.current_block {
            CurrentBlock::Search => {
                self.current_block = CurrentBlock::Results;
            }
            _ => {}
        }
    }

    /// ┌───────────────────┐┌─────────────┐
    /// │ Results         x ││ Preview     │
    /// │                   ││             │
    /// │                   ││             │
    /// │                   ││             │
    /// └───────────────────┘│             │
    /// ┌───────────────────┐│             │
    /// │ Search            ││             │
    /// └───────────────────┘└─────────────┘
    pub fn move_to_block_below(&mut self) {
        match self.current_block {
            CurrentBlock::Results => {
                self.current_block = CurrentBlock::Search;
            }
            _ => {}
        }
    }

    /// ┌───────────────────┐┌─────────────┐
    /// │ Results         x ││ Preview     │
    /// │                   ││             │
    /// │                   ││             │
    /// │                   ││             │
    /// └───────────────────┘│             │
    /// ┌───────────────────┐│             │
    /// │ Search          x ││             │
    /// └───────────────────┘└─────────────┘
    pub fn move_to_block_right(&mut self) {
        match self.current_block {
            CurrentBlock::Results | CurrentBlock::Search => {
                self.current_block = CurrentBlock::Preview;
            }
            _ => {}
        }
    }

    /// ┌───────────────────┐┌─────────────┐
    /// │ Results           ││ Preview   x │
    /// │                   ││             │
    /// │                   ││             │
    /// │                   ││             │
    /// └───────────────────┘│             │
    /// ┌───────────────────┐│             │
    /// │ Search            ││             │
    /// └───────────────────┘└─────────────┘
    pub fn move_to_block_left(&mut self) {
        match self.current_block {
            CurrentBlock::Preview => {
                self.current_block = CurrentBlock::Results;
            }
            _ => {}
        }
    }

    pub fn compute_highlights_for_selected(&mut self) {
        todo!()
    }

    pub fn scroll_preview_down(&mut self, offset: u16) {
        self.preview_state.scroll.0 = self.preview_state.scroll.0.saturating_add(offset).min(
            (self.preview_state.highlighted_lines.len() as isize
                - (2 * self.preview_pane_height / 3) as isize)
                .max(0) as u16,
        );
    }

    // TODO: same thing as above
    pub fn scroll_preview_up(&mut self, offset: u16) {
        self.preview_state.scroll.0 = self.preview_state.scroll.0.saturating_sub(offset);
    }

    pub fn reset_preview_scroll(&mut self) {
        self.preview_state.scroll = (0, 0);
    }
}
