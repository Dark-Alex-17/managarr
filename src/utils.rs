use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use serde_json::Number;

pub fn init_logging_config() -> log4rs::Config {
  let file_path = "/tmp/managarr.log";
  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    .build(file_path)
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

pub fn convert_to_gb(bytes: u64) -> f64 {
  bytes as f64 / 1024f64.powi(3)
}

pub fn convert_runtime(runtime: u64) -> (u64, u64) {
  let hours = runtime / 60;
  let minutes = runtime % 60;

  (hours, minutes)
}
