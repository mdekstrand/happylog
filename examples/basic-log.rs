use std::time::Duration;

use log::*;
use happylog::initialize;

pub fn main() {
  initialize(0).expect("log initialization failed");

  info!("started program");
  // this line shoudl not display
  debug!("hidden line");

  std::thread::sleep(Duration::from_millis(1000));

  info!("finishing");
}
