use std::{fs::OpenOptions, io::Write, sync::OnceLock, time::Instant};

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub struct DebugLog();

#[macro_export]
macro_rules! debug {
    () => {};

    ($fmt_str:literal) => {
        $crate::logger::DebugLog::log($crate::logger::LogLevel::Debug, file!(), line!(), column!(), $fmt_str.to_string());
    };

    ($fmt_str:literal, $($args:expr),*) => {
        {
            let temp_string = format!($fmt_str, $($args),*);
            $crate::logger::DebugLog::log($crate::logger::LogLevel::Debug, file!(), line!(), column!(), temp_string);
        }
    };
}

#[macro_export]
macro_rules! info {
    () => {};

    ($fmt_str:literal) => {
        $crate::logger::DebugLog::log($crate::logger::LogLevel::Info, file!(), line!(), column!(), $fmt_str.to_string());
    };

    ($fmt_str:literal, $($args:expr),*) => {
        {
            let temp_string = format!($fmt_str, $($args),*);
            $crate::logger::DebugLog::log($crate::logger::LogLevel::Info, file!(), line!(), column!(), temp_string);
        }
    };
}

#[macro_export]
macro_rules! warning {
    () => {};

    ($fmt_str:literal) => {
        $crate::logger::DebugLog::log($crate::logger::LogLevel::Warn, file!(), line!(), column!(), $fmt_str.to_string());
    };

    ($fmt_str:literal, $($args:expr),*) => {
        {
            let temp_string = format!($fmt_str, $($args),*);
            $crate::logger::DebugLog::log($crate::logger::LogLevel::Warn, file!(), line!(), column!(), temp_string);
        }
    };
}

#[macro_export]
macro_rules! error {
    () => {};

    ($fmt_str:literal) => {
        $crate::logger::DebugLog::log($crate::logger::LogLevel::Error, file!(), line!(), column!(), $fmt_str.to_string());
    };

    ($fmt_str:literal, $($args:expr),*) => {
        {
            let temp_string = format!($fmt_str, $($args),*);
            $crate::logger::DebugLog::log($crate::logger::LogLevel::Error, file!(), line!(), column!(), temp_string);
        }
    };
}

impl DebugLog {
    pub fn init() {
        if cfg!(debug_assertions) {
            let file_name = format!("{}.log", PKG_NAME.to_ascii_lowercase());
            match OpenOptions::new().create(true).write(true).truncate(true).open(&file_name) {
                Ok(file) => file,
                Err(e) => panic!("Failed to create file: {} ({}).", file_name, e),
            };
            info!("Start logging for {}.", PKG_NAME);
        }
    }
    pub fn log(level: LogLevel, src_file: &str, line: u32, col: u32, message: String) {
        if cfg!(debug_assertions) {
            let file_name = format!("{}.log", PKG_NAME.to_ascii_lowercase());
            let msg_string = if level == LogLevel::Debug {
                format!("{} {}: {} ({} [{}:{}])", DebugLog::log_timer(), DebugLog::log_level_to_str(level), message, src_file, line, col)
            } else {
                format!("{} {}: {}", DebugLog::log_timer(), DebugLog::log_level_to_str(level), message)
            };
            let mut file = match OpenOptions::new().append(true).open(&file_name) {
                Ok(file) => file,
                Err(e) => panic!("Failed to open logger log file: {} ({}).", file_name, e)
            };
            if let Err(e) = writeln!(&mut file, "{}", msg_string) {
                panic!("Failed to write to logger log file ({}).", e);
            }
        } else {
            if level == LogLevel::Error {
                match OpenOptions::new().create(true).write(true).truncate(true).open("error.log") {
                    Ok(mut file) => {
                        if writeln!(&mut file, "ERROR: {}", message).is_err() {
                            panic!("ERROR: {}", message);
                        }
                    },
                    Err(_) => panic!("ERROR: {}", message),
                };
                std::process::exit(1);
            }
        }
    }
    pub fn log_timer() -> String {
        let duration = Instant::now().duration_since(*DebugLog::start_time());
        let time = DebugLog::millis_to_time(duration.as_millis());
        format!("[{:02}:{:02}:{:02}.{:03}]", time.0, time.1, time.2, time.3)
    }
    fn start_time()->&'static Instant {
        static START_TIME: OnceLock<Instant> = OnceLock::new();
        #[allow(clippy::redundant_closure)]
        START_TIME.get_or_init(|| Instant::now())
    }
    fn log_level_to_str(level: LogLevel) -> &'static str {
        match level {
            LogLevel::Debug => "logger",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARNING",
            LogLevel::Error => "ERROR",
        }
    }
    fn millis_to_time(millis: u128) -> (u32,u8,u8,u32) {
        let mut mil = millis % 86_400_000;
        let hrs = mil / 3_600_000;
        mil -= hrs * 3_600_000;
        let min = mil / 60_000;
        mil -= min * 60_000;
        let sec = mil / 1000;
        mil -= sec * 1000;
        (hrs as u32, min as u8, sec as u8, mil as u32)
    }

}
