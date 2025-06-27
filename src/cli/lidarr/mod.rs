use std::sync::Arc;

pub mod add_command_handler;
pub mod delete_command_handler;
// TODO
// pub mod download_command_handler;
pub mod edit_command_handler;
pub mod get_command_handler;
pub mod list_command_handler;
pub mod manual_search_command_handler;
pub mod refresh_command_handler;
pub mod trigger_automatic_search_command_handler;

use add_command_handler::{LidarrAddCommand, LidarrAddCommandHandler};
use anyhow::Result;
use clap::Subcommand;
use delete_command_handler::{LidarrDeleteCommand, LidarrDeleteCommandHandler};
// TODO
// use download_command_handler::{LidarrDownloadCommand, LidarrDownloadCommandHandler};
use edit_command_handler::{LidarrEditCommand, LidarrEditCommandHandler};
use get_command_handler::{LidarrGetCommand, LidarrGetCommandHandler};
use list_command_handler::{LidarrListCommand, LidarrListCommandHandler};
use manual_search_command_handler::{LidarrManualSearchCommand, LidarrManualSearchCommandHandler};
use refresh_command_handler::{LidarrRefreshCommand, LidarrRefreshCommandHandler};
use tokio::sync::Mutex;
use trigger_automatic_search_command_handler::{
  LidarrTriggerAutomaticSearchCommand, LidarrTriggerAutomaticSearchCommandHandler,
};

use crate::{
  app::App,
  models::lidarr_models::LidarrTaskName,
  network::{lidarr_network::LidarrEvent, NetworkTrait},
};

use super::{CliCommandHandler, Command};

#[cfg(test)]
#[path = "lidarr_command_tests.rs"]
mod lidarr_command_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrCommand {
  #[command(
    subcommand,
    about = "Commands to add or create new resources within your Lidarr instance"
  )]
  Add(LidarrAddCommand),
  #[command(
    subcommand,
    about = "Commands to delete resources from your Lidarr instance"
  )]
  Delete(LidarrDeleteCommand),
  #[command(
    subcommand,
    about = "Commands to edit resources in your Lidarr instance"
  )]
  Edit(LidarrEditCommand),
  #[command(
    subcommand,
    about = "Commands to fetch details of the resources in your Lidarr instance"
  )]
  Get(LidarrGetCommand),
  #[command(
    subcommand,
    about = "Commands to download releases in your Lidarr instance"
  )]
  // TODO
  // Download(LidarrDownloadCommand),
  // #[command(
  //   subcommand,
  //   about = "Commands to list attributes from your Lidarr instance"
  // )]
  List(LidarrListCommand),
  #[command(
    subcommand,
    about = "Commands to refresh the data in your Lidarr instance"
  )]
  Refresh(LidarrRefreshCommand),
  #[command(subcommand, about = "Commands to manually search for releases")]
  ManualSearch(LidarrManualSearchCommand),
  #[command(
    subcommand,
    about = "Commands to trigger automatic searches for releases of different resources in your Lidarr instance"
  )]
  TriggerAutomaticSearch(LidarrTriggerAutomaticSearchCommand),
  #[command(about = "Clear the blocklist")]
  ClearBlocklist,
  #[command(about = "Mark the Lidarr history item with the given ID as 'failed'")]
  MarkHistoryItemAsFailed {
    #[arg(
      long,
      help = "The Lidarr ID of the history item you wish to mark as 'failed'",
      required = true
    )]
    history_item_id: i64,
  },
  #[command(about = "Search for a new artist to add to Lidarr")]
  SearchNewArtist {
    #[arg(
      long,
      help = "The name of the artist you want to search for",
      required = true
    )]
    query: String,
  },
  #[command(about = "Start the specified Lidarr task")]
  StartTask {
    #[arg(
      long,
      help = "The name of the task to trigger",
      value_enum,
      required = true
    )]
    task_name: LidarrTaskName,
  },
  #[command(
    about = "Test the indexer with the given ID. Note that a successful test returns an empty JSON body; i.e. '{}'"
  )]
  TestIndexer {
    #[arg(long, help = "The ID of the indexer to test", required = true)]
    indexer_id: i64,
  },
  #[command(about = "Test all Lidarr indexers")]
  TestAllIndexers,
  #[command(about = "Toggle monitoring for the specified track")]
  ToggleTrackMonitoring {
    #[arg(
      long,
      help = "The Lidarr ID of the track to toggle monitoring on",
      required = true
    )]
    track_id: i64,
  },
  #[command(
    about = "Toggle monitoring for the specified album that corresponds to the specified artist ID"
  )]
  ToggleAlbumMonitoring {
    #[arg(
      long,
      help = "The Lidarr ID of the artist that the album belongs to",
      required = true
    )]
    artist_id: i64,
    #[arg(
      long,
      help = "The album ID to toggle monitoring for",
      required = true
    )]
    album_id: i64,
  },
}

impl From<LidarrCommand> for Command {
  fn from(lidarr_command: LidarrCommand) -> Command {
    Command::Lidarr(lidarr_command)
  }
}

pub(super) struct LidarrCliHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrCommand> for LidarrCliHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrCliHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrCommand::Add(add_command) => {
        LidarrAddCommandHandler::with(self.app, add_command, self.network)
          .handle()
          .await?
      }
      LidarrCommand::Delete(delete_command) => {
        LidarrDeleteCommandHandler::with(self.app, delete_command, self.network)
          .handle()
          .await?
      }
      LidarrCommand::Edit(edit_command) => {
        LidarrEditCommandHandler::with(self.app, edit_command, self.network)
          .handle()
          .await?
      }
      // TODO
      // LidarrCommand::Download(download_command) => {
      //   LidarrDownloadCommandHandler::with(self.app, download_command, self.network)
      //     .handle()
      //     .await?
      // }
      LidarrCommand::Get(get_command) => {
        LidarrGetCommandHandler::with(self.app, get_command, self.network)
          .handle()
          .await?
      }
      LidarrCommand::List(list_command) => {
        LidarrListCommandHandler::with(self.app, list_command, self.network)
          .handle()
          .await?
      }
      LidarrCommand::Refresh(refresh_command) => {
        LidarrRefreshCommandHandler::with(self.app, refresh_command, self.network)
          .handle()
          .await?
      }
      LidarrCommand::ManualSearch(manual_search_command) => {
        LidarrManualSearchCommandHandler::with(self.app, manual_search_command, self.network)
          .handle()
          .await?
      }
      LidarrCommand::TriggerAutomaticSearch(trigger_automatic_search_command) => {
        LidarrTriggerAutomaticSearchCommandHandler::with(
          self.app,
          trigger_automatic_search_command,
          self.network,
        )
        .handle()
        .await?
      }
      LidarrCommand::ClearBlocklist => {
        self
          .network
          .handle_network_event(LidarrEvent::GetBlocklist.into())
          .await?;
        let resp = self
          .network
          .handle_network_event(LidarrEvent::ClearBlocklist.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrCommand::MarkHistoryItemAsFailed { history_item_id } => {
        let _ = self
          .network
          .handle_network_event(LidarrEvent::MarkHistoryItemAsFailed(history_item_id).into())
          .await?;
        "Lidarr history item marked as 'failed'".to_owned()
      }
      LidarrCommand::SearchNewArtist { query } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::SearchNewArtist(query).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrCommand::StartTask { task_name } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::StartTask(task_name).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrCommand::TestIndexer { indexer_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::TestIndexer(indexer_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrCommand::TestAllIndexers => {
        println!("Testing all Lidarr indexers. This may take a minute...");
        let resp = self
          .network
          .handle_network_event(LidarrEvent::TestAllIndexers.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrCommand::ToggleTrackMonitoring { track_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::ToggleTrackMonitoring(track_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrCommand::ToggleAlbumMonitoring {
        artist_id,
        album_id,
      } => {
        let resp = self
          .network
          .handle_network_event(
            LidarrEvent::ToggleAlbumMonitoring((artist_id, album_id)).into(),
          )
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
