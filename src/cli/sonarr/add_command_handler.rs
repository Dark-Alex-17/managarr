use std::sync::Arc;

use anyhow::Result;
use clap::{ArgAction, Subcommand};
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::sonarr_models::{AddSeriesBody, AddSeriesOptions, SeriesMonitor, SeriesType},
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "add_command_handler_tests.rs"]
mod add_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrAddCommand {
  #[command(about = "Add a new series to your Sonarr library")]
  Series {
    #[arg(
      long,
      help = "The TVDB ID of the series you wish to add to your library",
      required = true
    )]
    tvdb_id: i64,
    #[arg(
      long,
      help = "The root folder path where all series data and metadata should live",
      required = true
    )]
    root_folder_path: String,
    #[arg(
      long,
      help = "The ID of the quality profile to use for this series",
      required = true
    )]
    quality_profile_id: i64,
    #[arg(
      long,
      help = "The ID of the language profile to use for this series",
      required = true
    )]
    language_profile_id: i64,
    #[arg(
      long,
      help = "The type of series",
      value_enum,
      default_value_t = SeriesType::default()
    )]
    series_type: SeriesType,
    #[arg(long, help = "Disable monitoring for this series")]
    disable_monitoring: bool,
    #[arg(long, help = "Don't use season folders for this series")]
    disable_season_folders: bool,
    #[arg(
      long,
      help = "Tag IDs to tag the series with", 
      value_parser,
      action = ArgAction::Append
    )]
    tag: Vec<i64>,
    #[arg(
      long,
      help = "What Sonarr should monitor", 
      value_enum,
      default_value_t = SeriesMonitor::default()
    )]
    monitor: SeriesMonitor,
    #[arg(
      long,
      help = "Tell Sonarr to not start a search for this series once it's added to your library"
    )]
    no_search_for_series: bool,
  },
  #[command(about = "Add a new root folder")]
  RootFolder {
    #[arg(long, help = "The path of the new root folder", required = true)]
    root_folder_path: String,
  },
  #[command(about = "Add new tag")]
  Tag {
    #[arg(long, help = "The name of the tag to be added", required = true)]
    name: String,
  },
}

impl From<SonarrAddCommand> for Command {
  fn from(value: SonarrAddCommand) -> Self {
    Command::Sonarr(SonarrCommand::Add(value))
  }
}

pub(super) struct SonarrAddCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrAddCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrAddCommand> for SonarrAddCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrAddCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrAddCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      SonarrAddCommand::Series {
        tvdb_id,
        root_folder_path,
        quality_profile_id,
        language_profile_id,
        series_type,
        disable_monitoring,
        disable_season_folders,
        tag: tags,
        monitor,
        no_search_for_series,
      } => {
        let body = AddSeriesBody {
          tvdb_id,
          title: String::new(),
          monitored: !disable_monitoring,
          root_folder_path,
          quality_profile_id,
          language_profile_id,
          series_type: series_type.to_string(),
          season_folder: !disable_season_folders,
          tags,
          add_options: AddSeriesOptions {
            monitor: monitor.to_string(),
            search_for_cutoff_unmet_episodes: !no_search_for_series,
            search_for_missing_episodes: !no_search_for_series,
          },
        };
        let resp = self
          .network
          .handle_network_event(SonarrEvent::AddSeries(Some(body)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrAddCommand::RootFolder { root_folder_path } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::AddRootFolder(Some(root_folder_path)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrAddCommand::Tag { name } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::AddTag(name).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };
    Ok(result)
  }
}
