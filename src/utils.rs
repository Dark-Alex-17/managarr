use std::fs;
use std::path::PathBuf;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;

#[cfg(test)]
#[path = "utils_tests.rs"]
mod utils_tests;

pub fn get_log_path() -> PathBuf {
  let mut log_path = if cfg!(target_os = "linux") {
    dirs_next::cache_dir().unwrap_or_else(|| PathBuf::from("~/.cache"))
  } else if cfg!(target_os = "macos") {
    dirs_next::home_dir().unwrap().join("Library/Logs")
  } else {
    dirs_next::data_local_dir().unwrap_or_else(|| PathBuf::from("C:\\Logs"))
  };

  log_path.push("managarr");

  // Create the directory if it doesn't exist
  if let Err(e) = fs::create_dir_all(&log_path) {
    eprintln!("Failed to create log directory: {:?}", e);
  }

  log_path.push("managarr.log");
  log_path
}

pub fn init_logging_config() -> log4rs::Config {
  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(
      "{d(%Y-%m-%d %H:%M:%S%.3f)(utc)} <{i}> [{l}] {f}:{L} - {m}{n}",
    )))
    .build(get_log_path())
    .unwrap();

  log4rs::Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(
      Root::builder()
        .appender("logfile")
        .build(LevelFilter::Debug),
    )
    .unwrap()
}

pub fn convert_to_gb(bytes: i64) -> f64 {
  bytes as f64 / 1024f64.powi(3)
}

pub fn convert_runtime(runtime: i64) -> (i64, i64) {
  let hours = runtime / 60;
  let minutes = runtime % 60;

  (hours, minutes)
}
