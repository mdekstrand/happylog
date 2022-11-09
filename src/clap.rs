//! Support for command-line argument configuration of the logger.
use clap::Parser;
use log::{SetLoggerError};
use crate::{verbosity::Verbosity, init_from_verbosity};

#[cfg_attr(not(doc), allow(missing_docs))]
#[cfg_attr(doc, doc=r#"
Command line options for configuring the logger with StructOpt.

It's recommended to use this in your program like this:

```
#[derive(Clap, Debug)]
#[command(name="command")]
struct Command {
  #[arg(flatten)]
  logging: LogOpts
}
```

In your `main`, you can then call `opts.logging.init()` to initialize 
the logging framework.
"#)]
#[derive(Parser, Debug)]
pub struct LogOpts {
  /// Increases logging verbosity mode (-v, -vv, -vvv, etc.)
  #[arg(short="v", long="verbose", parse(from_occurrences))]
  verbose: i32,
  /// Silences informational output
  #[arg(short="q", long="quiet")]
  quiet: bool
}

impl LogOpts {
  /// Initialize logging
  pub fn init(&self) -> Result<(), SetLoggerError> {
    let mut verb = Verbosity::default();
    if self.quiet {
      verb.verbosity(-1);
    } else {
      verb.verbosity(self.verbose);
    }

    init_from_verbosity(verb)
  }
}
