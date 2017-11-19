#![deny(missing_docs)]

//! A ridiculously minimal and good-looking logger.

extern crate ansi_term;
extern crate env_logger;
extern crate libc;
extern crate log;
extern crate time;

use ansi_term::Colour;
use ansi_term::Colour::*;
use env_logger::LogBuilder;
use log::LogLevel::*;
use log::LogLevelFilter;
use std::env;

const DETAILS_COLOR: Colour = Fixed(8);

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
    let mut builder = LogBuilder::new();

    builder.filter(None, LogLevelFilter::Info);

    let (error, warn, info, debug, trace) =
        if ansi_supported() {(
               Red.paint("[ERROR]"),
            Yellow.paint("[WARN] "),
              Cyan.paint("[INFO] "),
             Green.paint("[DEBUG]"),
            Purple.paint("[TRACE]")
        )} else {(
            "[ERROR]".into(),
            "[WARN] ".into(),
            "[INFO] ".into(),
            "[DEBUG]".into(),
            "[TRACE]".into()
        )};

    builder.format(move |record| {
        if minimal {
            format!("{} {}",
                match record.level() {
                    Error => &error,
                    Warn  => &warn,
                    Info  => &info,
                    Debug => &debug,
                    Trace => &trace
                },

                record.args()
            )
        } else {
            format!("{}{}{} {} {}[{}]{} {}",
                DETAILS_COLOR.prefix(),
                time::now()
                    .strftime("%H:%M:%S")
                    .unwrap(),
                DETAILS_COLOR.suffix(),

                match record.level() {
                    Error => &error,
                    Warn  => &warn,
                    Info  => &info,
                    Debug => &debug,
                    Trace => &trace
                },

                DETAILS_COLOR.prefix(),
                record.target(),
                DETAILS_COLOR.suffix(),

                record.args()
            )
        }
    });

    if let Some(level) = level {
       builder.parse(level.as_ref());
    }

    builder.init().unwrap();
}

#[cfg(windows)] 
fn ansi_supported() -> bool {
    false
}

#[cfg(not(windows))]
fn ansi_supported() -> bool {
    0 != unsafe {
        libc::isatty(libc::STDERR_FILENO)
    }
}
