use int::app::App;
use logging::initialize_logging;
use ratatui::prelude::CrosstermBackend;
use std::io::{stdin, Read};
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};
use std::{io, thread};
use syntect::highlighting::ThemeSet;
use tracing::debug;

use app::file_results_to_ui_results;
use clap::Parser;

use crossbeam::queue::SegQueue;
use grep::regex::RegexMatcher;
use ignore::DirEntry;
use ratatui::crossterm::event::{self, poll, Event};

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
mod logging;
mod printer;
mod search;
mod upgrade;
mod utils;

type Terminal = ratatui::Terminal<CrosstermBackend<std::io::Stdout>>;

pub fn main() -> anyhow::Result<()> {
    let cli_args = process_cli_args(Cli::parse())?;

    if let Some(subcommand) = cli_args.sub_command {
        match subcommand {
            Commands::Upgrade { force } => {
                upgrade_gg(force);
                return Ok(());
            }
            Commands::Interactive => {
                initialize_logging()?;
                term::init_panic_hook();
                let theme_set = ThemeSet::load_defaults();
                //let theme = &theme_set.themes["Solarized (dark)"];
                let theme = &theme_set.themes["base16-mocha.dark"];

                //let theme = ThemeSet::get_theme("assets/themes/Catppuccin Mocha.tmTheme")?;
                //let theme = ThemeSet::get_theme("assets/themes/gruvbox-dark.tmTheme")?;

                let mut terminal = term::init()?;
                let mut app = app::App::default().preview_theme(&theme);
                let _ = run_app(&mut terminal, &mut app)?;
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
    cli_args: Option<&PostProcessedCli>,
    queue: Arc<SegQueue<FileResults>>,
) -> anyhow::Result<()> {
    let _cli_args;
    if cli_args.is_none() {
        _cli_args = PostProcessedCli::default();
    } else {
        _cli_args = cli_args.unwrap().clone();
    }
    let haystack_builder = walk_builder(
        target_paths.iter().map(|p| p.as_path()).collect(),
        &_cli_args.ignored_paths,
        _cli_args.n_threads,
        !_cli_args.disregard_gitignore,
        _cli_args.filter_filetypes.clone(),
    );
    let matcher: Arc<RegexMatcher> = Arc::new(build_matcher(&vec![pattern.into()])?);
    haystack_builder.build_parallel().run(|| {
        let matcher = Arc::clone(&matcher);
        let mut searcher = build_searcher(_cli_args.multiline);
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

const MAX_RESULTS_DISPLAY_COUNT: usize = 1000;
const MIN_SEARCH_PATTERN_LEN: usize = 2;
const KEY_REFRESH_RATE: Duration = Duration::from_millis(15);
const SLEEP_TIMEOUT: Duration = Duration::from_millis(2000);
const FRAMES_PER_SECOND: u32 = 60;
const SLEEP_FRAMES_PER_SECOND: u32 = 15;
const MAX_FED_RESULTS_PER_CYCLE: usize = 500;

fn run_app<'a>(terminal: &mut Terminal, app: &mut App) -> io::Result<bool> {
    let mut last_tick = Instant::now();
    let mut running_job_tx: Option<Arc<Mutex<mpsc::Sender<_>>>> = None;
    let mut should_draw = true;
    let mut last_key = (
        event::KeyEvent {
            code: event::KeyCode::Null,
            modifiers: event::KeyModifiers::empty(),
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE,
        },
        Instant::now(),
    );
    let mut frames_per_second = FRAMES_PER_SECOND;
    let mut last_significant_event = Instant::now();
    let mut sleeping = false;
    let mut fed_results;

    loop {
        if poll(Duration::from_millis(0))? {
            last_significant_event = Instant::now();
            sleeping = false;
            frames_per_second = FRAMES_PER_SECOND;
            if let Event::Key(key) = event::read()? {
                if !(key.kind == event::KeyEventKind::Release) {
                    if key == last_key.0 {
                        if last_key.1.elapsed() >= KEY_REFRESH_RATE {
                            last_key = (key, Instant::now());
                            handle_key_event(key, app, terminal);
                            should_draw = true;
                        }
                    } else {
                        last_key = (key, Instant::now());
                        handle_key_event(key, app, terminal);
                        should_draw = true;
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(true);
        }

        // search
        if app.input.value() != app.pattern {
            if app.input.value().len() >= MIN_SEARCH_PATTERN_LEN {
                if let Some(tx) = running_job_tx {
                    debug!("sending stop signal to message passing thread");
                    tx.lock().unwrap().send(()).unwrap();
                }
                let (tx, rx) = mpsc::channel();
                running_job_tx = Some(Arc::new(Mutex::new(tx)));
                app.results_list.results.clear();
                app.results_queue = Arc::new(SegQueue::new());
                let target_paths = vec![app.target_path.clone()];
                let search_results_queue: Arc<SegQueue<FileResults>> = Arc::new(SegQueue::new());
                let srq_search_handle = Arc::clone(&search_results_queue);
                let srq_results_handle = Arc::clone(&search_results_queue);
                let app_results_queue_handle = Arc::clone(&app.results_queue);
                let new_pattern = app.input.value().to_string();
                let _search_handle = {
                    thread::spawn(move || {
                        debug!("search thread started");
                        let _ = search(&target_paths, &new_pattern, None, srq_search_handle);
                        debug!("search thread stopped");
                    })
                };
                let _results_handle = {
                    thread::spawn(move || {
                        debug!("message passing thread started");
                        while rx.try_recv().is_err() {
                            if let Some(file_results) = srq_results_handle.pop() {
                                file_results_to_ui_results(file_results)
                                    .iter()
                                    .for_each(|r| {
                                        app_results_queue_handle.push(r.clone());
                                    });
                            }
                        }
                        debug!("message passing thread stopped");
                    })
                };
            } else {
                app.results_list.results.clear();
                app.results_queue = Arc::new(SegQueue::new());
                app.preview_state = app::PreviewState::default();
                should_draw = true;
            }
            app.pattern = app.input.value().to_string();
        }

        // handle search results
        fed_results = 0;
        while !app.results_queue.is_empty() && fed_results < MAX_FED_RESULTS_PER_CYCLE {
            if app.results_list.results.len() < MAX_RESULTS_DISPLAY_COUNT {
                if let Some(result) = app.results_queue.pop() {
                    app.results_list.results.push(result);
                    fed_results += 1;
                    should_draw = true;
                }
                last_significant_event = Instant::now();
            } else {
                should_draw = true;
                break;
            }
        }

        if app.results_list.results.is_empty() {
            app.results_list.state.select(None);
            should_draw = true;
        } else if app.results_list.state.selected().is_none() {
            app.results_list.state.select_first();
            should_draw = true;
        }

        // preview pane
        if let Some(selected) = app.results_list.state.selected() {
            let result = app.results_list.results[selected].clone();
            if let Some(last_selected) = &app.selected_result {
                if result.line_number != last_selected.line_number
                    || result.path != last_selected.path
                {
                    if result.path != last_selected.path {
                        debug!("computing highlights for new file");
                        app.compute_highlights(&result.path);
                        app.preview_state.file_name =
                            Some(result.path.to_string_lossy().to_string());
                    }
                    app.set_scroll_for_result(&result);
                    should_draw = true;
                }
                app.selected_result = Some(result);
            } else {
                app.set_scroll_for_result(&result);
                debug!("computing highlights for new file");
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
                should_draw = true;
            }
        }
        if should_draw && last_tick.elapsed() >= Duration::from_secs(1) / frames_per_second {
            last_tick = Instant::now();
            terminal.draw(|f| ui(f, app))?;
            should_draw = false;
        }
        if !sleeping && last_significant_event.elapsed() >= SLEEP_TIMEOUT {
            debug!("sleeping");
            frames_per_second = SLEEP_FRAMES_PER_SECOND;
            sleeping = true;
        }
    }
}
