use crate::app::App;
use crate::cli::lidarr::LidarrCommand;
use crate::cli::{CliCommandHandler, Command};
use crate::models::Serdeable;
use crate::models::lidarr_models::{LidarrRelease, LidarrSerdeable};
use crate::network::NetworkTrait;
use crate::network::lidarr_network::LidarrEvent;
use anyhow::Result;
use clap::Subcommand;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(test)]
#[path = "manual_search_command_handler_tests.rs"]
mod manual_search_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrManualSearchCommand {
  #[command(
    about = "Trigger a manual search of releases for the given album corresponding to the artist with the given ID"
  )]
  Album {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose releases you wish to fetch and list",
      required = true
    )]
    artist_id: i64,
    #[arg(long, help = "The Lidarr album ID to search for", required = true)]
    album_id: i64,
  },
  #[command(
    about = "Trigger a manual search of discography releases for the given artist corresponding to the artist with the given ID."
  )]
  Discography {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose discography releases you wish to fetch and list",
      required = true
    )]
    artist_id: i64,
  },
}

impl From<LidarrManualSearchCommand> for Command {
  fn from(value: LidarrManualSearchCommand) -> Self {
    Command::Lidarr(LidarrCommand::ManualSearch(value))
  }
}

pub(super) struct LidarrManualSearchCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrManualSearchCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrManualSearchCommand>
  for LidarrManualSearchCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrManualSearchCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrManualSearchCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrManualSearchCommand::Album {
        artist_id,
        album_id,
      } => {
        println!("Searching for album releases. This may take a minute...");
        match self
          .network
          .handle_network_event(LidarrEvent::GetAlbumReleases(artist_id, album_id).into())
          .await
        {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Releases(releases_vec))) => {
            let albums_vec: Vec<LidarrRelease> = releases_vec
              .into_iter()
              .filter(|release| !release.discography)
              .collect();
            serde_json::to_string_pretty(&albums_vec)?
          }
          Err(e) => return Err(e),
          _ => serde_json::to_string_pretty(&json!({"message": "Failed to parse response"}))?,
        }
      }
      LidarrManualSearchCommand::Discography { artist_id } => {
        println!("Searching for artist discography releases. This may take a minute...");
        match self
          .network
          .handle_network_event(LidarrEvent::GetDiscographyReleases(artist_id).into())
          .await
        {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Releases(releases_vec))) => {
            let discography_vec: Vec<LidarrRelease> = releases_vec
              .into_iter()
              .filter(|release| release.discography)
              .collect();
            serde_json::to_string_pretty(&discography_vec)?
          }
          Err(e) => return Err(e),
          _ => serde_json::to_string_pretty(&json!({"message": "Failed to parse response"}))?,
        }
      }
    };

    Ok(result)
  }
}
