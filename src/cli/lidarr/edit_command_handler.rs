use std::sync::Arc;

use anyhow::Result;
use clap::{ArgAction, ArgGroup, Subcommand};
use tokio::sync::Mutex;

use super::LidarrCommand;
use crate::models::Serdeable;
use crate::models::lidarr_models::LidarrSerdeable;
use crate::models::servarr_models::{EditIndexerParams, IndexerSettings};
use crate::{
  app::App,
  cli::{CliCommandHandler, Command, mutex_flags_or_option},
  models::lidarr_models::{EditArtistParams, NewItemMonitorType},
  network::{NetworkTrait, lidarr_network::LidarrEvent},
};

#[cfg(test)]
#[path = "edit_command_handler_tests.rs"]
mod edit_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrEditCommand {
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
  #[command(
    about = "Edit preferences for the specified artist",
    group(
      ArgGroup::new("edit_artist")
      .args([
        "enable_monitoring",
        "disable_monitoring",
        "monitor_new_items",
        "quality_profile_id",
        "metadata_profile_id",
        "root_folder_path",
        "tag",
        "clear_tags"
      ]).required(true)
      .multiple(true))
  )]
  Artist {
    #[arg(
      long,
      help = "The ID of the artist whose settings you want to edit",
      required = true
    )]
    artist_id: i64,
    #[arg(
      long,
      help = "Enable monitoring of this artist in Lidarr so Lidarr will automatically download releases from this artist if they are available",
      conflicts_with = "disable_monitoring"
    )]
    enable_monitoring: bool,
    #[arg(
      long,
      help = "Disable monitoring of this artist so Lidarr does not automatically download releases from this artist if they are available",
      conflicts_with = "enable_monitoring"
    )]
    disable_monitoring: bool,
    #[arg(
      long,
      help = "How Lidarr should monitor new albums from this artist",
      value_enum
    )]
    monitor_new_items: Option<NewItemMonitorType>,
    #[arg(long, help = "The ID of the quality profile to use for this artist")]
    quality_profile_id: Option<i64>,
    #[arg(long, help = "The ID of the metadata profile to use for this artist")]
    metadata_profile_id: Option<i64>,
    #[arg(
      long,
      help = "The root folder path where all artist data and metadata should live"
    )]
    root_folder_path: Option<String>,
    #[arg(
      long,
      help = "Tag IDs to tag this artist with",
      value_parser,
      action = ArgAction::Append,
      conflicts_with = "clear_tags"
    )]
    tag: Option<Vec<i64>>,
    #[arg(long, help = "Clear all tags on this artist", conflicts_with = "tag")]
    clear_tags: bool,
  },
  #[command(
    about = "Edit preferences for the specified indexer",
    group(
      ArgGroup::new("edit_indexer")
      .args([
        "name",
        "enable_rss",
        "disable_rss",
        "enable_automatic_search",
        "disable_automatic_search",
        "enable_interactive_search",
        "disable_automatic_search",
        "url",
        "api_key",
        "seed_ratio",
        "tag",
        "priority",
        "clear_tags"
      ]).required(true)
      .multiple(true))
  )]
  Indexer {
    #[arg(
      long,
      help = "The ID of the indexer whose settings you wish to edit",
      required = true
    )]
    indexer_id: i64,
    #[arg(long, help = "The name of the indexer")]
    name: Option<String>,
    #[arg(
      long,
      help = "Indicate to Lidarr that this indexer should be used when Lidarr periodically looks for releases via RSS Sync",
      conflicts_with = "disable_rss"
    )]
    enable_rss: bool,
    #[arg(
      long,
      help = "Disable using this indexer when Lidarr periodically looks for releases via RSS Sync",
      conflicts_with = "enable_rss"
    )]
    disable_rss: bool,
    #[arg(
      long,
      help = "Indicate to Lidarr that this indexer should be used when automatic searches are performed via the UI or by Lidarr",
      conflicts_with = "disable_automatic_search"
    )]
    enable_automatic_search: bool,
    #[arg(
      long,
      help = "Disable using this indexer whenever automatic searches are performed via the UI or by Lidarr",
      conflicts_with = "enable_automatic_search"
    )]
    disable_automatic_search: bool,
    #[arg(
      long,
      help = "Indicate to Lidarr that this indexer should be used when an interactive search is used",
      conflicts_with = "disable_interactive_search"
    )]
    enable_interactive_search: bool,
    #[arg(
      long,
      help = "Disable using this indexer whenever an interactive search is performed",
      conflicts_with = "enable_interactive_search"
    )]
    disable_interactive_search: bool,
    #[arg(long, help = "The URL of the indexer")]
    url: Option<String>,
    #[arg(long, help = "The API key used to access the indexer's API")]
    api_key: Option<String>,
    #[arg(
      long,
      help = "The ratio a torrent should reach before stopping; Empty uses the download client's default. Ratio should be at least 1.0 and follow the indexer's rules"
    )]
    seed_ratio: Option<String>,
    #[arg(
      long,
      help = "Only use this indexer for series with at least one matching tag ID. Leave blank to use with all series.",
      value_parser,
      action = ArgAction::Append,
      conflicts_with = "clear_tags"
    )]
    tag: Option<Vec<i64>>,
    #[arg(
      long,
      help = "Indexer Priority from 1 (Highest) to 50 (Lowest). Default: 25. Used when grabbing releases as a tiebreaker for otherwise equal releases, Lidarr will still use all enabled indexers for RSS Sync and Searching"
    )]
    priority: Option<i64>,
    #[arg(long, help = "Clear all tags on this indexer", conflicts_with = "tag")]
    clear_tags: bool,
  },
}

impl From<LidarrEditCommand> for Command {
  fn from(value: LidarrEditCommand) -> Self {
    Command::Lidarr(LidarrCommand::Edit(value))
  }
}

pub(super) struct LidarrEditCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrEditCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrEditCommand> for LidarrEditCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrEditCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrEditCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrEditCommand::AllIndexerSettings {
        maximum_size,
        minimum_age,
        retention,
        rss_sync_interval,
      } => {
        if let Serdeable::Lidarr(LidarrSerdeable::IndexerSettings(previous_indexer_settings)) = self
          .network
          .handle_network_event(LidarrEvent::GetAllIndexerSettings.into())
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
            .handle_network_event(LidarrEvent::EditAllIndexerSettings(params).into())
            .await?;
          "All indexer settings updated".to_owned()
        } else {
          String::new()
        }
      }
      LidarrEditCommand::Artist {
        artist_id,
        enable_monitoring,
        disable_monitoring,
        monitor_new_items,
        quality_profile_id,
        metadata_profile_id,
        root_folder_path,
        tag,
        clear_tags,
      } => {
        let monitored_value = mutex_flags_or_option(enable_monitoring, disable_monitoring);
        let edit_artist_params = EditArtistParams {
          artist_id,
          monitored: monitored_value,
          monitor_new_items,
          quality_profile_id,
          metadata_profile_id,
          root_folder_path,
          tags: tag,
          tag_input_string: None,
          clear_tags,
        };

        self
          .network
          .handle_network_event(LidarrEvent::EditArtist(edit_artist_params).into())
          .await?;
        "Artist Updated".to_owned()
      }
      LidarrEditCommand::Indexer {
        indexer_id,
        name,
        enable_rss,
        disable_rss,
        enable_automatic_search,
        disable_automatic_search,
        enable_interactive_search,
        disable_interactive_search,
        url,
        api_key,
        seed_ratio,
        tag,
        priority,
        clear_tags,
      } => {
        let rss_value = mutex_flags_or_option(enable_rss, disable_rss);
        let automatic_search_value =
          mutex_flags_or_option(enable_automatic_search, disable_automatic_search);
        let interactive_search_value =
          mutex_flags_or_option(enable_interactive_search, disable_interactive_search);
        let edit_indexer_params = EditIndexerParams {
          indexer_id,
          name,
          enable_rss: rss_value,
          enable_automatic_search: automatic_search_value,
          enable_interactive_search: interactive_search_value,
          url,
          api_key,
          seed_ratio,
          tags: tag,
          tag_input_string: None,
          priority,
          clear_tags,
        };

        self
          .network
          .handle_network_event(LidarrEvent::EditIndexer(edit_indexer_params).into())
          .await?;
        "Indexer updated".to_owned()
      }
    };

    Ok(result)
  }
}
