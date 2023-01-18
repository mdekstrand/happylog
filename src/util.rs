//! Utility functions for working with loggers
use log::LevelFilter;

/// Make a log level more verbose by one step
pub fn verbosify(f: LevelFilter) -> LevelFilter {
    match f {
        LevelFilter::Error => LevelFilter::Warn,
        LevelFilter::Warn => LevelFilter::Info,
        LevelFilter::Info => LevelFilter::Debug,
        LevelFilter::Debug => LevelFilter::Trace,
        x => x,
    }
}
