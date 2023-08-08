use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use regex::Regex;

pub fn init_logging_config() -> log4rs::Config {
  let file_path = "/tmp/managarr.log";
  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(
      "{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n})}",
    )))
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

pub fn strip_non_alphanumeric_characters(input: &str) -> String {
  Regex::new(r"[^a-zA-Z0-9\s]")
    .unwrap()
    .replace_all(&input.to_lowercase(), "")
    .to_string()
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use crate::utils::{convert_runtime, convert_to_gb, strip_non_alphanumeric_characters};

  #[test]
  fn test_convert_to_gb() {
    assert_eq!(convert_to_gb(2147483648), 2f64);
    assert_eq!(convert_to_gb(2662879723), 2.4799999995157123);
  }

  #[test]
  fn test_convert_runtime() {
    let (hours, minutes) = convert_runtime(154);

    assert_eq!(hours, 2);
    assert_eq!(minutes, 34);
  }

  #[test]
  fn test_strop_non_alphanumeric_characters() {
    assert_eq!(
      strip_non_alphanumeric_characters("Te$t S7r!ng::'~-_}"),
      String::from("tet s7rng")
    )
  }
}
