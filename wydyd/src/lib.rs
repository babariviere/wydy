// Config for clippy
#![allow(unknown_lints)]
#![allow(doc_markdown)]

extern crate app_dirs;
// extern crate fern;
#[macro_use]
extern crate log;
extern crate parser_rs;
extern crate simplelog;
extern crate verex;

pub mod command;
pub mod env;
pub mod parser;
pub mod server;
mod url_check;

use app_dirs::AppInfo;
use simplelog::*;
use std::fs::File;

const APP_INFO: AppInfo = AppInfo {
    name: "wydy",
    author: "notkild",
};

/// Init logging.
pub fn init_logging(log_level: u8) {
    let dir = std::env::current_exe().unwrap().parent().unwrap().join(".wydyd.log");
    if dir.exists() {
        std::fs::remove_file(&dir).unwrap();
    }

    let log_level = match log_level {
        1 => LogLevelFilter::Error,
        2 => LogLevelFilter::Warn,
        3 => LogLevelFilter::Info,
        4 => LogLevelFilter::Debug,
        5 => LogLevelFilter::Trace,
        _ => LogLevelFilter::Off,
    };

    CombinedLogger::init(vec![
            TermLogger::new(log_level, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Trace, Config::default(), File::create(dir).unwrap()),
        ])
        .unwrap();

}
