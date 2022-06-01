use indicatif::{ProgressBar, MultiProgress};
use log::*;

use std::io;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

use crate::target::LogTarget;

static LOG_OUT: AtomicPtr<Target> = AtomicPtr::new(ptr::null_mut());


fn get_target() -> impl LogTarget {
  let out = LOG_OUT.load(Ordering::Relaxed);
  let rout = if out.is_null() {
    io::stderr()
  } else {
    unsafe { &*out }
  };
  rout.clone()
}

/// Trait for printing log lines
pub trait Println {
  fn println(&self, text: &str) -> io::Result<()>;
}

/// Context for managing the scope of a progress bar logging target.
pub struct LogState {
  previous: Target
}

struct LogEnv {
  level: LevelFilter,
  out: &'static AtomicPtr<Target>
}

impl Log for LogEnv {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= self.level
  }

  fn log(&self, record: &Record) {
    let pass = record.level() <= self.level;
    if pass {
      let out = get_target();
      let msg = format!("[{:>5}] {}", record.level(), record.args());
      match out {
        Target::Stderr => {
          eprintln!("{}", msg);
        },
        Target::PB(ref prog) => {
          prog.println(msg);
        },
        Target::MPB(ref mpb) => {
          mpb.println(msg);
        }
      }
    }
  }

  fn flush(&self) {}
}

/// Initialize the logging system with a specified log level.
pub fn initialize(level: LevelFilter) -> Result<(), SetLoggerError> {
  LOG_OUT.store(Box::leak(Box::new(Target::Stderr)), Ordering::Relaxed);
  let logger = LogEnv {
    level: level,
    out: &LOG_OUT
  };
  set_boxed_logger(Box::new(logger))?;
  set_max_level(level);
  Ok(())
}

/// Set a target such as a progress bar, overriding the previou target.
/// 
/// This is used to support things such as redirecting logging output to an
/// [indicatif] progress bar to coordinate logging and progress output. This
/// function sets up the logging system to write to the specified target.  It
/// returns an object that, when dropped, unsets the saved progress bar target.
/// 
/// Example:
/// 
/// ```
/// let _pbs = happylog::push_target(&pb);
/// # lots of operations
/// pb.finish_and_clear()
/// # let _pbs go out of scope
/// ```
pub fn push_target<T: Into<Target>>(tgt: T) -> LogState {
  let pbb = Box::new(tgt.into());
  let prev = get_target();
  LOG_OUT.store(Box::leak(pbb), Ordering::Relaxed);
  LogState {
    previous: prev
  }
}

impl Drop for LogState {
  fn drop(&mut self) {
    let pbox = Box::new(self.previous.clone());
    LOG_OUT.store(Box::leak(pbox), Ordering::Relaxed);
  }
}
