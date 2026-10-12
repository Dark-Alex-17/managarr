#![cfg(unix)]

use std::{fs, path::PathBuf, process, str};

#[cfg(target_os = "linux")]
use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;

struct ProbeEnv {
  base: PathBuf,
}

impl ProbeEnv {
  fn new(test_name: &str) -> Self {
    let base = std::env::temp_dir().join(format!(
      "managarr_unwritable_log_dir_{test_name}_{}",
      process::id()
    ));
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(base.join("home")).unwrap();
    fs::create_dir_all(base.join("fresh_tmp")).unwrap();
    fs::write(base.join("blockfile"), "not a directory").unwrap();
    fs::write(
      base.join("config.yml"),
      "radarr:\n  - name: Movies\n    host: 127.0.0.1\n    port: 7878\n    api_token: test-token\n",
    )
    .unwrap();

    Self { base }
  }

  #[cfg(target_os = "linux")]
  fn blocked_cache_dir(&self) -> PathBuf {
    self.base.join("blockfile").join("cache")
  }

  #[cfg(target_os = "linux")]
  fn command(&self) -> Command {
    let mut command = cargo_bin_cmd!("managarr");
    command
      .env_clear()
      .env("PATH", std::env::var_os("PATH").unwrap())
      .env("HOME", self.base.join("home"))
      .env("XDG_CACHE_HOME", self.blocked_cache_dir())
      .env("TMPDIR", self.base.join("fresh_tmp"))
      .arg("--config-file")
      .arg(self.base.join("config.yml"));
    command
  }
}

impl Drop for ProbeEnv {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.base);
  }
}

#[cfg(target_os = "linux")]
#[test]
fn unwritable_log_dir_warns_and_cli_command_still_succeeds() {
  let env = ProbeEnv::new("completions");
  let expected_log_dir = env.blocked_cache_dir().join("managarr");

  let mut command = env.command();
  let assertion = command.args(["completions", "bash"]).assert().success();
  let output = assertion.get_output();
  let stderr = str::from_utf8(&output.stderr).unwrap();

  assert!(
    !output.stdout.is_empty(),
    "completions produced no stdout despite exiting successfully"
  );
  assert!(
    stderr.contains(&format!(
      "Failed to create log directory {}",
      expected_log_dir.display()
    )),
    "stderr did not warn about the uncreatable log directory: {stderr}"
  );
  assert!(
    stderr.contains("file logging will be disabled"),
    "stderr did not announce that file logging is disabled: {stderr}"
  );
  assert!(
    !stderr.contains("panicked"),
    "binary panicked instead of degrading gracefully: {stderr}"
  );
  assert!(
    fs::read_dir(env.base.join("fresh_tmp")).unwrap().count() == 0,
    "log output fell back to a temp directory despite the no-fallback ruling"
  );
}

#[cfg(target_os = "linux")]
#[test]
fn tail_logs_with_unopenable_log_file_returns_error_without_panicking() {
  let env = ProbeEnv::new("tail_logs");
  let expected_log_file = env
    .blocked_cache_dir()
    .join("managarr")
    .join("managarr.log");

  let mut command = env.command();
  let assertion = command
    .arg("tail-logs")
    .timeout(std::time::Duration::from_secs(30))
    .assert()
    .failure();
  let stderr = str::from_utf8(&assertion.get_output().stderr).unwrap();

  assert!(
    stderr.contains(&format!(
      "Unable to open log file {}",
      expected_log_file.display()
    )),
    "tail-logs error did not name the log file: {stderr}"
  );
  assert_eq!(
    stderr.matches("Failed to create log directory").count(),
    1,
    "the create-dir warning must print exactly once per process: {stderr}"
  );
  assert!(
    stderr.contains("file logging may be disabled"),
    "tail-logs error did not mention that file logging may be disabled: {stderr}"
  );
  assert!(
    !stderr.contains("panicked"),
    "tail-logs panicked instead of returning an error: {stderr}"
  );
}

#[cfg(target_os = "linux")]
#[test]
fn writable_cache_dir_creates_log_file_without_warnings() {
  let env = ProbeEnv::new("happy_path");
  let cache_dir = env.base.join("home").join("cache");
  fs::create_dir_all(&cache_dir).unwrap();

  let mut command = env.command();
  let assertion = command
    .env("XDG_CACHE_HOME", &cache_dir)
    .args(["completions", "bash"])
    .assert()
    .success();
  let stderr = str::from_utf8(&assertion.get_output().stderr).unwrap();

  assert!(
    stderr.is_empty(),
    "happy path emitted unexpected stderr: {stderr}"
  );
  assert!(
    cache_dir.join("managarr").join("managarr.log").is_file(),
    "logfile appender did not create managarr.log in the writable cache dir"
  );
}

#[test]
fn unset_home_does_not_panic_on_startup() {
  let env = ProbeEnv::new("unset_home");

  let mut command = cargo_bin_cmd!("managarr");
  let assertion = command
    .env_clear()
    .env("PATH", std::env::var_os("PATH").unwrap())
    .env("TMPDIR", env.base.join("fresh_tmp"))
    .current_dir(&env.base)
    .arg("--config-file")
    .arg(env.base.join("config.yml"))
    .args(["completions", "bash"])
    .assert()
    .success();
  let output = assertion.get_output();
  let stderr = str::from_utf8(&output.stderr).unwrap();

  assert!(
    !output.stdout.is_empty(),
    "completions produced no stdout despite exiting successfully"
  );
  assert!(
    !stderr.contains("panicked"),
    "binary panicked instead of degrading gracefully: {stderr}"
  );
}
