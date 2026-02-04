use std::sync::Arc;

use anyhow::Result;
use clap::{Subcommand, command};
use clap_complete::Shell;
use indoc::indoc;
use lidarr::{LidarrCliHandler, LidarrCommand};
use radarr::{RadarrCliHandler, RadarrCommand};
use sonarr::{SonarrCliHandler, SonarrCommand};
use tokio::sync::Mutex;

use crate::{app::App, network::NetworkTrait};

pub mod lidarr;
pub mod radarr;
pub mod sonarr;

#[cfg(test)]
#[path = "cli_tests.rs"]
mod cli_tests;

#[derive(Debug, Clone, Subcommand, PartialEq, Eq)]
pub enum Command {
  #[command(subcommand, about = "Commands for manging your Radarr instance")]
  Radarr(RadarrCommand),

  #[command(subcommand, about = "Commands for manging your Sonarr instance")]
  Sonarr(SonarrCommand),

  #[command(subcommand, about = "Commands for manging your Lidarr instance")]
  Lidarr(LidarrCommand),

  #[command(
    arg_required_else_help = true,
    about = "Generate shell completions for the Managarr CLI"
  )]
  Completions {
    #[arg(value_enum)]
    shell: Shell,
  },

  #[command(about = "Tail Managarr logs")]
  TailLogs {
    #[arg(long, help = "Disable colored log output")]
    no_color: bool,
  },

  #[command(about = indoc!{"
      Print the full path to the default configuration file.
      This file can be changed to another location using the '--config-file' flag
    "})]
  ConfigPath,
}

pub trait CliCommandHandler<'a, 'b, T: Into<Command>> {
  fn with(app: &'a Arc<Mutex<App<'b>>>, command: T, network: &'a mut dyn NetworkTrait) -> Self;
  async fn handle(self) -> Result<String>;
}

pub(crate) async fn handle_command(
  app: &Arc<Mutex<App<'_>>>,
  command: Command,
  network: &mut dyn NetworkTrait,
) -> Result<String> {
  let result = match command {
    Command::Radarr(radarr_command) => {
      RadarrCliHandler::with(app, radarr_command, network)
        .handle()
        .await?
    }
    Command::Sonarr(sonarr_command) => {
      SonarrCliHandler::with(app, sonarr_command, network)
        .handle()
        .await?
    }
    Command::Lidarr(lidarr_command) => {
      LidarrCliHandler::with(app, lidarr_command, network)
        .handle()
        .await?
    }
    _ => String::new(),
  };

  Ok(result)
}

#[inline]
pub fn mutex_flags_or_option(positive: bool, negative: bool) -> Option<bool> {
  if positive {
    Some(true)
  } else if negative {
    Some(false)
  } else {
    None
  }
}

#[inline]
pub fn mutex_flags_or_default(positive: bool, negative: bool, default_value: bool) -> bool {
  if positive {
    true
  } else if negative {
    false
  } else {
    default_value
  }
}
