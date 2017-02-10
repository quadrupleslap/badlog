extern crate ansi_term;
extern crate env_logger;
extern crate libc;
extern crate log;

use ansi_term::Colour::*;
use env_logger::LogBuilder;
use log::LogLevel::*;
use log::LogLevelFilter;
use std::env;

pub fn init(envar: &str) {
    let mut builder = LogBuilder::new();

    builder.filter(None, LogLevelFilter::Info);

    if ansi_supported() {
        builder.format(|record| {
            format!("{} {}", match record.level() {
                Error =>    Red.paint("[ERROR]"),
                Warn  => Yellow.paint(" [WARN]"),
                Info  =>   Cyan.paint(" [INFO]"),
                Debug =>  Green.paint("[DEBUG]"),
                Trace => Purple.paint("[TRACE]"),
            }, record.args())
        });
    } else {
        builder.format(|record| {
            format!("{} {}", match record.level() {
                Error => "[ERROR]",
                Warn  => " [WARN]",
                Info  => " [INFO]",
                Debug => "[DEBUG]",
                Trace => "[TRACE]",
            }, record.args())
        });
    }

    if let Ok(var) = env::var(envar) {
       builder.parse(&var);
    }

    builder.init().unwrap();
}

#[cfg(windows)] 
fn ansi_supported() -> bool {
    //TODO: Except Windows 10 "Threshold 2".
    false
}

#[cfg(not(windows))]
fn ansi_supported() -> bool {
    let isatty = unsafe {
        libc::isatty(libc::STDERR_FILENO)
    };

    isatty != 0
}
