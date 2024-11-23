use std::sync::Arc;

use add_command_handler::{SonarrAddCommand, SonarrAddCommandHandler};
use anyhow::Result;
use clap::Subcommand;
use delete_command_handler::{SonarrDeleteCommand, SonarrDeleteCommandHandler};
use get_command_handler::{SonarrGetCommand, SonarrGetCommandHandler};
use list_command_handler::{SonarrListCommand, SonarrListCommandHandler};
use tokio::sync::Mutex;

use crate::{
  app::App,
  models::sonarr_models::SonarrTaskName,
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::{CliCommandHandler, Command};

mod add_command_handler;
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
    about = "Commands to add or create new resources within your Sonarr instance"
  )]
  Add(SonarrAddCommand),
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
  #[command(about = "Mark the Sonarr history item with the given ID as 'failed'")]
  MarkHistoryItemAsFailed {
    #[arg(
      long,
      help = "The Sonarr ID of the history item you wish to mark as 'failed'",
      required = true
    )]
    history_item_id: i64,
  },
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
  #[command(about = "Start the specified Sonarr task")]
  StartTask {
    #[arg(
      long,
      help = "The name of the task to trigger",
      value_enum,
      required = true
    )]
    task_name: SonarrTaskName,
  },
  #[command(
    about = "Test the indexer with the given ID. Note that a successful test returns an empty JSON body; i.e. '{}'"
  )]
  TestIndexer {
    #[arg(long, help = "The ID of the indexer to test", required = true)]
    indexer_id: i64,
  },
  #[command(about = "Test all Radarr indexers")]
  TestAllIndexers,
  #[command(about = "Trigger an automatic search for the series with the specified ID")]
  TriggerAutomaticSeriesSearch {
    #[arg(
      long,
      help = "The ID of the series you want to trigger an automatic search for",
      required = true
    )]
    series_id: i64,
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

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      SonarrCommand::Add(add_command) => {
        SonarrAddCommandHandler::with(self.app, add_command, self.network)
          .handle()
          .await?
      }
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
        let resp = self
          .network
          .handle_network_event(SonarrEvent::ClearBlocklist.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrCommand::MarkHistoryItemAsFailed { history_item_id } => {
        let _ = self
          .network
          .handle_network_event(SonarrEvent::MarkHistoryItemAsFailed(history_item_id).into())
          .await?;
        "Sonarr history item marked as 'failed'".to_owned()
      }
      SonarrCommand::ManualEpisodeSearch { episode_id } => {
        println!("Searching for episode releases. This may take a minute...");
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetEpisodeReleases(Some(episode_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrCommand::ManualSeasonSearch {
        series_id,
        season_number,
      } => {
        println!("Searching for season releases. This may take a minute...");
        let resp = self
          .network
          .handle_network_event(
            SonarrEvent::GetSeasonReleases(Some((series_id, season_number))).into(),
          )
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrCommand::StartTask { task_name } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::StartTask(Some(task_name)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrCommand::TestIndexer { indexer_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::TestIndexer(Some(indexer_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrCommand::TestAllIndexers => {
        println!("Testing all Sonarr indexers. This may take a minute...");
        let resp = self
          .network
          .handle_network_event(SonarrEvent::TestAllIndexers.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrCommand::TriggerAutomaticSeriesSearch { series_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::TriggerAutomaticSeriesSearch(Some(series_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
