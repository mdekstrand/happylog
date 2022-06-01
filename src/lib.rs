//! Easy CLI progress-enabled logging support.
mod console_emitter;

pub mod util;
pub mod target;

#[cfg(feature="structopt")]
pub mod args;

pub use console_emitter::initialize;
pub use console_emitter::push_target;
pub use console_emitter::LogState;
