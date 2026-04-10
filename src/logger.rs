use std::fmt::Arguments;
use std::fs::{self, OpenOptions};
use std::io::Write;

use chrono::Local;

use crate::LOGGER;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    DEBUG,
    INFO, 
    WARN, 
    ERROR 
}

pub struct Logger {
    pub level: LogLevel
}

pub fn log_internal(level: LogLevel, msg: Arguments) {
    // TODO Uh lets just log to a file for now. No batching maybe background thread?

    // i suppose background thread will do this once and not every time
    // BECAUse opening the file and dir every log not really that cool
    fs::create_dir_all("log").expect("failed to create log dir");

    let logger = LOGGER.get().expect("failed to get logger");
    if level >= logger.level {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("log/log.txt").expect("failed to open log file");

        let now = Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        if let Err(e) = writeln!(file, "{} [{:?}] {}", timestamp, level, msg) {
            eprintln!("Writing to log file failed: {e}");
        }
    }
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logger::log_internal($crate::logger::LogLevel::ERROR, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logger::log_internal($crate::logger::LogLevel::WARN, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logger::log_internal($crate::logger::LogLevel::INFO, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::log_internal($crate::logger::LogLevel::DEBUG, format_args!($($arg)*));
    };
}
