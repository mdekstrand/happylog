//! Support for configuring logging verbosity levels.

use std::borrow::Cow;

use log::LevelFilter;

static DEFAULT_SEQUENCE: &[LevelFilter] = &[
  LevelFilter::Warn,
  LevelFilter::Info,
  LevelFilter::Debug,
  LevelFilter::Trace,
];

/// A single verbosity level.
struct VerbosityLevel {
  default: LevelFilter,
  modules: Vec<(Cow<'static, str>, LevelFilter)>,
}

impl From<&LevelFilter> for VerbosityLevel {
  fn from(lf: &LevelFilter) -> Self {
    VerbosityLevel { default: lf.clone(), modules: Vec::new() }
  }
}

/// Representation of the current verbosity on a sequence of levels.
pub struct Verbosity {
  levels: Vec<VerbosityLevel>,
  current: usize,
}

impl Default for Verbosity {
  fn default() -> Self {
    Verbosity {
      levels: DEFAULT_SEQUENCE.iter().map(|f| f.into()).collect(),
      current: 1,
    }
  }
}
