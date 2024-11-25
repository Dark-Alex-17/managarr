use std::sync::Arc;

use anyhow::Result;
use clap::{ArgGroup, Subcommand};
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::{
    sonarr_models::{IndexerSettings, SonarrSerdeable},
    Serdeable,
  },
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "edit_command_handler_tests.rs"]
mod edit_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrEditCommand {
  #[command(
    about = "Edit and indexer settings that apply to all indexers", 
    group(
      ArgGroup::new("edit_settings")
      .args([
        "maximum_size", 
        "minimum_age", 
        "retention", 
        "rss_sync_interval", 
      ]).required(true)
      .multiple(true))
  )]
  AllIndexerSettings {
    #[arg(
      long,
      help = "The maximum size for a release to be grabbed in MB. Set to zero to set to unlimited"
    )]
    maximum_size: Option<i64>,
    #[arg(
      long,
      help = "Usenet only: Minimum age in minutes of NZBs before they are grabbed. Use this to give new releases time to propagate to your usenet provider."
    )]
    minimum_age: Option<i64>,
    #[arg(
      long,
      help = "Usenet only: The retention time in days to retain releases. Set to zero to set for unlimited retention"
    )]
    retention: Option<i64>,
    #[arg(
      long,
      help = "The RSS sync interval in minutes. Set to zero to disable (this will stop all automatic release grabbing)"
    )]
    rss_sync_interval: Option<i64>,
  },
}

impl From<SonarrEditCommand> for Command {
  fn from(value: SonarrEditCommand) -> Self {
    Command::Sonarr(SonarrCommand::Edit(value))
  }
}

pub(super) struct SonarrEditCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrEditCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrEditCommand> for SonarrEditCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrEditCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrEditCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      SonarrEditCommand::AllIndexerSettings {
        maximum_size,
        minimum_age,
        retention,
        rss_sync_interval,
      } => {
        if let Serdeable::Sonarr(SonarrSerdeable::IndexerSettings(previous_indexer_settings)) = self
          .network
          .handle_network_event(SonarrEvent::GetAllIndexerSettings.into())
          .await?
        {
          let params = IndexerSettings {
            id: 1,
            maximum_size: maximum_size.unwrap_or(previous_indexer_settings.maximum_size),
            minimum_age: minimum_age.unwrap_or(previous_indexer_settings.minimum_age),
            retention: retention.unwrap_or(previous_indexer_settings.retention),
            rss_sync_interval: rss_sync_interval
              .unwrap_or(previous_indexer_settings.rss_sync_interval),
          };
          self
            .network
            .handle_network_event(SonarrEvent::EditAllIndexerSettings(Some(params)).into())
            .await?;
          "All indexer settings updated".to_owned()
        } else {
          String::new()
        }
      }
    };

    Ok(result)
  }
}
