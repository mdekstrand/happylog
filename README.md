# Console logging backend

This is a logging implementation for [log](https://docs.rs/log/) that is designed for console programs.
It is easy to configure, optionally integrates with [structopt](https://docs.rs/structopt/) for configuration,
and can write log messages to `stderr` in concert with [indicatif](https://docs.rs/indicatif/) for
coordinated logging and progress reporting.

## Example

You can use this as follows:

```rust
use log::*;
use happylog::*;

fn main() -> () {
    initialize(LevelFilter::Info).unwrap();
    info!("info logging message");
}
```
