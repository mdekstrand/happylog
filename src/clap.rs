//! Support for command-line argument configuration of the logger.
use crate::{init_from_verbosity, verbosity::Verbosity};
use clap::Args;
use log::SetLoggerError;

#[cfg_attr(not(doc), allow(missing_docs))]
#[cfg_attr(
    doc,
    doc = r#"
Command line options for configuring the logger with StructOpt.

It's recommended to use this in your program like this:

```
use clap::Parser;
use happylog::clap::LogOpts;
#[derive(Parser, Debug)]
#[command(name="command")]
struct Command {
  #[command(flatten)]
  logging: LogOpts
}
```

In your `main`, you can then call `opts.logging.init()` to initialize 
the logging framework.
"#
)]
#[derive(Args, Debug)]
pub struct LogOpts {
    /// Increases logging verbosity mode (-v, -vv, -vvv, etc.)
    #[arg(short='v', long="verbose", action=clap::ArgAction::Count)]
    verbose: u8,
    /// Silences informational output
    #[arg(short = 'q', long = "quiet", conflicts_with = "verbose")]
    quiet: bool,
}

impl LogOpts {
    /// Initialize logging
    pub fn init(&self) -> Result<(), SetLoggerError> {
        let mut verb = Verbosity::default();
        if self.quiet {
            verb.verbosity(-1);
        } else {
            verb.verbosity(self.verbose.into());
        }

        init_from_verbosity(verb)
    }
}
