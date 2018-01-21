#![deny(missing_docs)]

//! A ridiculously minimal and good-looking logger.

extern crate env_logger;
extern crate libc;
extern crate log;
extern crate chrono;

use chrono::Local;
use env_logger::Builder;
use env_logger::fmt::{Color, Style, StyledValue};
use log::{Level, LevelFilter};
use log::Level::{Trace, Debug, Info, Warn, Error};
use std::{env, fmt};
use std::io::Write;

/// Use the given string as the log level.
pub fn init<T: AsRef<str>>(level: Option<T>) {
    inner(level, false)
}

/// Use the value of the given environment variable as the log level.
pub fn init_from_env<T: AsRef<str>>(envar: T) {
    init(env::var(envar.as_ref()).ok())
}

/// Same as `init`, but hiding timestamps and target modules.
pub fn minimal<T: AsRef<str>>(level: Option<T>) {
    inner(level, true)
}

/// Same as `init_from_env`, but hiding timestamps and target modules.
pub fn minimal_from_env<T: AsRef<str>>(envar: T) {
    minimal(env::var(envar.as_ref()).ok())
}

fn inner<T: AsRef<str>>(level: Option<T>, minimal: bool) {
    let mut builder = Builder::new();

    builder.filter(None, LevelFilter::Info);

    if minimal {
        builder.format(|buf, record| {
            let mut level_style = buf.style();
            let level = format_level(
                &mut level_style,
                record.level());

            writeln!(buf, "{} {}", level, record.args())
        });
    } else {
        builder.format(|buf, record| {
            let mut level_style = buf.style();
            let level = format_level(
                &mut level_style,
                record.level());

            let mut detail_style = buf.style();
            detail_style.set_color(Color::Black);
            detail_style.set_intense(true);

            writeln!(buf, "{} {} {} {}",
                detail_style.value(Local::now().format("%H:%M:%S")),
                level,
                detail_style.value(Brackets(record.target())),
                record.args())
        });
    }

    if let Some(level) = level {
       builder.parse(level.as_ref());
    }

    builder.init();
}

fn format_level(style: &mut Style, level: Level) -> StyledValue<&'static str> {
    let (color, string) = match level {
        Error => (Color::Red,     "[ERROR]"),
        Warn  => (Color::Yellow,  "[WARN] "),
        Info  => (Color::Cyan,    "[INFO] "),
        Debug => (Color::Green,   "[DEBUG]"),
        Trace => (Color::Magenta, "[TRACE]"),
    };
    style.set_color(color);
    style.value(string)
}

struct Brackets<T>(T);
impl<T: fmt::Display> fmt::Display for Brackets<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}
