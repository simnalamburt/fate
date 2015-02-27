use log::Log;
use log::LogLevel;
use log::LogLevelFilter;
use log::LogRecord;
use log::SetLoggerError;
use log::set_logger;

pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _level: LogLevel, _module: &str) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.level(), record.location().module_path) {
            println!("{}: {}", record.level(), record.args());
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(SimpleLogger)
    })
}
