//! Happy log formatters.

use std::fmt::Arguments;

use fern::FormatCallback;
use log::Record;

#[cfg(feature = "colored")]
use fern::colors::{Color, ColoredLevelConfig};

#[cfg(feature = "colored")]
static COLORS: ColoredLevelConfig = ColoredLevelConfig {
    error: Color::BrightRed,
    warn: Color::BrightYellow,
    info: Color::BrightBlue,
    debug: Color::White,
    trace: Color::White,
};

pub fn format_plain(out: FormatCallback<'_>, message: &Arguments<'_>, rec: &Record<'_>) {
    out.finish(format_args!("[{:5}] {}", rec.level(), message))
}

#[cfg(feature = "colored")]
pub fn format_color(out: FormatCallback<'_>, message: &Arguments<'_>, rec: &Record<'_>) {
    out.finish(format_args!(
        "[{:5}] {}",
        COLORS.color(rec.level()),
        message
    ))
}

#[cfg(not(feature = "colored"))]
pub fn format_color(out: FormatCallback<'_>, message: &Arguments<'_>, rec: &Record<'_>) {
    format_plain(out, message, rec);
}
