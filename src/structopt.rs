//! Support for command-line argument configuration of the logger.
use crate::{init_from_verbosity, verbosity::Verbosity};
use log::SetLoggerError;
use structopt::StructOpt;

#[cfg_attr(not(doc), allow(missing_docs))]
#[cfg_attr(
    doc,
    doc = r#"
Command line options for configuring the logger with StructOpt.

It's recommended to use this in your program like this:

```
use structopt::StructOpt;
use happylog::structopt::LogOpts;

#[derive(StructOpt, Debug)]
#[structopt(name="command")]
struct Command {
  #[structopt(flatten)]
  logging: LogOpts
}
```

In your `main`, you can then call `opts.logging.init()` to initialize 
the logging framework.
"#
)]
#[derive(StructOpt, Debug)]
pub struct LogOpts {
    /// Increases logging verbosity mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i32,
    /// Silences informational output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
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
