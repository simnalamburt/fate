use log::{set_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        use std::io::Write;

        std::io::stdout().flush().unwrap();
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    set_logger(&SimpleLogger)?;
    set_max_level(LevelFilter::Info);
    Ok(())
}
