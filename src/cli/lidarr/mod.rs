use std::sync::Arc;

use anyhow::Result;
use clap::{Subcommand, arg};
use delete_command_handler::{LidarrDeleteCommand, LidarrDeleteCommandHandler};
use list_command_handler::{LidarrListCommand, LidarrListCommandHandler};
use tokio::sync::Mutex;

use crate::network::lidarr_network::LidarrEvent;
use crate::{app::App, network::NetworkTrait};

use super::{CliCommandHandler, Command};

mod delete_command_handler;
mod list_command_handler;

#[cfg(test)]
#[path = "lidarr_command_tests.rs"]
mod lidarr_command_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrCommand {
  #[command(
    subcommand,
    about = "Commands to delete resources from your Lidarr instance"
  )]
  Delete(LidarrDeleteCommand),
  #[command(
    subcommand,
    about = "Commands to list attributes from your Lidarr instance"
  )]
  List(LidarrListCommand),
  #[command(
    about = "Toggle monitoring for the specified artist corresponding to the given artist ID"
  )]
  ToggleArtistMonitoring {
    #[arg(
      long,
      help = "The Lidarr ID of the artist to toggle monitoring on",
      required = true
    )]
    artist_id: i64,
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
      LidarrCommand::Delete(delete_command) => {
        LidarrDeleteCommandHandler::with(self.app, delete_command, self.network)
          .handle()
          .await?
      }
      LidarrCommand::List(list_command) => {
        LidarrListCommandHandler::with(self.app, list_command, self.network)
          .handle()
          .await?
      }
      LidarrCommand::ToggleArtistMonitoring { artist_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::ToggleArtistMonitoring(artist_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
