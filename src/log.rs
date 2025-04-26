//! Logging utilities for `spf.rs` for debugging.
//!
//! <div class="warning">
//!
//! This module uses a mutable static variable `LOGGER` to facilitate logging within the crate.
//! As such this module is not thread safe and uses `unsafe` code to access the global variable.
//! This module is simply provided as a convenience for debugging and should not be used in production
//! code.
//!
//! </div>
//!
//! The [`core`] module optionally uses this module to log information about the process of
//! converting a [`super::core::Layout`] struct into a [`Vec<u8>`] and vice versa when using the
//! [`super::core::layout_to_data`] and [`super::core::layout_from_data`] functions respectivy.
//!
//! This module uses the [`termcolor`] dependency in order to print more easily readable messages on the
//! console.

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

#[allow(dead_code)]
pub(crate) struct Logger {
    pub(crate) message: String,
    pub(crate) ranges: HashMap<Range<usize>, Color>,
    pub(crate) buffer_writer: BufferWriter,
    pub(crate) buffer: Buffer,
    pub(crate) log_level: LogLevel,
}

#[allow(dead_code)]
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
    Mutex::new(Logger {
        message: String::new(),
        ranges: HashMap::new(),
        buffer_writer,
        buffer,
        log_level: LogLevel::None,
    })
});

#[allow(non_snake_case)]
#[allow(static_mut_refs)]
/// Sets the `log_level` of the global static `LOGGER` variable.
pub fn LOGGER_set_log_level(level: LogLevel) {
    unsafe {
        let mut logger = LOGGER.lock().unwrap();
        logger.log_level = level;
    }
}
