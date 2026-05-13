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
    pub level: LogLevel,
    pub tx: tokio::sync::mpsc::Sender<String>,
}

pub async fn init_logger(mut rx: tokio::sync::mpsc::Receiver<String>) {
    fs::create_dir_all("log").expect("failed to create log dir");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log/log.txt").expect("failed to open log file");

    loop {
        if let Some(log) = rx.recv().await {
            if let Err(e) = writeln!(file, "{}", log) {
                eprintln!("Writing to log file failed: {e}");
            }
        } else {
            return;
        }
    }
}

pub fn log_internal(level: LogLevel, msg: Arguments) {
    let logger = LOGGER.get().expect("failed to get logger");
    if level >= logger.level {
        let now = Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        let log = format!("{} [{:?}] {}", timestamp, level, msg); 

        // Errors are ignored here i think
        tokio::spawn(logger.tx.send(log));
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
