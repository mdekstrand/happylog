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
    initialize(0).unwrap();
    info!("info logging message");
}
```

## Major Changes

In 0.3, Happylog changed to use Fern instead of its own log target.  It will
eventually expose Fern dispatchers to allow for more thorough log configuration.

## Acknowledgements

Copyright &copy; 2020–2022 Boise State University.  Distributed under the MIT
License; see LICENSE.md. This material is based upon work supported by the
National Science Foundation under Grant No. IIS 17-51278. Any opinions,
findings, and conclusions or recommendations expressed in this material are
those of the author(s) and do not necessarily reflect the views of the National
Science Foundation.
