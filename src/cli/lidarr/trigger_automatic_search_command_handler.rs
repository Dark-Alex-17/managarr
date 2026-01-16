use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, lidarr_network::LidarrEvent},
};

use super::LidarrCommand;

#[cfg(test)]
#[path = "trigger_automatic_search_command_handler_tests.rs"]
mod trigger_automatic_search_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrTriggerAutomaticSearchCommand {
  #[command(about = "Trigger an automatic search for the album with the specified ID")]
  Album {
    #[arg(
      long,
      help = "The Lidarr ID of the album you want to trigger an automatic search for",
      required = true
    )]
    album_id: i64,
  },
  #[command(about = "Trigger an automatic search for the artist with the specified ID")]
  Artist {
    #[arg(
      long,
      help = "The ID of the artist you want to trigger an automatic search for",
      required = true
    )]
    artist_id: i64,
  },
}

impl From<LidarrTriggerAutomaticSearchCommand> for Command {
  fn from(value: LidarrTriggerAutomaticSearchCommand) -> Self {
    Command::Lidarr(LidarrCommand::TriggerAutomaticSearch(value))
  }
}

pub(super) struct LidarrTriggerAutomaticSearchCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrTriggerAutomaticSearchCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrTriggerAutomaticSearchCommand>
  for LidarrTriggerAutomaticSearchCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrTriggerAutomaticSearchCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrTriggerAutomaticSearchCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrTriggerAutomaticSearchCommand::Album { album_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::TriggerAutomaticAlbumSearch(album_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrTriggerAutomaticSearchCommand::Artist { artist_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::TriggerAutomaticArtistSearch(artist_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
