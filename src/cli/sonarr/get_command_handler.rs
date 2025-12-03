use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, sonarr_network::SonarrEvent},
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

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      SonarrGetCommand::AllIndexerSettings => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetAllIndexerSettings.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrGetCommand::EpisodeDetails { episode_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetEpisodeDetails(episode_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrGetCommand::HostConfig => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetHostConfig.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrGetCommand::SecurityConfig => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetSecurityConfig.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrGetCommand::SeriesDetails { series_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetSeriesDetails(series_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrGetCommand::SystemStatus => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetStatus.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
