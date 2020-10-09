use indicatif::ProgressBar;
use log::*;

use std::io;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::marker::PhantomData;

static LOG_OUT: AtomicPtr<Target> = AtomicPtr::new(ptr::null_mut());

#[derive(Debug, Clone)]
enum Target {
  Stderr,
  PB(ProgressBar)
}

fn get_target() -> Target {
  let out = LOG_OUT.load(Ordering::Relaxed);
  let rout = if out.is_null() {
    &Target::Stderr
  } else {
    unsafe { &*out }
  };
  rout.clone()
}

/// Trait for printing log lines
pub trait Println {
  fn println(&self, text: &str) -> io::Result<()>;
}

/// Progress bar logging context
pub struct LogPBState<'a> {
  phantom: PhantomData<&'a str>,
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

pub fn set_progress<'a>(pb: &'a ProgressBar) -> LogPBState<'a> {
  let pbb = Box::new(Target::PB(pb.clone()));
  let prev = get_target();
  LOG_OUT.store(Box::leak(pbb), Ordering::Relaxed);
  LogPBState {
    phantom: PhantomData,
    previous: prev
  }
}

impl <'a> Drop for LogPBState<'a> {
  fn drop(&mut self) {
    let pbox = Box::new(self.previous.clone());
    LOG_OUT.store(Box::leak(pbox), Ordering::Relaxed);
  }
}
