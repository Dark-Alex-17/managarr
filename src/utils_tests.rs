#[cfg(test)]
mod tests {
  use std::fs::{self, File};
  use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

  use pretty_assertions::assert_eq;

  use crate::utils::{convert_f64_to_gb, convert_runtime, convert_to_gb, was_log_rotated};

  #[test]
  fn test_convert_to_gb() {
    assert_eq!(convert_to_gb(2147483648), 2f64);
    assert_eq!(convert_to_gb(2662879723), 2.4799999995157123);
  }

  #[test]
  fn test_convert_f64_to_gb() {
    assert_eq!(convert_f64_to_gb(2147483648f64), 2f64);
    assert_eq!(convert_f64_to_gb(2662879723f64), 2.4799999995157123);
  }

  #[test]
  fn test_convert_runtime() {
    let (hours, minutes) = convert_runtime(154);

    assert_eq!(hours, 2);
    assert_eq!(minutes, 34);
  }

  #[test]
  fn test_was_log_rotated_returns_false_when_file_has_not_rotated() {
    let path = std::env::temp_dir().join("managarr_test_no_rotation.log");
    fs::write(&path, "line one\nline two\n").unwrap();

    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(0)).unwrap();

    assert!(!was_log_rotated(&path, &mut reader));

    fs::remove_file(&path).unwrap();
  }

  #[test]
  fn test_was_log_rotated_returns_true_and_reopens_reader_after_rotation() {
    let path = std::env::temp_dir().join("managarr_test_rotation.log");
    fs::write(&path, "original content that is long enough\n").unwrap();

    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(0)).unwrap();

    fs::write(&path, "new\n").unwrap();

    assert!(was_log_rotated(&path, &mut reader));

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert_eq!(line, "new\n");

    fs::remove_file(&path).unwrap();
  }

  #[test]
  fn test_was_log_rotated_returns_false_when_file_grows() {
    let path = std::env::temp_dir().join("managarr_test_growing.log");
    fs::write(&path, "initial\n").unwrap();

    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(0)).unwrap();

    let mut appender = fs::OpenOptions::new().append(true).open(&path).unwrap();
    appender.write_all(b"more data\n").unwrap();

    assert!(!was_log_rotated(&path, &mut reader));

    fs::remove_file(&path).unwrap();
  }
}
