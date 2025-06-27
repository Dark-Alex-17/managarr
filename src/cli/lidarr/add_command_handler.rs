use std::sync::Arc;

use anyhow::Result;
use clap::{ArgAction, Subcommand};
use tokio::sync::Mutex;

use super::LidarrCommand;
use crate::models::servarr_models::AddRootFolderBody;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::lidarr_models::{AddArtistBody, AddArtistOptions, ArtistStatus},
  network::{lidarr_network::LidarrEvent, NetworkTrait},
};

#[cfg(test)]
#[path = "add_command_handler_tests.rs"]
mod add_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrAddCommand {
  #[command(about = "Add a new artist to your Lidarr library")]
  Artist {
    #[arg(
      long,
      help = "The MusicBrainz ID of the artist you wish to add to your library",
      required = true
    )]
    foreign_artist_id: String,
    #[arg(long, help = "The name of the artist", required = true)]
    artist_name: String,
    #[arg(
      long,
      help = "The root folder path where all artist data and metadata should live",
      required = true
    )]
    root_folder_path: String,
    #[arg(
      long,
      help = "The ID of the quality profile to use for this artist",
      required = true
    )]
    quality_profile_id: i64,
    #[arg(
      long,
      help = "The ID of the metadata profile to use for this artist",
      required = true
    )]
    metadata_profile_id: i64,
    #[arg(long, help = "Disable monitoring for this artist")]
    disable_monitoring: bool,
    #[arg(
      long,
      help = "Tag IDs to tag the artist with", 
      value_parser,
      action = ArgAction::Append
    )]
    tag: Vec<i64>,
    #[arg(
      long,
      help = "What Lidarr should monitor", 
      value_enum,
      default_value_t = ArtistStatus::default()
    )]
    monitor: ArtistStatus,
    #[arg(
      long,
      help = "Tell Lidarr to not start a search for this artist once it's added to your library"
    )]
    no_search_for_albums: bool,
  },
  #[command(about = "Add a new root folder")]
  RootFolder {
    #[arg(long, help = "The path of the new root folder", required = true)]
    root_folder_path: String,
  },
  #[command(about = "Add new tag")]
  Tag {
    #[arg(long, help = "The name of the tag to be added", required = true)]
    name: String,
  },
}

impl From<LidarrAddCommand> for Command {
  fn from(value: LidarrAddCommand) -> Self {
    Command::Lidarr(LidarrCommand::Add(value))
  }
}

pub(super) struct LidarrAddCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrAddCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrAddCommand> for LidarrAddCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrAddCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrAddCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrAddCommand::Artist {
        foreign_artist_id,
        artist_name,
        root_folder_path,
        quality_profile_id,
        metadata_profile_id,
        disable_monitoring,
        tag: tags,
        monitor,
        no_search_for_albums,
      } => {
        let body = AddArtistBody {
          foreign_artist_id,
          quality_profile_id,
          monitored: !disable_monitoring,
          root_folder_path,
          metadata_profile_id,
          tags,
          tag_input_string: None,
          add_options: AddArtistOptions {
            monitor: monitor.to_string(),
            search_for_missing_albums: !no_search_for_albums,
          },
        };
        let resp = self
          .network
          .handle_network_event(LidarrEvent::AddArtist(body).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrAddCommand::RootFolder { root_folder_path } => {
        let add_root_folder_body = AddRootFolderBody {
          path: root_folder_path,
        };
        let resp = self
          .network
          .handle_network_event(LidarrEvent::AddRootFolder(add_root_folder_body).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrAddCommand::Tag { name } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::AddTag(name).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };
    Ok(result)
  }
}
