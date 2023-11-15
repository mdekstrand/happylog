use std::time::Duration;

use happylog::{new_progress, structopt::LogOpts};
use log::*;
use structopt::StructOpt;

static NUM_STEPS: u64 = 1000;

/// Example program with a basic progress bar.
#[derive(StructOpt, Debug)]
#[structopt(name = "basic-progress")]
struct BPExample {
    #[structopt(flatten)]
    logging: LogOpts,
}

pub fn main() {
    let opts = BPExample::from_args();
    opts.logging.init().expect("log initialization failed");

    info!("started program");
    // this line should only display when verbose
    debug!("debug line");

    let pb = new_progress(NUM_STEPS);
    for i in 0..NUM_STEPS {
        let step = i + 1;
        if step % 25 == 0 {
            info!("doing step {}", step);
        } else if step % 5 == 0 {
            debug!("intermediate step {}", step);
        } else {
            trace!("intermediate step {}", step);
        }
        std::thread::sleep(Duration::from_millis(10));
        pb.inc(1);
    }
    drop(pb);

    info!("finishing");
}
