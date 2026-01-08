use std::sync::Arc;

use anyhow::Result;
use clap::{ArgAction, Subcommand, arg};
use tokio::sync::Mutex;

use super::LidarrCommand;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::lidarr_models::{AddArtistBody, AddArtistOptions, MonitorType, NewItemMonitorType},
  network::{NetworkTrait, lidarr_network::LidarrEvent},
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
      help = "The MusicBrainz foreign artist ID of the artist you wish to add to your library",
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
      help = "What Lidarr should monitor for this artist",
      value_enum,
      default_value_t = MonitorType::default()
    )]
    monitor: MonitorType,
    #[arg(
      long,
      help = "How Lidarr should monitor new items for this artist",
      value_enum,
      default_value_t = NewItemMonitorType::default()
    )]
    monitor_new_items: NewItemMonitorType,
    #[arg(
      long,
      help = "Tell Lidarr to not start a search for missing albums once the artist is added to your library"
    )]
    no_search_for_missing_albums: bool,
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
    app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrAddCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrAddCommandHandler {
      _app: app,
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
        monitor_new_items,
        no_search_for_missing_albums,
      } => {
        let body = AddArtistBody {
          foreign_artist_id,
          artist_name,
          monitored: !disable_monitoring,
          root_folder_path,
          quality_profile_id,
          metadata_profile_id,
          tags,
          tag_input_string: None,
          add_options: AddArtistOptions {
            monitor,
            monitor_new_items,
            search_for_missing_albums: !no_search_for_missing_albums,
          },
        };
        let resp = self
          .network
          .handle_network_event(LidarrEvent::AddArtist(body).into())
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
