use std::fs::{self, File};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use regex::Regex;
use reqwest::{Certificate, Client};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::app::{log_and_print_error, App, AppConfig};
use crate::cli::{self, Command};
use crate::network::Network;

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

pub async fn tail_logs(no_color: bool) {
  let re = Regex::new(r"^(?P<timestamp>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3})\s+<(?P<opid>[^\s>]+)>\s+\[(?P<level>[A-Z]+)\]\s+(?P<logger>[^:]+):(?P<line>\d+)\s+-\s+(?P<message>.*)$").unwrap();
  let file_path = get_log_path();
  let file = File::open(&file_path).expect("Cannot open file");
  let mut reader = BufReader::new(file);

  if let Err(e) = reader.seek(SeekFrom::End(0)) {
    eprintln!("Unable to tail log file: {e:?}");
    process::exit(1);
  };

  let mut lines = reader.lines();

  tokio::spawn(async move {
    loop {
      if let Some(Ok(line)) = lines.next() {
        if no_color {
          println!("{}", line);
        } else {
          let colored_line = colorize_log_line(&line, &re);
          println!("{}", colored_line);
        }
      }
    }
  })
  .await
  .unwrap();
}

fn colorize_log_line(line: &str, re: &Regex) -> String {
  if let Some(caps) = re.captures(line) {
    let level = &caps["level"];
    let message = &caps["message"];

    let colored_message = match level {
      "ERROR" => message.red(),
      "WARN" => message.yellow(),
      "INFO" => message.green(),
      "DEBUG" => message.blue(),
      _ => message.normal(),
    };

    let timestamp = &caps["timestamp"];
    let opid = &caps["opid"];
    let logger = &caps["logger"];
    let line_number = &caps["line"];

    format!(
      "{} <{}> [{}] {}:{} - {}",
      timestamp.white(),
      opid.cyan(),
      level.bold(),
      logger.magenta(),
      line_number.bold(),
      colored_message
    )
  } else {
    line.to_string()
  }
}

pub(super) fn load_config(path: &str) -> Result<AppConfig> {
  let file = File::open(path).map_err(|e| anyhow!(e))?;
  let reader = BufReader::new(file);
  let config = serde_yaml::from_reader(reader)?;
  Ok(config)
}

pub(super) fn build_network_client(config: &AppConfig) -> Client {
  let mut client_builder = Client::builder()
    .pool_max_idle_per_host(10)
    .http2_keep_alive_interval(Duration::from_secs(5))
    .tcp_keepalive(Duration::from_secs(5));

  if let Some(radarr_config) = &config.radarr {
    if let Some(ref cert_path) = &radarr_config.ssl_cert_path {
      let cert = create_cert(cert_path, "Radarr");
      client_builder = client_builder.add_root_certificate(cert);
    }
  }

  if let Some(sonarr_config) = &config.sonarr {
    if let Some(ref cert_path) = &sonarr_config.ssl_cert_path {
      let cert = create_cert(cert_path, "Sonarr");
      client_builder = client_builder.add_root_certificate(cert);
    }
  }

  match client_builder.build() {
    Ok(client) => client,
    Err(e) => {
      error!("Unable to create reqwest client: {}", e);
      eprintln!("error: {}", e.to_string().red());
      process::exit(1);
    }
  }
}

pub(super) fn create_cert(cert_path: &String, servarr_name: &str) -> Certificate {
  match fs::read(cert_path) {
    Ok(cert) => match Certificate::from_pem(&cert) {
      Ok(certificate) => certificate,
      Err(_) => {
        log_and_print_error(format!(
          "Unable to read the specified {} SSL certificate",
          servarr_name
        ));
        process::exit(1);
      }
    },
    Err(_) => {
      log_and_print_error(format!(
        "Unable to open specified {} SSL certificate",
        servarr_name
      ));
      process::exit(1);
    }
  }
}

pub(super) fn render_spinner() -> ProgressBar {
  let pb = ProgressBar::new_spinner();
  pb.enable_steady_tick(Duration::from_millis(60));
  pb.set_style(
    ProgressStyle::with_template("{spinner:.blue}")
      .unwrap()
      .tick_strings(&[
        "⢀⠀", "⡀⠀", "⠄⠀", "⢂⠀", "⡂⠀", "⠅⠀", "⢃⠀", "⡃⠀", "⠍⠀", "⢋⠀", "⡋⠀", "⠍⠁", "⢋⠁", "⡋⠁", "⠍⠉",
        "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⢈⠩", "⡀⢙", "⠄⡙", "⢂⠩", "⡂⢘", "⠅⡘", "⢃⠨", "⡃⢐",
        "⠍⡐", "⢋⠠", "⡋⢀", "⠍⡁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⠈⠩",
        "⠀⢙", "⠀⡙", "⠀⠩", "⠀⢘", "⠀⡘", "⠀⠨", "⠀⢐", "⠀⡐", "⠀⠠", "⠀⢀", "⠀⡀",
      ]),
  );
  pb.set_message("Querying...");
  pb
}

pub(super) async fn start_cli_with_spinner(
  config: AppConfig,
  reqwest_client: Client,
  cancellation_token: CancellationToken,
  app: Arc<Mutex<App<'_>>>,
  command: Command,
) {
  config.verify_config_present_for_cli(&command);
  app.lock().await.cli_mode = true;
  let pb = render_spinner();
  let app_nw = Arc::clone(&app);
  let mut network = Network::new(&app_nw, cancellation_token, reqwest_client);
  match cli::handle_command(&app, command, &mut network).await {
    Ok(output) => {
      pb.finish();
      println!("{}", output);
    }
    Err(e) => {
      pb.finish();
      eprintln!("error: {}", e.to_string().red());
      process::exit(1);
    }
  }
}

pub(super) async fn start_cli_no_spinner(
  config: AppConfig,
  reqwest_client: Client,
  cancellation_token: CancellationToken,
  app: Arc<Mutex<App<'_>>>,
  command: Command,
) {
  config.verify_config_present_for_cli(&command);
  app.lock().await.cli_mode = true;
  let app_nw = Arc::clone(&app);
  let mut network = Network::new(&app_nw, cancellation_token, reqwest_client);
  match cli::handle_command(&app, command, &mut network).await {
    Ok(output) => {
      println!("{}", output);
    }
    Err(e) => {
      eprintln!("error: {}", e.to_string().red());
      process::exit(1);
    }
  }
}
