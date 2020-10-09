use structopt::StructOpt;
use log::SetLoggerError;

/// Command line options for configuring the logger with StructOpt.
/// 
/// It's recommended to use this in your program like this:
/// 
/// ```
/// #[derive(StructOpt, Debug)]
/// #[structopt(name="command")]
/// struct Command {
///   #[structopt(flatten)]
///   logging: LogOpts
/// }
/// ```
/// 
/// In your `main`, you can then call `opts.logging.init()` to initialize 
/// the logging framework.
#[derive(StructOpt, Debug)]
pub struct LogOpts {
  /// Verbose mode (-v, -vv, -vvv, etc.)
  #[structopt(short="v", long="verbose", parse(from_occurrences))]
  verbose: usize,
  /// Silence output
  #[structopt(short="q", long="quiet")]
  quiet: bool
}

impl LogOpts {
  /// Initialize logging
  pub fn init(&self) -> Result<(), SetLoggerError> {
    let mut level = LevelFilter::Info;
    if self.quiet {
      level = LevelFilter::Off;
    }
    for _i in 0..self.verbose {
      level = verbosify(level);
    }
    crate::console_emitter::init(level)
  }
}
