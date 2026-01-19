use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::lidarr_models::DeleteParams,
  network::{NetworkTrait, lidarr_network::LidarrEvent},
};

use super::LidarrCommand;

#[cfg(test)]
#[path = "delete_command_handler_tests.rs"]
mod delete_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrDeleteCommand {
  #[command(about = "Delete an album from your Lidarr library")]
  Album {
    #[arg(long, help = "The ID of the album to delete", required = true)]
    album_id: i64,
    #[arg(long, help = "Delete the album files from disk as well")]
    delete_files_from_disk: bool,
    #[arg(long, help = "Add a list exclusion for this album")]
    add_list_exclusion: bool,
  },
  #[command(about = "Delete the specified item from the Lidarr blocklist")]
  BlocklistItem {
    #[arg(
      long,
      help = "The ID of the blocklist item to remove from the blocklist",
      required = true
    )]
    blocklist_item_id: i64,
  },
  #[command(about = "Delete the specified track file from disk")]
  TrackFile {
    #[arg(long, help = "The ID of the track file to delete", required = true)]
    track_file_id: i64,
  },
  #[command(about = "Delete an artist from your Lidarr library")]
  Artist {
    #[arg(long, help = "The ID of the artist to delete", required = true)]
    artist_id: i64,
    #[arg(long, help = "Delete the artist files from disk as well")]
    delete_files_from_disk: bool,
    #[arg(long, help = "Add a list exclusion for this artist")]
    add_list_exclusion: bool,
  },
  #[command(about = "Delete the specified download")]
  Download {
    #[arg(long, help = "The ID of the download to delete", required = true)]
    download_id: i64,
  },
  #[command(about = "Delete the indexer with the given ID")]
  Indexer {
    #[arg(long, help = "The ID of the indexer to delete", required = true)]
    indexer_id: i64,
  },
  #[command(about = "Delete the root folder with the given ID")]
  RootFolder {
    #[arg(long, help = "The ID of the root folder to delete", required = true)]
    root_folder_id: i64,
  },
  #[command(about = "Delete the tag with the specified ID")]
  Tag {
    #[arg(long, help = "The ID of the tag to delete", required = true)]
    tag_id: i64,
  },
}

impl From<LidarrDeleteCommand> for Command {
  fn from(value: LidarrDeleteCommand) -> Self {
    Command::Lidarr(LidarrCommand::Delete(value))
  }
}

pub(super) struct LidarrDeleteCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrDeleteCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrDeleteCommand> for LidarrDeleteCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrDeleteCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrDeleteCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrDeleteCommand::Album {
        album_id,
        delete_files_from_disk,
        add_list_exclusion,
      } => {
        let delete_album_params = DeleteParams {
          id: album_id,
          delete_files: delete_files_from_disk,
          add_import_list_exclusion: add_list_exclusion,
        };
        let resp = self
          .network
          .handle_network_event(LidarrEvent::DeleteAlbum(delete_album_params).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrDeleteCommand::BlocklistItem { blocklist_item_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::DeleteBlocklistItem(blocklist_item_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrDeleteCommand::TrackFile { track_file_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::DeleteTrackFile(track_file_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrDeleteCommand::Artist {
        artist_id,
        delete_files_from_disk,
        add_list_exclusion,
      } => {
        let delete_artist_params = DeleteParams {
          id: artist_id,
          delete_files: delete_files_from_disk,
          add_import_list_exclusion: add_list_exclusion,
        };
        let resp = self
          .network
          .handle_network_event(LidarrEvent::DeleteArtist(delete_artist_params).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrDeleteCommand::Download { download_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::DeleteDownload(download_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrDeleteCommand::Indexer { indexer_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::DeleteIndexer(indexer_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrDeleteCommand::RootFolder { root_folder_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::DeleteRootFolder(root_folder_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrDeleteCommand::Tag { tag_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::DeleteTag(tag_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
