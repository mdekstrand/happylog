use indicatif::{MultiProgress, ProgressBar};
use log::Record;

use std::ptr::{null_mut};
use std::sync::atomic::{AtomicPtr, Ordering};

static mut BACKEND: AtomicPtr<MultiProgress> = AtomicPtr::new(null_mut());

pub(crate) fn initialize(pb: MultiProgress) {
  let pbb = Box::new(pb);
  let pbp = Box::leak(pbb);
  let prev = unsafe {
    BACKEND.swap(pbp, Ordering::Relaxed)
  };
  if !prev.is_null() {
    // oops, we already initialized!
    panic!("happylog initialized twice");
  }
}

/// Get a MultiProgress, making sure we have one.
#[allow(dead_code)]
fn ensure_mp() -> MultiProgress {
  unsafe {
    // load one if we have one
    let mut mpp = BACKEND.load(Ordering::Relaxed);
    // if we do not, try to set one up
    if mpp.is_null() {
      let boxed = Box::new(MultiProgress::new());
      let ptr = Box::leak(boxed);
        
      let res = BACKEND.compare_exchange(mpp, ptr, Ordering::Relaxed, Ordering::Relaxed);
      match res {
        Ok(_old) => {
          // we successfully set the progress
          mpp = ptr;
        },
        Err(_) => {
          // we did not - someone beat us to a progress
          // so we can load it! (we never clear progress)
          let _ = Box::from(ptr);  // to drop the unused fresh multi-progress
          mpp = BACKEND.load(Ordering::Relaxed);
        }
      }
    }
    // now the MP pointer is not null, by one of 3 means:
    // - it was non-null initially
    // - it was non-null, but we created a new one and stored it
    // - it was non-null, and someone beat us to creating a new one, and we loaded theirs
    (*mpp).clone()
  }
}

pub(crate) fn emit_record(record: &Record<'_>) {
  let line = format!("{}", record.args());
  let write = unsafe {
    BACKEND.load(Ordering::Relaxed).as_ref().expect("progress not initialized")
  };
  write.println(line).expect("error writing to log backend")
}

/// Register a progress bar with the logger. 
/// 
/// This is used for code that constructs the progress bar itself; it adds it to HappyLog's
/// multi-progress so output can be coordinated with the logger.
pub fn add_progress(pb: ProgressBar) {
  let mp = ensure_mp();
  mp.add(pb.clone());
}

/// Create a new progress bar.
pub fn new_progress(len: u64) -> ProgressBar {
  let pb = ProgressBar::new(len);
  add_progress(pb.clone());
  pb
}

/// Create a new spinner bar.
pub fn new_spinner() -> ProgressBar {
  let pb = ProgressBar::new_spinner();
  add_progress(pb.clone());
  pb
}
