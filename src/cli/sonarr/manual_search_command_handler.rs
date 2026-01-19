use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;
use tokio::sync::Mutex;

use super::SonarrCommand;
use crate::models::Serdeable;
use crate::models::sonarr_models::{SonarrRelease, SonarrSerdeable};
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, sonarr_network::SonarrEvent},
};

#[cfg(test)]
#[path = "manual_search_command_handler_tests.rs"]
mod manual_search_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrManualSearchCommand {
  #[command(about = "Trigger a manual search of releases for the episode with the given ID")]
  Episode {
    #[arg(
      long,
      help = "The Sonarr ID of the episode whose releases you wish to fetch and list",
      required = true
    )]
    episode_id: i64,
  },
  #[command(
    about = "Trigger a manual search of releases for the given season corresponding to the series with the given ID"
  )]
  Season {
    #[arg(
      long,
      help = "The Sonarr ID of the series whose releases you wish to fetch and list",
      required = true
    )]
    series_id: i64,
    #[arg(long, help = "The season number to search for", required = true)]
    season_number: i64,
  },
}

impl From<SonarrManualSearchCommand> for Command {
  fn from(value: SonarrManualSearchCommand) -> Self {
    Command::Sonarr(SonarrCommand::ManualSearch(value))
  }
}

pub(super) struct SonarrManualSearchCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrManualSearchCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrManualSearchCommand>
  for SonarrManualSearchCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrManualSearchCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrManualSearchCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      SonarrManualSearchCommand::Episode { episode_id } => {
        println!("Searching for episode releases. This may take a minute...");
        match self
          .network
          .handle_network_event(SonarrEvent::GetEpisodeReleases(episode_id).into())
          .await
        {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Releases(releases_vec))) => {
            let seasons_vec: Vec<SonarrRelease> = releases_vec
              .into_iter()
              .filter(|release| !release.full_season)
              .collect();
            serde_json::to_string_pretty(&seasons_vec)?
          }
          Err(e) => return Err(e),
          _ => serde_json::to_string_pretty(&json!({"message": "Failed to parse response"}))?,
        }
      }
      SonarrManualSearchCommand::Season {
        series_id,
        season_number,
      } => {
        println!("Searching for season releases. This may take a minute...");
        match self
          .network
          .handle_network_event(SonarrEvent::GetSeasonReleases(series_id, season_number).into())
          .await
        {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Releases(releases_vec))) => {
            let seasons_vec: Vec<SonarrRelease> = releases_vec
              .into_iter()
              .filter(|release| release.full_season)
              .collect();
            serde_json::to_string_pretty(&seasons_vec)?
          }
          Err(e) => return Err(e),
          _ => serde_json::to_string_pretty(&json!({"message": "Failed to parse response"}))?,
        }
      }
    };

    Ok(result)
  }
}
