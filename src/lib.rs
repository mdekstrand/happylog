//! Easy CLI progress-enabled logging support.
use std::io::stderr;

use fern::{Dispatch, Output};
use indicatif::MultiProgress;
use indicatif::ProgressDrawTarget;
use log::SetLoggerError;
use verbosity::Verbosity;
mod progress;

pub mod util;
mod format;
mod verbosity;

#[cfg(feature="structopt")]
#[path = "structopt_args.rs"]
pub mod structopt;

#[cfg(feature="structopt")]
pub use structopt::LogOpts;

#[cfg(feature="clap")]
#[path = "clap_args.rs"]
pub mod clap;

#[cfg(feature="clap")]
pub use clap::LogOpts;

pub use fern;
pub use indicatif;
pub use progress::add_progress;
pub use progress::new_progress;
pub use progress::new_spinner;

pub fn initialize(verbose: i32) -> Result<(), SetLoggerError> {
  let mut verb = Verbosity::default();
  verb.verbosity(verbose);
  init_from_verbosity(verb)
}

fn init_from_verbosity(verbose: Verbosity) -> Result<(), SetLoggerError> {
  let mp = if verbose.is_quiet() {
    MultiProgress::with_draw_target(ProgressDrawTarget::hidden())
  } else {
    MultiProgress::with_draw_target(ProgressDrawTarget::stderr())
  };
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
