use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "trigger_automatic_search_command_handler_tests.rs"]
mod trigger_automatic_search_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrTriggerAutomaticSearchCommand {
  #[command(about = "Trigger an automatic search for the series with the specified ID")]
  Series {
    #[arg(
      long,
      help = "The ID of the series you want to trigger an automatic search for",
      required = true
    )]
    series_id: i64,
  },
  #[command(
    about = "Trigger an automatic search for the given season corresponding to the series with the given ID"
  )]
  Season {
    #[arg(
      long,
      help = "The Sonarr ID of the series whose season you wish to trigger an automatic search for",
      required = true
    )]
    series_id: i64,
    #[arg(long, help = "The season number to search for", required = true)]
    season_number: i64,
  },
  #[command(about = "Trigger an automatic search for the episode with the specified ID")]
  Episode {
    #[arg(
      long,
      help = "The ID of the episode you want to trigger an automatic search for",
      required = true
    )]
    episode_id: i64,
  },
}

impl From<SonarrTriggerAutomaticSearchCommand> for Command {
  fn from(value: SonarrTriggerAutomaticSearchCommand) -> Self {
    Command::Sonarr(SonarrCommand::TriggerAutomaticSearch(value))
  }
}

pub(super) struct SonarrTriggerAutomaticSearchCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrTriggerAutomaticSearchCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrTriggerAutomaticSearchCommand>
  for SonarrTriggerAutomaticSearchCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrTriggerAutomaticSearchCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrTriggerAutomaticSearchCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      SonarrTriggerAutomaticSearchCommand::Series { series_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::TriggerAutomaticSeriesSearch(Some(series_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrTriggerAutomaticSearchCommand::Season {
        series_id,
        season_number,
      } => {
        let resp = self
          .network
          .handle_network_event(
            SonarrEvent::TriggerAutomaticSeasonSearch(Some((series_id, season_number))).into(),
          )
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrTriggerAutomaticSearchCommand::Episode { episode_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::TriggerAutomaticEpisodeSearch(episode_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
