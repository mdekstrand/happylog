//! Easy CLI progress-enabled logging support.
use std::io::stderr;

use fern::{Dispatch, Output};
use indicatif::MultiProgress;
use log::SetLoggerError;
use verbosity::Verbosity;
mod progress;

pub mod util;
mod format;
mod verbosity;

#[cfg(feature="structopt")]
pub mod args;
#[cfg(feature="structopt")]
pub use args::LogOpts;

pub use progress::add_progress;
pub use progress::new_progress;
pub use progress::new_spinner;

pub fn initialize(verbose: i32) -> Result<(), SetLoggerError> {
  let mut verb = Verbosity::default();
  verb.verbosity(verbose);
  init_from_verbosity(verb)
}

fn init_from_verbosity(verbose: Verbosity) -> Result<(), SetLoggerError> {
  let mp = MultiProgress::new();
  progress::initialize(mp.clone());
  let setup = Dispatch::new();
  let setup = verbose.add_filters(setup);
  let setup = if mp.is_hidden() {
    setup.format(format::format_plain).chain(stderr())
  } else {
    let out = Output::call(progress::emit_record);
    setup.format(format::format_color).chain(out)
  };

  setup.apply()
}
