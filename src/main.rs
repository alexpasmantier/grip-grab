use int::app::App;
use std::io;
use std::io::{stdin, Read};
use std::path::PathBuf;
use std::sync::Arc;
use syntect::highlighting::ThemeSet;

use app::file_results_to_ui_results;
use clap::Parser;

use crossbeam::queue::SegQueue;
use grep::regex::RegexMatcher;
use ignore::DirEntry;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, Event};
use ratatui::Terminal;
use syntect::parsing::SyntaxSet;

use crate::cli::cli::{process_cli_args, Cli, Commands, PostProcessedCli};
use crate::fs::fs::{is_readable_stdin, walk_builder};
use crate::int::{app, keys::handle_key_event, term, ui::ui};
use crate::printer::printer::{PrinterConfig, ResultsPrinter};
use crate::search::search::{
    build_matcher, build_searcher, search_file, search_reader, FileResults,
};
use crate::upgrade::upgrade::upgrade_gg;

mod cli;
mod fs;
mod int;
mod printer;
mod search;
mod upgrade;
mod utils;

pub fn main() -> anyhow::Result<()> {
    let cli_args = process_cli_args(Cli::parse())?;

    if let Some(subcommand) = cli_args.sub_command {
        match subcommand {
            Commands::Upgrade { force } => {
                upgrade_gg(force);
                return Ok(());
            }
            Commands::Interactive => {
                term::init_panic_hook();
                let theme_set = ThemeSet::load_defaults();
                let theme = &theme_set.themes["base16-eighties.dark"];

                let mut terminal = term::init()?;
                let mut app = app::App::default().preview_theme(theme);
                let _ = run_app(&mut terminal, &mut app, &cli_args)?;
                term::restore()?;
                return Ok(());
            }
        }
    }

    if is_readable_stdin() {
        let stdin = stdin();
        let mut buf = Vec::new();
        if stdin.lock().read_to_end(&mut buf)? != 0 {
            let matcher = build_matcher(&cli_args.patterns)?;
            let mut searcher = build_searcher(cli_args.multiline);
            match search_reader(buf.as_slice(), &matcher, &mut searcher) {
                Ok(search_results) => {
                    let printer_config = PrinterConfig {
                        mode: cli_args.print_mode,
                        absolute_paths: cli_args.absolute_paths,
                        colored_output: cli_args.colored_output,
                        disable_hyperlinks: cli_args.disable_hyperlinks,
                        ..Default::default()
                    };
                    let mut printer = ResultsPrinter::new(printer_config);
                    printer.write(FileResults {
                        path: PathBuf::from("stdin"),
                        results: search_results,
                    })?;
                    printer.print()?;
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
            return Ok(());
        }
    }

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
    let mut printer = ResultsPrinter::new(printer_config);
    let printer_queue = Arc::into_inner(queue).unwrap();
    while !printer_queue.is_empty() {
        let file_results = printer_queue.pop().unwrap();
        printer.write(file_results)?;
    }

    printer.print()?;
    Ok(())
}

fn search(
    target_paths: &[PathBuf],
    pattern: &str,
    cli_args: &PostProcessedCli,
    queue: Arc<SegQueue<FileResults>>,
) -> anyhow::Result<()> {
    let haystack_builder = walk_builder(
        target_paths.iter().map(|p| p.as_path()).collect(),
        &cli_args.ignored_paths,
        cli_args.n_threads,
        !cli_args.disregard_gitignore,
        cli_args.filter_filetypes.clone(),
    );
    let matcher: Arc<RegexMatcher> = Arc::new(build_matcher(&vec![pattern.into()])?);
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
    Ok(())
}

const MIN_SEARCH_PATTERN_LEN: usize = 2;

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    cli_args: &PostProcessedCli,
) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            handle_key_event(key, app);
        }

        if app.should_quit {
            return Ok(true);
        }

        if app.input.value() != app.pattern && app.input.value().len() > MIN_SEARCH_PATTERN_LEN {
            app.results_list.results.clear();
            app.pattern = app.input.value().to_string();
            let target_paths = vec![app.target_path.clone()];
            let arc_queue: Arc<SegQueue<FileResults>> = Arc::new(SegQueue::new());
            let _ = search(
                &target_paths,
                &app.input.value(),
                cli_args,
                Arc::clone(&arc_queue),
            );
            let queue = Arc::into_inner(arc_queue).unwrap();
            while !queue.is_empty() {
                let file_results = queue.pop().unwrap();
                app.results_list
                    .results
                    .append(&mut file_results_to_ui_results(file_results));
            }
            if !app.results_list.results.is_empty() {
                app.results_list.state.select(Some(0));
            } else {
                app.results_list.state.select(None);
            }
        }

        if let Some(selected) = app.results_list.state.selected() {
            let result = app.results_list.results[selected].clone();
            if let Some(last_selected) = &app.selected_result {
                if result.line_number != last_selected.line_number
                    || result.path != last_selected.path
                {
                    if result.path != last_selected.path {
                        app.compute_highlights(&result.path);
                        app.preview_state.file_name =
                            Some(result.path.to_string_lossy().to_string());
                    }
                    app.set_scroll_for_result(&result)
                }
                app.selected_result = Some(result);
            } else {
                app.set_scroll_for_result(&result);
                app.compute_highlights(&result.path);
                app.selected_result = Some(result);
                app.preview_state.file_name = Some(
                    app.selected_result
                        .as_ref()
                        .unwrap()
                        .path
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }
    }
}
