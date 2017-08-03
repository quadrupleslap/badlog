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

pub fn init<T: AsRef<str>>(level: Option<T>) {
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
    });

    if let Some(level) = level {
       builder.parse(level.as_ref());
    }

    builder.init().unwrap();
}

pub fn init_from_env<T: AsRef<str>>(envar: T) {
    init(env::var(envar.as_ref()).ok());
}

#[cfg(windows)] 
fn ansi_supported() -> bool {
    ansi_term::enable_ansi_support().is_ok()
}

#[cfg(not(windows))]
fn ansi_supported() -> bool {
    0 != unsafe {
        libc::isatty(libc::STDERR_FILENO)
    }
}
