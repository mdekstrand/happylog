//! Support for configuring logging verbosity levels.
//!
//! This isn't yet exposed. Eventually it will allow extensive verbosity
//! customization.

use std::{borrow::Cow, cmp::min};

use fern::Dispatch;
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
        VerbosityLevel {
            default: lf.clone(),
            modules: Vec::new(),
        }
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

impl Verbosity {
    fn level(&self) -> &VerbosityLevel {
        let idx = min(self.current, self.levels.len() - 1);
        &self.levels[idx]
    }

    pub fn add_filters(&self, dispatch: Dispatch) -> Dispatch {
        let lvl = self.level();
        let mut dispatch = dispatch.level(lvl.default);
        for (m, l) in lvl.modules.iter() {
            dispatch = dispatch.level_for(m.clone(), l.clone());
        }
        dispatch
    }

    pub fn verbosity(&mut self, verbose: i32) {
        assert!(verbose >= -1);
        self.current = (1 + verbose) as usize;
    }

    pub fn is_quiet(&self) -> bool {
        return self.current == 0;
    }

    #[allow(dead_code)]
    pub fn is_normal(&self) -> bool {
        return self.current == 1;
    }

    #[allow(dead_code)]
    pub fn is_verbose(&self) -> bool {
        return self.current > 1;
    }
}
