mod console_emitter;

/// Utility functions for working with loggers
pub mod util;

/// Support for command-line argument configuration of the logger.
#[cfg(feature="structopt")]
pub mod args;

pub use console_emitter::initialize;
pub use console_emitter::set_progress;
