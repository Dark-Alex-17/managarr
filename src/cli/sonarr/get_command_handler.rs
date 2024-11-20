use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  execute_network_event,
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "get_command_handler_tests.rs"]
mod get_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrGetCommand {
  #[command(about = "Get the shared settings for all indexers")]
  AllIndexerSettings,
  #[command(about = "Get detailed information for the episode with the given ID")]
  EpisodeDetails {
    #[arg(
      long,
      help = "The Sonarr ID of the episode whose details you wish to fetch",
      required = true
    )]
    episode_id: i64,
  },
  #[command(about = "Fetch the host config for your Sonarr instance")]
  HostConfig,
  #[command(about = "Fetch the security config for your Sonarr instance")]
  SecurityConfig,
  #[command(about = "Get detailed information for the series with the given ID")]
  SeriesDetails {
    #[arg(
      long,
      help = "The Sonarr ID of the series whose details you wish to fetch",
      required = true
    )]
    series_id: i64,
  },
  #[command(about = "Get the system status")]
  SystemStatus,
}

impl From<SonarrGetCommand> for Command {
  fn from(value: SonarrGetCommand) -> Self {
    Command::Sonarr(SonarrCommand::Get(value))
  }
}

pub(super) struct SonarrGetCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrGetCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrGetCommand> for SonarrGetCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrGetCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrGetCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<()> {
    match self.command {
      SonarrGetCommand::AllIndexerSettings => {
        execute_network_event!(self, SonarrEvent::GetAllIndexerSettings);
      }
      SonarrGetCommand::EpisodeDetails { episode_id } => {
        execute_network_event!(self, SonarrEvent::GetEpisodeDetails(Some(episode_id)));
      }
      SonarrGetCommand::HostConfig => {
        execute_network_event!(self, SonarrEvent::GetHostConfig);
      }
      SonarrGetCommand::SecurityConfig => {
        execute_network_event!(self, SonarrEvent::GetSecurityConfig);
      }
      SonarrGetCommand::SeriesDetails { series_id } => {
        execute_network_event!(self, SonarrEvent::GetSeriesDetails(Some(series_id)));
      }
      SonarrGetCommand::SystemStatus => {
        execute_network_event!(self, SonarrEvent::GetStatus);
      }
    }

    Ok(())
  }
}
