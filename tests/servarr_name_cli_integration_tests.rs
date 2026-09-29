use std::{
  fs,
  path::{Path, PathBuf},
  process, str,
  sync::atomic::{AtomicU64, Ordering},
};

use assert_cmd::cargo::cargo_bin_cmd;
use mockito::Server;

static NEXT_CONFIG_ID: AtomicU64 = AtomicU64::new(0);

struct TemporaryConfig {
  path: PathBuf,
}

impl TemporaryConfig {
  fn new(contents: &str) -> Self {
    let path = std::env::temp_dir().join(format!(
      "managarr-servarr-name-{}-{}.yaml",
      process::id(),
      NEXT_CONFIG_ID.fetch_add(1, Ordering::Relaxed)
    ));
    fs::write(&path, contents).unwrap();
    Self { path }
  }

  fn path(&self) -> &Path {
    &self.path
  }
}

impl Drop for TemporaryConfig {
  fn drop(&mut self) {
    let _ = fs::remove_file(&self.path);
  }
}

#[test]
fn named_instance_of_different_servarr_type_rejects_command() {
  let mut radarr_server = Server::new();
  let radarr_mock = radarr_server
    .mock("GET", "/api/v3/system/status")
    .match_header("X-Api-Key", "test-token")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(r#"{"version":"radarr","startTime":"2024-01-01T00:00:00Z"}"#)
    .expect(0)
    .create();
  let mut sonarr_server = Server::new();
  let sonarr_mock = sonarr_server
    .mock("GET", "/api/v3/system/status")
    .match_header("X-Api-Key", "test-token")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(r#"{"version":"sonarr","startTime":"2024-01-01T00:00:00Z"}"#)
    .expect(0)
    .create();
  let config = TemporaryConfig::new(&format!(
    "radarr:\n  - name: Movies\n    uri: {}\n    api_token: test-token\nsonarr:\n  - name: Shows\n    uri: {}\n    api_token: test-token\n",
    radarr_server.url(),
    sonarr_server.url()
  ));

  let mut command = cargo_bin_cmd!("managarr");
  let assertion = command
    .arg("--config-file")
    .arg(config.path())
    .args([
      "--disable-spinner",
      "--servarr-name",
      "Shows",
      "radarr",
      "get",
      "system-status",
    ])
    .assert()
    .failure();
  let stderr = str::from_utf8(&assertion.get_output().stderr).unwrap();

  assert!(
    stderr.contains("Shows"),
    "CLI error did not identify the rejected instance: {stderr}"
  );
  radarr_mock.assert();
  sonarr_mock.assert();
}
