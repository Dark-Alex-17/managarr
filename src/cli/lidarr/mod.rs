use std::sync::Arc;

use add_command_handler::{LidarrAddCommand, LidarrAddCommandHandler};
use anyhow::Result;
use clap::{Subcommand, arg};
use delete_command_handler::{LidarrDeleteCommand, LidarrDeleteCommandHandler};
use edit_command_handler::{LidarrEditCommand, LidarrEditCommandHandler};
use get_command_handler::{LidarrGetCommand, LidarrGetCommandHandler};
use list_command_handler::{LidarrListCommand, LidarrListCommandHandler};
use refresh_command_handler::{LidarrRefreshCommand, LidarrRefreshCommandHandler};
use tokio::sync::Mutex;

use crate::network::lidarr_network::LidarrEvent;
use crate::{app::App, network::NetworkTrait};

use super::{CliCommandHandler, Command};

mod add_command_handler;
mod delete_command_handler;
mod edit_command_handler;
mod get_command_handler;
mod list_command_handler;
mod refresh_command_handler;

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
    about = "Commands to list attributes from your Lidarr instance"
  )]
  List(LidarrListCommand),
  #[command(
    subcommand,
    about = "Commands to refresh the data in your Lidarr instance"
  )]
  Refresh(LidarrRefreshCommand),
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
