# micro-log
Logging environment for no_std based Rust projects

Highly derived from [rust-simple_logger](https://github.com/borntyping/rust-simple_logger)

## Basic Usgae

```
#[macro_use] extern crate log;
# extern crate micro_log;

fn main() {
    // allow log level of Info and more severe
    micro_log::init_with_level(log::Level::Info).unwrap();

    debug!("Logs a message at the debug level.");
    error!("Logs a message at the error level.");
    info!("Logs a message at the info level.");
    trace!("Logs a message at the trace level.");
    warn!("Logs a message at the warn level.");
    
    // alternatively, we could acheive the same results above with
    log!(Level::Debug, "Logs a message at the debug level.");
    log!(Level::Error, "Logs a message at the error level.");
    log!(Level::Info, "Logs a message at the info level.");
    log!(Level::Trace, "Logs a message at the trace level.");
    log!(Level::Warn, "Logs a message at the warn level.");
}
```
