extern crate ansi_term;
extern crate env_logger;
extern crate libc;
extern crate log;
extern crate time;

use ansi_term::Colour::*;
use env_logger::LogBuilder;
use log::LogLevel::*;
use log::LogLevelFilter;
use std::env;

pub fn init<T: AsRef<str>>(level: Option<T>) {
    let mut builder = LogBuilder::new();

    builder.filter(None, LogLevelFilter::Info);

    let (error, warn, info, debug, trace) =
        if ansi_supported() {(
               Red.paint("[ERROR]").to_string(),
            Yellow.paint("[WARN] ").to_string(),
              Cyan.paint("[INFO] ").to_string(),
             Green.paint("[DEBUG]").to_string(),
            Purple.paint("[TRACE]").to_string()
        )} else {(
            "[ERROR]".to_owned(),
            "[WARN] ".to_owned(),
            "[INFO] ".to_owned(),
            "[DEBUG]".to_owned(),
            "[TRACE]".to_owned()
        )};

    builder.format(move |record| {
        format!("{} {} {} {}",
            Fixed(8).paint(time::now().strftime("%H:%M:%S").unwrap().to_string()),
            match record.level() {
                Error => &error,
                Warn  => &warn,
                Info  => &info,
                Debug => &debug,
                Trace => &trace
            },
            Fixed(8).paint(format!("[{}]", record.target())),
            record.args()
        )
    });

    if let Some(level) = level {
       builder.parse(level.as_ref());
    }

    builder.init().unwrap();
}

pub fn init_from_env(envar: &str) {
    init(env::var(envar).ok());
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
