use std::{
    env::current_dir,
    io::{Result, Write},
    path::{Path, PathBuf},
};
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use crate::search::{FileResults, SearchResult};

#[derive(Debug, Eq, PartialEq)]
pub enum PrintMode {
    Text,
    Json,
    Files,
}

pub struct Printer {
    writer: BufferWriter,
    buffer: Buffer,
    config: PrinterConfig,
    cwd: PathBuf,
}

pub struct PrinterConfig {
    pub mode: PrintMode,
    // TODO: refactorize this
    pub colored_output: bool,
    pub color_specs: ColorSpecs,
    pub absolute_paths: bool,
}

impl Default for PrinterConfig {
    fn default() -> PrinterConfig {
        PrinterConfig {
            mode: PrintMode::Text,
            colored_output: true,
            color_specs: ColorSpecs::default(),
            absolute_paths: false,
        }
    }
}

pub struct ColorSpecs {
    paths: ColorSpec,
    line_numbers: ColorSpec,
    lines: ColorSpec,
}

impl Default for ColorSpecs {
    fn default() -> ColorSpecs {
        let mut paths: ColorSpec = ColorSpec::new();
        paths
            .set_fg(Some(Color::Green))
            .set_italic(true)
            .set_bold(true)
            .set_underline(true);
        let mut line_numbers: ColorSpec = ColorSpec::new();
        line_numbers.set_fg(Some(Color::Yellow)).set_bold(true);
        let mut lines: ColorSpec = ColorSpec::new();
        lines.set_fg(Some(Color::White));
        ColorSpecs {
            paths,
            line_numbers,
            lines,
        }
    }
}

impl Printer {
    pub fn new(config: PrinterConfig) -> Printer {
        let color_choice = if !config.colored_output || config.mode == PrintMode::Json {
            ColorChoice::Never
        } else {
            ColorChoice::Always
        };
        let bufwriter = BufferWriter::stdout(color_choice);
        let buffer = bufwriter.buffer();
        Printer {
            writer: bufwriter,
            buffer,
            config,
            cwd: current_dir().unwrap(),
        }
    }

    pub fn write(&mut self, results: FileResults) -> Result<()> {
        let path: &Path;
        if self.config.absolute_paths {
            path = &results.path;
        } else {
            path = results.path.strip_prefix(self.cwd.clone()).unwrap();
        }
        match self.config.mode {
            PrintMode::Text => self.write_colored_text_results(path, results.results),
            PrintMode::Json => self.writeln_to_buffer(serde_json::to_string(&results)?),
            PrintMode::Files => self.write_colored_path(path),
        }
    }

    fn write_colored_text_results(
        &mut self,
        path: &Path,
        search_results: Vec<SearchResult>,
    ) -> Result<()> {
        self.write_colored_path(path)?;
        self.write_colored_search_results(search_results)?;
        self.write_newline_to_buffer()
    }

    fn write_colored_path(&mut self, path: &Path) -> Result<()> {
        self.buffer.set_color(&self.config.color_specs.paths)?;
        writeln!(&mut self.buffer, "{}", path.to_string_lossy())
    }

    fn write_colored_search_results(&mut self, results: Vec<SearchResult>) -> Result<()> {
        results.iter().try_for_each(|result| {
            self.buffer
                .set_color(&self.config.color_specs.line_numbers)?;
            write!(&mut self.buffer, "{}:\t", result.line_number)?;
            self.buffer.set_color(&self.config.color_specs.lines)?;
            write!(&mut self.buffer, "{}", result.line)
        })
    }

    fn writeln_to_buffer(&mut self, text: String) -> Result<()> {
        writeln!(self.buffer, "{}", text)
    }

    fn write_newline_to_buffer(&mut self) -> Result<()> {
        writeln!(self.buffer, "")
    }

    pub fn print(&mut self) -> Result<()> {
        self.writer.print(&self.buffer)?;
        self.reset_ansi_formatting()
    }

    fn reset_ansi_formatting(&mut self) -> Result<()> {
        self.buffer.reset()?;
        self.write_newline_to_buffer()
    }
}
