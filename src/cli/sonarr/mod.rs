use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use delete_command_handler::{SonarrDeleteCommand, SonarrDeleteCommandHandler};
use get_command_handler::{SonarrGetCommand, SonarrGetCommandHandler};
use list_command_handler::{SonarrListCommand, SonarrListCommandHandler};
use tokio::sync::Mutex;

use crate::{
  app::App,
  execute_network_event,
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::{CliCommandHandler, Command};

mod delete_command_handler;
mod get_command_handler;
mod list_command_handler;

#[cfg(test)]
#[path = "sonarr_command_tests.rs"]
mod sonarr_command_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrCommand {
  #[command(
    subcommand,
    about = "Commands to delete resources from your Sonarr instance"
  )]
  Delete(SonarrDeleteCommand),
  #[command(
    subcommand,
    about = "Commands to fetch details of the resources in your Sonarr instance"
  )]
  Get(SonarrGetCommand),
  #[command(
    subcommand,
    about = "Commands to list attributes from your Sonarr instance"
  )]
  List(SonarrListCommand),
  #[command(about = "Clear the blocklist")]
  ClearBlocklist,
  #[command(about = "Trigger a manual search of releases for the episode with the given ID")]
  ManualEpisodeSearch {
    #[arg(
      long,
      help = "The Sonarr ID of the episode whose releases you wish to fetch and list",
      required = true
    )]
    episode_id: i64,
  },
  #[command(
    about = "Trigger a manual search of releases for the given season corresponding to the series with the given ID"
  )]
  ManualSeasonSearch {
    #[arg(
      long,
      help = "The Sonarr ID of the series whose releases you wish to fetch and list",
      required = true
    )]
    series_id: i64,
    #[arg(long, help = "The season number to search for", required = true)]
    season_number: i64,
  },
}

impl From<SonarrCommand> for Command {
  fn from(sonarr_command: SonarrCommand) -> Command {
    Command::Sonarr(sonarr_command)
  }
}

pub(super) struct SonarrCliHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrCommand> for SonarrCliHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrCliHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<()> {
    match self.command {
      SonarrCommand::Delete(delete_command) => {
        SonarrDeleteCommandHandler::with(self.app, delete_command, self.network)
          .handle()
          .await?
      }
      SonarrCommand::Get(get_command) => {
        SonarrGetCommandHandler::with(self.app, get_command, self.network)
          .handle()
          .await?
      }
      SonarrCommand::List(list_command) => {
        SonarrListCommandHandler::with(self.app, list_command, self.network)
          .handle()
          .await?
      }
      SonarrCommand::ClearBlocklist => {
        self
          .network
          .handle_network_event(SonarrEvent::GetBlocklist.into())
          .await?;
        execute_network_event!(self, SonarrEvent::ClearBlocklist);
      }
      SonarrCommand::ManualEpisodeSearch { episode_id } => {
        println!("Searching for episode releases. This may take a minute...");
        execute_network_event!(self, SonarrEvent::GetEpisodeReleases(Some(episode_id)));
      }
      SonarrCommand::ManualSeasonSearch {
        series_id,
        season_number,
      } => {
        println!("Searching for season releases. This may take a minute...");
        execute_network_event!(
          self,
          SonarrEvent::GetSeasonReleases(Some((series_id, season_number)))
        );
      }
    }

    Ok(())
  }
}
