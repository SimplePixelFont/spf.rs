use std::collections::HashMap;
use std::io::{self, Write};
use std::ops::Range;
use std::sync::LazyLock;
use std::sync::Mutex;
use termcolor::WriteColor;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec};

pub(crate) struct Logger {
    pub(crate) message: String,
    pub(crate) ranges: HashMap<Range<usize>, Color>,
    pub(crate) buffer_writer: BufferWriter,
    pub(crate) buffer: Buffer,
}

impl Logger {
    pub(crate) fn flush_info(&mut self) -> io::Result<()> {
        self.buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;

        write!(&mut self.buffer, "[ Info: ")?;
        self.buffer.reset()?;

        writeln!(&mut self.buffer, "{}", self.message)?;

        self.buffer_writer.print(&self.buffer)
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
    });
});
