#[cfg(test)]
mod tests {
  use anyhow::{Error, anyhow};
  use clap::Parser;
  use serde_json::json;
  use std::fs::{self, File};
  use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

  use pretty_assertions::{assert_eq, assert_str_eq};
  use tokio::sync::mpsc;
  use tokio_util::sync::CancellationToken;

  use crate::{
    Cli,
    app::{App, AppConfig, ServarrConfig},
    network::NetworkError,
    utils::{
      convert_runtime, format_cli_error, format_size, select_cli_configuration, was_log_rotated,
    },
  };

  #[test]
  fn test_select_cli_configuration_selects_valid_named_instance() {
    let selected_config = ServarrConfig {
      name: Some("4K Movies".to_owned()),
      ..ServarrConfig::default()
    };
    let config = AppConfig {
      radarr: Some(vec![ServarrConfig::default(), selected_config.clone()]),
      ..AppConfig::default()
    };
    let (network_tx, _network_rx) = mpsc::channel(1);
    let mut app = App::new(network_tx, config.clone(), CancellationToken::new());
    let args = Cli::try_parse_from([
      "managarr",
      "radarr",
      "clear-blocklist",
      "--servarr-name",
      "4K Movies",
    ])
    .unwrap();

    select_cli_configuration(
      &mut app,
      &config,
      args.command.as_ref().unwrap(),
      args.global.servarr_name.as_deref(),
    )
    .unwrap();

    assert_eq!(app.server_tabs.get_active_config(), &Some(selected_config));
  }

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
  fn test_format_cli_error_network_error_with_json_body() {
    let error = Error::new(NetworkError {
      message: "Request failed. Received 404 Not Found response code with body: Not Found"
        .to_owned(),
      status: 404,
      body: Some(json!({ "status": 404, "title": "Not Found" })),
    });

    let output = format_cli_error(&error);

    assert_str_eq!(
      output,
      r#"{
  "error": {
    "message": "Request failed. Received 404 Not Found response code with body: Not Found",
    "status": 404,
    "body": {
      "status": 404,
      "title": "Not Found"
    }
  }
}"#
    );
  }

  #[test]
  fn test_format_cli_error_network_error_with_string_body() {
    let error = Error::new(NetworkError {
      message:
        "Request failed. Received 503 Service Unavailable response code with body: Service Unavailable"
          .to_owned(),
      status: 503,
      body: Some(json!("Service Unavailable")),
    });

    let output = format_cli_error(&error);

    assert_str_eq!(
      output,
      r#"{
  "error": {
    "message": "Request failed. Received 503 Service Unavailable response code with body: Service Unavailable",
    "status": 503,
    "body": "Service Unavailable"
  }
}"#
    );
  }

  #[test]
  fn test_format_cli_error_network_error_with_null_body() {
    let error = Error::new(NetworkError {
      message: "Request failed. Received 404 Not Found response code with body: ".to_owned(),
      status: 404,
      body: None,
    });

    let output = format_cli_error(&error);

    assert_str_eq!(
      output,
      r#"{
  "error": {
    "message": "Request failed. Received 404 Not Found response code with body: ",
    "status": 404,
    "body": null
  }
}"#
    );
  }

  #[test]
  fn test_format_cli_error_network_error_wrapped_in_context() {
    let error = Error::new(NetworkError {
      message: "Request failed. Received 404 Not Found response code with body: Not Found"
        .to_owned(),
      status: 404,
      body: Some(json!({ "status": 404, "title": "Not Found" })),
    })
    .context("failed to fetch system status");

    let output = format_cli_error(&error);

    assert_str_eq!(
      output,
      r#"{
  "error": {
    "message": "Request failed. Received 404 Not Found response code with body: Not Found",
    "status": 404,
    "body": {
      "status": 404,
      "title": "Not Found"
    }
  }
}"#
    );
  }

  #[test]
  fn test_format_cli_error_plain_anyhow_error() {
    let error = anyhow!("Failed to send request. connection refused ");

    let output = format_cli_error(&error);

    assert_str_eq!(
      output,
      r#"{
  "error": {
    "message": "Failed to send request. connection refused ",
    "status": null,
    "body": null
  }
}"#
    );
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
