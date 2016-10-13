extern crate fern;
#[macro_use]
extern crate log;
extern crate time;
extern crate verex;

pub mod command;
pub mod config;
pub mod env;
mod url_check;
pub mod server;

pub fn init_logging(log_level: u8) {
    let dir = std::env::current_exe().unwrap().parent().unwrap().join(".wydyd.log");
    if dir.exists() {
        std::fs::remove_file(&dir).unwrap();
    }

    let log_level = match log_level {
        1 => log::LogLevelFilter::Error,
        2 => log::LogLevelFilter::Warn,
        3 => log::LogLevelFilter::Info,
        4 => log::LogLevelFilter::Debug,
        5 => log::LogLevelFilter::Trace,
        _ => log::LogLevelFilter::Off,
    };

    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, location: &log::LogLocation| {
            if *level == log::LogLevel::Debug || *level == log::LogLevel::Trace {
                format!("[{}][{}] {}:{}: {}",
                        time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(),
                        level,
                        location.file(),
                        location.line(),
                        msg)
            } else {
                format!("[{}][{}]: {}",
                        time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(),
                        level,
                        msg)
            }
        }),
        output: vec![fern::OutputConfig::file(&dir), fern::OutputConfig::stdout()],
        level: log_level,
    };
    // TODO modify log level
    fern::init_global_logger(logger_config, log_level).unwrap();
    trace!("Init logging");
}
