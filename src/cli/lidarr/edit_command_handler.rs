use std::sync::Arc;

use anyhow::Result;
use clap::{ArgAction, ArgGroup, Subcommand};
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command, mutex_flags_or_option},
  models::lidarr_models::{EditArtistParams, NewItemMonitorType},
  network::{NetworkTrait, lidarr_network::LidarrEvent},
};

use super::LidarrCommand;

#[cfg(test)]
#[path = "edit_command_handler_tests.rs"]
mod edit_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrEditCommand {
  #[command(
    about = "Edit preferences for the specified artist",
    group(
      ArgGroup::new("edit_artist")
      .args([
        "enable_monitoring",
        "disable_monitoring",
        "monitor_new_items",
        "quality_profile_id",
        "metadata_profile_id",
        "root_folder_path",
        "tag",
        "clear_tags"
      ]).required(true)
      .multiple(true))
  )]
  Artist {
    #[arg(
      long,
      help = "The ID of the artist whose settings you want to edit",
      required = true
    )]
    artist_id: i64,
    #[arg(
      long,
      help = "Enable monitoring of this artist in Lidarr so Lidarr will automatically download releases from this artist if they are available",
      conflicts_with = "disable_monitoring"
    )]
    enable_monitoring: bool,
    #[arg(
      long,
      help = "Disable monitoring of this artist so Lidarr does not automatically download releases from this artist if they are available",
      conflicts_with = "enable_monitoring"
    )]
    disable_monitoring: bool,
    #[arg(
      long,
      help = "How Lidarr should monitor new albums from this artist",
      value_enum
    )]
    monitor_new_items: Option<NewItemMonitorType>,
    #[arg(long, help = "The ID of the quality profile to use for this artist")]
    quality_profile_id: Option<i64>,
    #[arg(long, help = "The ID of the metadata profile to use for this artist")]
    metadata_profile_id: Option<i64>,
    #[arg(
      long,
      help = "The root folder path where all artist data and metadata should live"
    )]
    root_folder_path: Option<String>,
    #[arg(
      long,
      help = "Tag IDs to tag this artist with",
      value_parser,
      action = ArgAction::Append,
      conflicts_with = "clear_tags"
    )]
    tag: Option<Vec<i64>>,
    #[arg(long, help = "Clear all tags on this artist", conflicts_with = "tag")]
    clear_tags: bool,
  },
}

impl From<LidarrEditCommand> for Command {
  fn from(value: LidarrEditCommand) -> Self {
    Command::Lidarr(LidarrCommand::Edit(value))
  }
}

pub(super) struct LidarrEditCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrEditCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrEditCommand> for LidarrEditCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrEditCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrEditCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrEditCommand::Artist {
        artist_id,
        enable_monitoring,
        disable_monitoring,
        monitor_new_items,
        quality_profile_id,
        metadata_profile_id,
        root_folder_path,
        tag,
        clear_tags,
      } => {
        let monitored_value = mutex_flags_or_option(enable_monitoring, disable_monitoring);
        let edit_artist_params = EditArtistParams {
          artist_id,
          monitored: monitored_value,
          monitor_new_items,
          quality_profile_id,
          metadata_profile_id,
          root_folder_path,
          tags: tag,
          tag_input_string: None,
          clear_tags,
        };

        self
          .network
          .handle_network_event(LidarrEvent::EditArtist(edit_artist_params).into())
          .await?;
        "Artist Updated".to_owned()
      }
    };

    Ok(result)
  }
}
