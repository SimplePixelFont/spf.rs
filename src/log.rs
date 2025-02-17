use std::collections::HashMap;
use std::io::{self, Write};
use std::ops::Range;
use std::sync::LazyLock;
use std::sync::Mutex;
use termcolor::WriteColor;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec};

#[derive(Copy, Clone)]
pub enum LogLevel {
    None = 0,
    Info = 1,
    Debug = 2,
}

pub(crate) struct Logger {
    pub(crate) message: String,
    pub(crate) ranges: HashMap<Range<usize>, Color>,
    pub(crate) buffer_writer: BufferWriter,
    pub(crate) buffer: Buffer,
    pub(crate) log_level: LogLevel,
}

impl Logger {
    pub(crate) fn flush_info(&mut self) -> io::Result<()> {
        self.buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;

        write!(&mut self.buffer, "[ Info: ")?;
        self.buffer.reset()?;

        writeln!(&mut self.buffer, "{}", self.message)?;

        self.buffer_writer.print(&self.buffer)?;

        self.message.clear();
        self.buffer.clear();

        Ok(())
    }
    pub(crate) fn flush_debug(&mut self) -> io::Result<()> {
        self.buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;

        write!(&mut self.buffer, "[ Debug: ")?;
        self.buffer.reset()?;

        writeln!(&mut self.buffer, "{}", self.message)?;

        self.buffer_writer.print(&self.buffer)?;

        self.message.clear();
        self.buffer.clear();

        Ok(())
    }
}

pub(crate) static mut LOGGER: LazyLock<Mutex<Logger>> = LazyLock::new(|| {
    let buffer_writer = BufferWriter::stdout(ColorChoice::Always);
    let buffer = buffer_writer.buffer();
    return Mutex::new(Logger {
        message: String::new(),
        ranges: HashMap::new(),
        buffer_writer: buffer_writer,
        buffer: buffer,
        log_level: LogLevel::None,
    });
});

pub fn set_logger_level(level: LogLevel) {
    unsafe {
        let mut logger = LOGGER.lock().unwrap();
        logger.log_level = level;
    }
}
