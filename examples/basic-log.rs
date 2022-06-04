use log::*;
use happylog::initialize;

pub fn main() {
  initialize(LevelFilter::INFO);

  info!("started program");
  // this line shoudl not display
  debug!("hidden line");

  std::thread::sleep(1000);

  info!("finishing");
}
