use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::sonarr_models::DeleteSeriesParams,
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "delete_command_handler_tests.rs"]
mod delete_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrDeleteCommand {
  #[command(about = "Delete the specified item from the Sonarr blocklist")]
  BlocklistItem {
    #[arg(
      long,
      help = "The ID of the blocklist item to remove from the blocklist",
      required = true
    )]
    blocklist_item_id: i64,
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
  #[command(about = "Delete a series from your Sonarr library")]
  Series {
    #[arg(long, help = "The ID of the series to delete", required = true)]
    series_id: i64,
    #[arg(long, help = "Delete the series files from disk as well")]
    delete_files_from_disk: bool,
    #[arg(long, help = "Add a list exclusion for this series")]
    add_list_exclusion: bool,
  },
  #[command(about = "Delete the tag with the specified ID")]
  Tag {
    #[arg(long, help = "The ID of the tag to delete", required = true)]
    tag_id: i64,
  },
}

impl From<SonarrDeleteCommand> for Command {
  fn from(value: SonarrDeleteCommand) -> Self {
    Command::Sonarr(SonarrCommand::Delete(value))
  }
}

pub(super) struct SonarrDeleteCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrDeleteCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrDeleteCommand> for SonarrDeleteCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrDeleteCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrDeleteCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let resp = match self.command {
      SonarrDeleteCommand::BlocklistItem { blocklist_item_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::DeleteBlocklistItem(Some(blocklist_item_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrDeleteCommand::Download { download_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::DeleteDownload(Some(download_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrDeleteCommand::Indexer { indexer_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::DeleteIndexer(Some(indexer_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrDeleteCommand::RootFolder { root_folder_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::DeleteRootFolder(Some(root_folder_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrDeleteCommand::Series {
        series_id,
        delete_files_from_disk,
        add_list_exclusion,
      } => {
        let delete_series_params = DeleteSeriesParams {
          id: series_id,
          delete_series_files: delete_files_from_disk,
          add_list_exclusion,
        };
        let resp = self
          .network
          .handle_network_event(SonarrEvent::DeleteSeries(Some(delete_series_params)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrDeleteCommand::Tag { tag_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::DeleteTag(tag_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(resp)
  }
}
