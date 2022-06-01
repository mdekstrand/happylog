//! Logging targets.
use std::io::Write;
use std::io;

use indicatif::MultiProgress;
use indicatif::ProgressBar;

/// Trait implented by viable log targets.
pub trait LogTarget {
  fn write_msg(&self, msg: &str) -> io::Result<()>;
}

impl LogTarget for io::Stderr {
  fn write_msg(&self, msg: &str) -> io::Result<()> {
    let mut lock = self.lock();
    writeln!(lock, "{}", msg)
  }
}

impl LogTarget for ProgressBar {
  fn write_msg(&self, msg: &str) -> io::Result<()> {
    self.println(msg);
    Ok(())
  }
}

impl LogTarget for MultiProgress {
  fn write_msg(&self, msg: &str) -> io::Result<()> {
    if self.is_hidden() {
      io::stderr().write_msg(msg)
    } else {
      self.println(msg)
    }
  }
}
