use std::io::{self, Result, StdoutLock, Write};

use crate::search::FileResults;

#[derive(Debug)]
pub enum PrintMode {
    Text,
    Json,
    Files,
}

pub struct Printer {
    writer: io::BufWriter<StdoutLock<'static>>,
    mode: PrintMode,
}

impl Printer {
    pub fn new(mode: PrintMode) -> Printer {
        let stdout = io::stdout();
        let lock = stdout.lock();
        Printer {
            writer: io::BufWriter::new(lock),
            mode,
        }
    }

    pub fn write(&mut self, results: FileResults) -> Result<()> {
        self.writeln_to_buffer(match self.mode {
            PrintMode::Text => format!("{}", results),
            PrintMode::Json => serde_json::to_string(&results)?,
            PrintMode::Files => format!("{}", results.path.to_string_lossy()),
        })
    }

    fn writeln_to_buffer(&mut self, text: String) -> Result<()> {
        writeln!(self.writer, "{}", text)
    }

    pub fn print(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
