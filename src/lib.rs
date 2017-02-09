extern crate ansi_term;
extern crate log;
extern crate env_logger;

use ansi_term::Colour::*;
use env_logger::LogBuilder;
use log::LogLevel::*;
use std::env;

pub fn init(envar: &'static str) {
    let mut builder = LogBuilder::new();

    builder.format(|record| {
        format!("{} {}", match record.level() {
            Error =>    Red.paint("[ERROR]"),
            Warn  => Yellow.paint(" [WARN]"),
            Info  =>   Cyan.paint(" [INFO]"),
            Debug =>  Green.paint("[DEBUG]"),
            Trace => Purple.paint("[TRACE]"),
        }, record.args())
    });

    if let Ok(var) = env::var(envar) {
       builder.parse(&var);
    }

    builder.init().unwrap();
}
