use std::sync::Arc;

use anyhow::Result;
use clap::{ArgAction, ArgGroup, Subcommand};
use tokio::sync::Mutex;

use super::ReadarrCommand;
use crate::models::servarr_models::EditIndexerParams;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command, mutex_flags_or_option},
  models::readarr_models::{EditAuthorParams, NewItemMonitorType},
  network::{NetworkTrait, readarr_network::ReadarrEvent},
};

#[cfg(test)]
#[path = "edit_command_handler_tests.rs"]
mod edit_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrEditCommand {
  #[command(
    about = "Edit preferences for the specified author",
    group(
      ArgGroup::new("edit_author")
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
  Author {
    #[arg(
      long,
      help = "The ID of the author whose settings you want to edit",
      required = true
    )]
    author_id: i64,
    #[arg(
      long,
      help = "Enable monitoring of this author in Readarr so Readarr will automatically download books from this author if they are available",
      conflicts_with = "disable_monitoring"
    )]
    enable_monitoring: bool,
    #[arg(
      long,
      help = "Disable monitoring of this author so Readarr does not automatically download books from this author if they are available",
      conflicts_with = "enable_monitoring"
    )]
    disable_monitoring: bool,
    #[arg(
      long,
      help = "How Readarr should monitor new books from this author",
      value_enum
    )]
    monitor_new_items: Option<NewItemMonitorType>,
    #[arg(long, help = "The ID of the quality profile to use for this author")]
    quality_profile_id: Option<i64>,
    #[arg(long, help = "The ID of the metadata profile to use for this author")]
    metadata_profile_id: Option<i64>,
    #[arg(
      long,
      help = "The root folder path where all author data and metadata should live"
    )]
    root_folder_path: Option<String>,
    #[arg(
      long,
      help = "Tag IDs to tag this author with",
      value_parser,
      action = ArgAction::Append,
      conflicts_with = "clear_tags"
    )]
    tag: Option<Vec<i64>>,
    #[arg(long, help = "Clear all tags on this author", conflicts_with = "tag")]
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
        "disable_interactive_search",
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
      help = "Indicate to Readarr that this indexer should be used when Readarr periodically looks for releases via RSS Sync",
      conflicts_with = "disable_rss"
    )]
    enable_rss: bool,
    #[arg(
      long,
      help = "Disable using this indexer when Readarr periodically looks for releases via RSS Sync",
      conflicts_with = "enable_rss"
    )]
    disable_rss: bool,
    #[arg(
      long,
      help = "Indicate to Readarr that this indexer should be used when automatic searches are performed via the UI or by Readarr",
      conflicts_with = "disable_automatic_search"
    )]
    enable_automatic_search: bool,
    #[arg(
      long,
      help = "Disable using this indexer whenever automatic searches are performed via the UI or by Readarr",
      conflicts_with = "enable_automatic_search"
    )]
    disable_automatic_search: bool,
    #[arg(
      long,
      help = "Indicate to Readarr that this indexer should be used when an interactive search is used",
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
      help = "Only use this indexer for authors with at least one matching tag ID. Leave blank to use with all authors.",
      value_parser,
      action = ArgAction::Append,
      conflicts_with = "clear_tags"
    )]
    tag: Option<Vec<i64>>,
    #[arg(
      long,
      help = "Indexer Priority from 1 (Highest) to 50 (Lowest). Default: 25. Used when grabbing releases as a tiebreaker for otherwise equal releases, Readarr will still use all enabled indexers for RSS Sync and Searching"
    )]
    priority: Option<i64>,
    #[arg(long, help = "Clear all tags on this indexer", conflicts_with = "tag")]
    clear_tags: bool,
  },
}

impl From<ReadarrEditCommand> for Command {
  fn from(value: ReadarrEditCommand) -> Self {
    Command::Readarr(ReadarrCommand::Edit(value))
  }
}

pub(super) struct ReadarrEditCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrEditCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrEditCommand> for ReadarrEditCommandHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrEditCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrEditCommandHandler {
      _app: app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      ReadarrEditCommand::Author {
        author_id,
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
        let edit_author_params = EditAuthorParams {
          author_id,
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
          .handle_network_event(ReadarrEvent::EditAuthor(edit_author_params).into())
          .await?;
        "Author updated".to_owned()
      }
      ReadarrEditCommand::Indexer {
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
          .handle_network_event(ReadarrEvent::EditIndexer(edit_indexer_params).into())
          .await?;
        "Indexer updated".to_owned()
      }
    };

    Ok(result)
  }
}
