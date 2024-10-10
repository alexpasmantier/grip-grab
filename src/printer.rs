use devicons::FileIcon;
use std::{
    env::current_dir,
    fmt,
    io::{IsTerminal, Result, Write},
    path::{Path, PathBuf},
};
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use crate::search::{FileResults, SearchResult};
use std::io::stdout;

#[derive(Debug, Eq, PartialEq)]
pub enum PrintMode {
    Text,
    Json,
    Files,
}

pub struct ResultsPrinter {
    writer: BufferWriter,
    buffer: Buffer,
    config: PrinterConfig,
    cwd: PathBuf,
}

pub struct PrinterConfig {
    pub mode: PrintMode,
    pub colored_output: bool,
    pub color_specs: ColorSpecs,
    pub absolute_paths: bool,
    pub disable_hyperlinks: bool,
    pub disable_devicons: bool,
}

impl Default for PrinterConfig {
    fn default() -> PrinterConfig {
        PrinterConfig {
            mode: PrintMode::Text,
            colored_output: true,
            color_specs: ColorSpecs::default(),
            absolute_paths: false,
            disable_hyperlinks: false,
            disable_devicons: false,
        }
    }
}

pub struct ColorSpecs {
    paths: ColorSpec,
    line_numbers: ColorSpec,
    lines: ColorSpec,
    matched: ColorSpec,
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
        let mut matched: ColorSpec = ColorSpec::new();
        matched.set_fg(Some(Color::Red)).set_bold(true);
        ColorSpecs {
            paths,
            line_numbers,
            lines,
            matched,
        }
    }
}

impl ResultsPrinter {
    pub fn new(mut config: PrinterConfig) -> ResultsPrinter {
        let stdout = stdout();
        if !stdout.is_terminal() {
            config.disable_hyperlinks = true;
            config.colored_output = false;
        }
        let color_choice = if !config.colored_output || config.mode == PrintMode::Json {
            ColorChoice::Never
        } else {
            ColorChoice::Always
        };
        let bufwriter = BufferWriter::stdout(color_choice);
        let buffer = bufwriter.buffer();
        ResultsPrinter {
            writer: bufwriter,
            buffer,
            config,
            cwd: current_dir().unwrap(),
        }
    }

    const MAX_BUFFER_SIZE: usize = 1024;

    pub fn write(&mut self, results: FileResults) -> Result<()> {
        if self.buffer.len() > Self::MAX_BUFFER_SIZE {
            self.buffer.flush()?;
        }
        match self.config.mode {
            PrintMode::Text => self.write_colored_text_results(&results.path, results.results),
            PrintMode::Json => self.writeln_to_buffer(serde_json::to_string(&FileResults {
                path: results.path.clone(),
                results: results.results,
            })?),
            PrintMode::Files => self.write_colored_path(&results.path),
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
        if !self.config.disable_devicons {
            let icon = FileIcon::from(path);
            self.buffer.set_color(ColorSpec::new().set_fg(Some(
                devicons_to_termcolor_color(&icon.color).unwrap_or(Color::White),
            )))?;
            write!(&mut self.buffer, "{} ", icon.icon)?;
        }

        self.buffer.set_color(&self.config.color_specs.paths)?;
        let display_path = if self.config.absolute_paths {
            path.to_string_lossy()
        } else if path.starts_with(&self.cwd) {
            path.strip_prefix(&self.cwd).unwrap().to_string_lossy()
        } else {
            path.to_string_lossy()
        };
        if self.config.disable_hyperlinks {
            return write!(&mut self.buffer, "{}\n", display_path);
        }
        let path_str = path.to_string_lossy();
        let link = Hyperlink {
            uri: &format!("file://{}", path_str),
            id: None,
        };
        write!(&mut self.buffer, "{link}{}{link:#}\n", display_path)
    }

    fn write_colored_search_results(&mut self, results: Vec<SearchResult>) -> Result<()> {
        results.iter().try_for_each(|result| {
            self.write_colored_line(result)?;
            Ok(())
        })
    }

    fn write_colored_line(&mut self, result: &SearchResult) -> Result<()> {
        self.buffer
            .set_color(&self.config.color_specs.line_numbers)?;
        write!(&mut self.buffer, "{}:\t", result.line_number)?;
        self.write_colored_matches(result)
    }

    fn write_colored_matches(&mut self, result: &SearchResult) -> Result<()> {
        self.buffer.set_color(&self.config.color_specs.lines)?;
        let mut last_end_offset = 0;
        result
            .matches
            .iter()
            .try_for_each(|match_range| -> Result<()> {
                write!(
                    &mut self.buffer,
                    "{}",
                    &result.line[last_end_offset..match_range.start]
                )?;
                self.buffer.set_color(&self.config.color_specs.matched)?;
                write!(
                    &mut self.buffer,
                    "{}",
                    &result.line[match_range.start..match_range.end]
                )?;
                self.buffer.set_color(&self.config.color_specs.lines)?;
                last_end_offset = match_range.end;
                Ok(())
            })?;
        write!(&mut self.buffer, "{}", &result.line[last_end_offset..])
    }

    fn writeln_to_buffer(&mut self, text: String) -> Result<()> {
        writeln!(self.buffer, "{}", text)
    }

    fn write_newline_to_buffer(&mut self) -> Result<()> {
        writeln!(self.buffer, "")
    }

    pub fn wipeout(&mut self) -> Result<()> {
        self.buffer.flush()?;
        self.reset_ansi_formatting()
    }

    fn reset_ansi_formatting(&mut self) -> Result<()> {
        self.buffer.reset()?;
        write!(&mut self.buffer, "")?;
        self.writer.print(&self.buffer)
    }
}

fn devicons_to_termcolor_color(d_color: &str) -> Option<Color> {
    d_color.strip_prefix("#").and_then(|hex| {
        u32::from_str_radix(hex, 16)
            .ok()
            .map(|c| Color::Rgb((c >> 16) as u8, (c >> 8) as u8, c as u8))
    })
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Hyperlink<'a> {
    // maybe this should use u8 to support non-utf encodings?
    uri: &'a str,
    id: Option<&'a str>,
}

const OSC8: &str = "\x1b]8";

/// string terminator
const ST: &str = "\x1b\\";

impl fmt::Display for Hyperlink<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url = self.uri;
        if f.alternate() {
            // based off of the cargo internal hyperlink behavior.
            // if the alternate flag is specified, end the hyperlink.
            write!(f, "{OSC8};;{ST}")
        } else if let Some(id) = self.id {
            write!(f, "{OSC8};id={id};{url}{ST}")
        } else {
            write!(f, "{OSC8};;{url}{ST}")
        }
    }
}
