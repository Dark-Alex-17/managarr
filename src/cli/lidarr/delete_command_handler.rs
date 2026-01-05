use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::lidarr_models::DeleteArtistParams,
  network::{NetworkTrait, lidarr_network::LidarrEvent},
};

use super::LidarrCommand;

#[cfg(test)]
#[path = "delete_command_handler_tests.rs"]
mod delete_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrDeleteCommand {
  #[command(about = "Delete an artist from your Lidarr library")]
  Artist {
    #[arg(long, help = "The ID of the artist to delete", required = true)]
    artist_id: i64,
    #[arg(long, help = "Delete the artist files from disk as well")]
    delete_files_from_disk: bool,
    #[arg(long, help = "Add a list exclusion for this artist")]
    add_list_exclusion: bool,
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
      LidarrDeleteCommand::Artist {
        artist_id,
        delete_files_from_disk,
        add_list_exclusion,
      } => {
        let delete_artist_params = DeleteArtistParams {
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
    };

    Ok(result)
  }
}
