use std::sync::Arc;

use anyhow::Result;
use clap::{ArgAction, ArgGroup, Subcommand};
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{mutex_flags_or_default, mutex_flags_or_option, CliCommandHandler, Command},
  execute_network_event,
  models::{
    radarr_models::{
      EditCollectionParams, EditIndexerParams, EditMovieParams, IndexerSettings,
      MinimumAvailability, RadarrSerdeable,
    },
    Serdeable,
  },
  network::{radarr_network::RadarrEvent, NetworkTrait},
};

use super::RadarrCommand;

#[cfg(test)]
#[path = "edit_command_handler_tests.rs"]
mod edit_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum RadarrEditCommand {
  #[command(
    about = "Edit and indexer settings that apply to all indexers", 
    group(
      ArgGroup::new("edit_settings")
      .args([
        "allow_hardcoded_subs", 
        "disable_allow_hardcoded_subs", 
        "availability_delay", 
        "maximum_size", 
        "minimum_age", 
        "prefer_indexer_flags", 
        "disable_prefer_indexer_flags", 
        "retention", 
        "rss_sync_interval", 
        "whitelisted_subtitle_tags"
      ]).required(true)
      .multiple(true))
  )]
  AllIndexerSettings {
    #[arg(
      long,
      help = "Detected hardcoded subs will be automatically downloaded",
      conflicts_with = "disable_allow_hardcoded_subs"
    )]
    allow_hardcoded_subs: bool,
    #[arg(
      long,
      help = "Disable allowing detected hardcoded subs from being automatically downloaded",
      conflicts_with = "allow_hardcoded_subs"
    )]
    disable_allow_hardcoded_subs: bool,
    #[arg(
      long,
      help = "Amount of time in days before or after available date to search for Movie"
    )]
    availability_delay: Option<i64>,
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
      help = "Prioritize releases with special flags",
      conflicts_with = "disable_prefer_indexer_flags"
    )]
    prefer_indexer_flags: bool,
    #[arg(
      long,
      help = "Disable prioritizing releases with special flags",
      conflicts_with = "prefer_indexer_flags"
    )]
    disable_prefer_indexer_flags: bool,
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
    #[arg(
      long,
      help = "A comma separated list of subtitle tags that will not be considered hardcoded"
    )]
    whitelisted_subtitle_tags: Option<String>,
  },
  #[command(
    about = "Edit preferences for the specified collection",
    group(
      ArgGroup::new("edit_collection")
      .args([
        "enable_monitoring",
        "disable_monitoring",
        "minimum_availability",
        "quality_profile_id",
        "root_folder_path",
        "search_on_add",
        "disable_search_on_add"
      ]).required(true)
      .multiple(true))
  )]
  Collection {
    #[arg(
      long,
      help = "The ID of the collection whose preferences you want to edit",
      required = true
    )]
    collection_id: i64,
    #[arg(
      long,
      help = "Monitor to automatically have movies from this collection added to your library",
      conflicts_with = "disable_monitoring"
    )]
    enable_monitoring: bool,
    #[arg(
      long,
      help = "Disable monitoring for this collection so movies from this collection are not automatically added to your library",
      conflicts_with = "enable_monitoring"
    )]
    disable_monitoring: bool,
    #[arg(
      long,
      help = "Specify the minimum availability for all movies in this collection",
      value_enum
    )]
    minimum_availability: Option<MinimumAvailability>,
    #[arg(
      long,
      help = "The ID of the quality profile that all movies in this collection should use"
    )]
    quality_profile_id: Option<i64>,
    #[arg(
      long,
      help = "The root folder path that all movies in this collection should exist under"
    )]
    root_folder_path: Option<String>,
    #[arg(
      long,
      help = "Search for movies from this collection when added to your library",
      conflicts_with = "disable_search_on_add"
    )]
    search_on_add: bool,
    #[arg(
      long,
      help = "Disable triggering searching whenever new movies are added to this collection",
      conflicts_with = "search_on_add"
    )]
    disable_search_on_add: bool,
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
      help = "Indicate to Radarr that this indexer should be used when Radarr periodically looks for releases via RSS Sync",
      conflicts_with = "disable_rss"
    )]
    enable_rss: bool,
    #[arg(
      long,
      help = "Disable using this indexer when Radarr periodically looks for releases via RSS Sync",
      conflicts_with = "enable_rss"
    )]
    disable_rss: bool,
    #[arg(
      long,
      help = "Indicate to Radarr that this indexer should be used when automatic searches are performed via the UI or by Radarr",
      conflicts_with = "disable_automatic_search"
    )]
    enable_automatic_search: bool,
    #[arg(
      long,
      help = "Disable using this indexer whenever automatic searches are performed via the UI or by Radarr",
      conflicts_with = "enable_automatic_search"
    )]
    disable_automatic_search: bool,
    #[arg(
      long,
      help = "Indicate to Radarr that this indexer should be used when an interactive search is used",
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
      help = "Only use this indexer for movies with at least one matching tag ID. Leave blank to use with all movies.",
      value_parser,
      action = ArgAction::Append,
      conflicts_with = "clear_tags"
    )]
    tag: Option<Vec<i64>>,
    #[arg(
      long,
      help = "Indexer Priority from 1 (Highest) to 50 (Lowest). Default: 25. Used when grabbing releases as a tiebreaker for otherwise equal releases, Radarr will still use all enabled indexers for RSS Sync and Searching"
    )]
    priority: Option<i64>,
    #[arg(long, help = "Clear all tags on this indexer", conflicts_with = "tag")]
    clear_tags: bool,
  },
  #[command(
    about = "Edit preferences for the specified movie",
    group(
      ArgGroup::new("edit_movie")
      .args([
        "enable_monitoring",
        "disable_monitoring",
        "minimum_availability",
        "quality_profile_id",
        "root_folder_path",
        "tag",
        "clear_tags"
      ]).required(true)
      .multiple(true))
  )]
  Movie {
    #[arg(
      long,
      help = "The ID of the movie whose settings you want to edit",
      required = true
    )]
    movie_id: i64,
    #[arg(
      long,
      help = "Enable monitoring of this movie in Radarr so Radarr will automatically download this movie if it is available",
      conflicts_with = "disable_monitoring"
    )]
    enable_monitoring: bool,
    #[arg(
      long,
      help = "Disable monitoring of this movie so Radarr does not automatically download the movie if it is found to be available",
      conflicts_with = "enable_monitoring"
    )]
    disable_monitoring: bool,
    #[arg(
      long,
      help = "The minimum availability to monitor for this film",
      value_enum
    )]
    minimum_availability: Option<MinimumAvailability>,
    #[arg(long, help = "The ID of the quality profile to use for this movie")]
    quality_profile_id: Option<i64>,
    #[arg(
      long,
      help = "The root folder path where all film data and metadata should live"
    )]
    root_folder_path: Option<String>,
    #[arg(
      long,
      help = "Tag IDs to tag this movie with",
      value_parser,
      action = ArgAction::Append,
      conflicts_with = "clear_tags"
    )]
    tag: Option<Vec<i64>>,
    #[arg(long, help = "Clear all tags on this movie", conflicts_with = "tag")]
    clear_tags: bool,
  },
}

impl From<RadarrEditCommand> for Command {
  fn from(value: RadarrEditCommand) -> Self {
    Command::Radarr(RadarrCommand::Edit(value))
  }
}

pub(super) struct RadarrEditCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: RadarrEditCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, RadarrEditCommand> for RadarrEditCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: RadarrEditCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    RadarrEditCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<()> {
    match self.command {
      RadarrEditCommand::AllIndexerSettings {
        allow_hardcoded_subs,
        disable_allow_hardcoded_subs,
        availability_delay,
        maximum_size,
        minimum_age,
        prefer_indexer_flags,
        disable_prefer_indexer_flags,
        retention,
        rss_sync_interval,
        whitelisted_subtitle_tags,
      } => {
        if let Serdeable::Radarr(RadarrSerdeable::IndexerSettings(previous_indexer_settings)) = self
          .network
          .handle_network_event(RadarrEvent::GetAllIndexerSettings.into())
          .await?
        {
          let allow_hardcoded_subs_value = mutex_flags_or_default(
            allow_hardcoded_subs,
            disable_allow_hardcoded_subs,
            previous_indexer_settings.allow_hardcoded_subs,
          );
          let prefer_indexer_flags_value = mutex_flags_or_default(
            prefer_indexer_flags,
            disable_prefer_indexer_flags,
            previous_indexer_settings.prefer_indexer_flags,
          );
          let params = IndexerSettings {
            id: 1,
            allow_hardcoded_subs: allow_hardcoded_subs_value,
            availability_delay: availability_delay
              .unwrap_or(previous_indexer_settings.availability_delay),
            maximum_size: maximum_size.unwrap_or(previous_indexer_settings.maximum_size),
            minimum_age: minimum_age.unwrap_or(previous_indexer_settings.minimum_age),
            prefer_indexer_flags: prefer_indexer_flags_value,
            retention: retention.unwrap_or(previous_indexer_settings.retention),
            rss_sync_interval: rss_sync_interval
              .unwrap_or(previous_indexer_settings.rss_sync_interval),
            whitelisted_hardcoded_subs: whitelisted_subtitle_tags
              .clone()
              .unwrap_or_else(|| {
                previous_indexer_settings
                  .whitelisted_hardcoded_subs
                  .text
                  .clone()
              })
              .into(),
          };
          execute_network_event!(
            self,
            RadarrEvent::EditAllIndexerSettings(Some(params)),
            "All indexer settings updated"
          );
        }
      }
      RadarrEditCommand::Collection {
        collection_id,
        enable_monitoring,
        disable_monitoring,
        minimum_availability,
        quality_profile_id,
        root_folder_path,
        search_on_add,
        disable_search_on_add,
      } => {
        let monitored_value = mutex_flags_or_option(enable_monitoring, disable_monitoring);
        let search_on_add_value = mutex_flags_or_option(search_on_add, disable_search_on_add);

        let edit_collection_params = EditCollectionParams {
          collection_id,
          monitored: monitored_value,
          minimum_availability,
          quality_profile_id,
          root_folder_path,
          search_on_add: search_on_add_value,
        };
        execute_network_event!(
          self,
          RadarrEvent::EditCollection(Some(edit_collection_params)),
          "Collection Updated"
        );
      }
      RadarrEditCommand::Indexer {
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
          priority,
          clear_tags,
        };

        execute_network_event!(
          self,
          RadarrEvent::EditIndexer(Some(edit_indexer_params)),
          "Indexer updated"
        );
      }
      RadarrEditCommand::Movie {
        movie_id,
        enable_monitoring,
        disable_monitoring,
        minimum_availability,
        quality_profile_id,
        root_folder_path,
        tag,
        clear_tags,
      } => {
        let monitored_value = mutex_flags_or_option(enable_monitoring, disable_monitoring);
        let edit_movie_params = EditMovieParams {
          movie_id,
          monitored: monitored_value,
          minimum_availability,
          quality_profile_id,
          root_folder_path,
          tags: tag,
          clear_tags,
        };

        execute_network_event!(
          self,
          RadarrEvent::EditMovie(Some(edit_movie_params)),
          "Movie updated"
        );
      }
    }

    Ok(())
  }
}
