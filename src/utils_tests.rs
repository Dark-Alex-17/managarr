#[cfg(test)]
mod tests {
  use std::fs::{self, File};
  use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::utils::{convert_runtime, format_size, was_log_rotated};

  #[test]
  fn test_format_size() {
    assert_str_eq!(format_size(2_457_600, 2), "2.34 MB");
    assert_str_eq!(format_size(2_469_606_195, 2), "2.30 GB");
    assert_str_eq!(format_size(1_073_741_824, 2), "1.00 GB");
    assert_str_eq!(format_size(1_073_741_823, 2), "1024.00 MB");
    assert_str_eq!(format_size(0, 2), "0.00 MB");
    assert_str_eq!(format_size(6_710_886, 1), "6.4 MB");
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
