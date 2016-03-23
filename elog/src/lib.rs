extern crate log;
use log::{LogRecord, LogLevel, LogMetadata, SetLoggerError, LogLevelFilter};

extern crate chrono;
use chrono::*;

extern crate colored;
use colored::*;

extern crate rustc_serialize;
use std::str::FromStr;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

pub struct SimpleLogger {
    target: String,
}

impl SimpleLogger {

    pub fn new() -> SimpleLogger {
        let target = cfg_str!("target");
        SimpleLogger {
            target: target.to_string(),
        }
    }

}

impl log::Log for SimpleLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        if self.target == "run" {
            metadata.level() <= LogLevel::Info
        } else {
            metadata.level() <= LogLevel::Info
        }
    }

    fn log(&self, record: &LogRecord) {
        let mt = record.metadata();
        if self.enabled(mt) {
            let head = format!("[{}] [{}] [{}]", Local::now(), record.level(), mt.target());
            let colored_head;
            match record.level() {
                LogLevel::Error => {
                    colored_head = head.red();
                },
                LogLevel::Debug => {
                    colored_head = head.yellow();
                },
                _ => {
                    colored_head = head.green();
                }
            }
            println!("{} {}", colored_head, record.args());
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    let target = cfg_str!("target");
    log::set_logger(|max_log_level| {
        if target == "run" {
            max_log_level.set(LogLevelFilter::Info);
        } else {
            max_log_level.set(LogLevelFilter::Trace);
        }
        Box::new(SimpleLogger::new())
    })
}