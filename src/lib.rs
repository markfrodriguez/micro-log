#![no_std]
extern crate log;
extern crate libc_print;

// expose macro for "standard" println!
use libc_print::std_name::*;
use log::{
    Log,
    Level,
    Metadata,
    Record,
    SetLoggerError
};

struct MicroLog {
    level: Level,
}

static LOGGER: MicroLog = MicroLog { level: Level::Trace };

impl Log for MicroLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        // nothing to do here
    }
}

/// Initializes the global logger with a MicroLog instance with
/// `max_log_level` set to a specific log level.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate micro_log;
/// #
/// # fn main() {
/// micro_log::init_with_level(log::Level::Warn).unwrap();
///
/// warn!("This is an example message.");
/// info!("This message will not be logged.");
/// # }
/// ```
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level.to_level_filter()))
}

/// Initializes the global logger with a MicoLog instance with
/// `max_log_level` set to `LogLevel::Trace`.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate micro_log;
/// #
/// # fn main() {
/// micro_log::init().unwrap();
/// warn!("This is an example message.");
/// # }
/// ```
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_log() {
        init();
        log::info!("hello, world!");
        assert!(true);
    }
}
