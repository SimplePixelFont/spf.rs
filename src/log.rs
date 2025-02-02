pub(crate) struct Logger {
    message: String,
}

impl Logger {
    pub(crate) fn flush_info(&mut self) {}
}

trait SPFDebug {
    fn as_string(&self) -> String;
}
