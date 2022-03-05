// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}

pub fn print_level (level: LogLevel) -> String {
    match level {
        LogLevel::Info => String::from("INFO"),
        LogLevel::Warning => String::from("WARNING"),
        LogLevel::Error => String::from("ERROR"),
        LogLevel::Debug => String::from("DEBUG"),
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    return format!("[{}]: {}", format!("{:?}", level).to_uppercase(), message);
}
pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message);
}
pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message);
}
pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message);
}
