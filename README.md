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

## Acknowledgements

Copyright &copy; 2020 Boise State University.  Distributed under the MIT License; see LICENSE.md.
This material is based upon work supported by the National Science Foundation under
Grant No. IIS 17-51278. Any opinions, findings, and conclusions or recommendations
expressed in this material are those of the author(s) and do not necessarily reflect
the views of the National Science Foundation.
