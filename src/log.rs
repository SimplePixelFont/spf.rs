use std::collections::HashMap;
use std::ops::Range;
use std::sync::LazyLock;
use termcolor::Color;

pub(crate) struct Logger {
    message: String,
    ranges: LazyLock<HashMap<Range<usize>, Color>>,
}

impl Logger {
    pub(crate) fn flush_info(&mut self) {}
}

pub(crate) static mut LOGGER: Logger = Logger {
    message: String::new(),
    ranges: LazyLock::new(|| HashMap::new()),
};
