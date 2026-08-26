use std::sync::Arc;

use anyhow::Result;
use clap::{ArgAction, Subcommand};
use tokio::sync::Mutex;

use super::ReadarrCommand;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::readarr_models::{
    AddAuthorBody, AddAuthorOptions, AddReadarrRootFolderBody, MonitorType, NewItemMonitorType,
  },
  network::{NetworkTrait, readarr_network::ReadarrEvent},
};

#[cfg(test)]
#[path = "add_command_handler_tests.rs"]
mod add_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrAddCommand {
  #[command(about = "Add a new author to your Readarr library")]
  Author {
    #[arg(
      long,
      help = "The Goodreads foreign author ID of the author you wish to add to your library",
      required = true
    )]
    foreign_author_id: String,
    #[arg(long, help = "The name of the author", required = true)]
    author_name: String,
    #[arg(
      long,
      help = "The root folder path where all author data and metadata should live",
      required = true
    )]
    root_folder_path: String,
    #[arg(
      long,
      help = "The ID of the quality profile to use for this author",
      required = true
    )]
    quality_profile_id: i64,
    #[arg(
      long,
      help = "The ID of the metadata profile to use for this author",
      required = true
    )]
    metadata_profile_id: i64,
    #[arg(long, help = "Disable monitoring for this author")]
    disable_monitoring: bool,
    #[arg(
      long,
      help = "Tag IDs to tag the author with",
      value_parser,
      action = ArgAction::Append
    )]
    tag: Vec<i64>,
    #[arg(
      long,
      help = "What Readarr should monitor for this author",
      value_enum,
      default_value_t = MonitorType::default()
    )]
    monitor: MonitorType,
    #[arg(
      long,
      help = "How Readarr should monitor new items for this author",
      value_enum,
      default_value_t = NewItemMonitorType::default()
    )]
    monitor_new_items: NewItemMonitorType,
    #[arg(
      long,
      help = "Tell Readarr to not start a search for missing books once the author is added to your library"
    )]
    no_search_for_missing_books: bool,
  },
  #[command(about = "Add a new root folder")]
  RootFolder {
    #[arg(long, help = "The name of the root folder", required = true)]
    name: String,
    #[arg(long, help = "The path of the new root folder", required = true)]
    root_folder_path: String,
    #[arg(
      long,
      help = "The ID of the default quality profile for authors in this root folder",
      required = true
    )]
    quality_profile_id: i64,
    #[arg(
      long,
      help = "The ID of the default metadata profile for authors in this root folder",
      required = true
    )]
    metadata_profile_id: i64,
    #[arg(
      long,
      help = "The default monitor option for authors in this root folder",
      value_enum,
      default_value_t = MonitorType::default()
    )]
    monitor: MonitorType,
    #[arg(
      long,
      help = "The default monitor new items option for authors in this root folder",
      value_enum,
      default_value_t = NewItemMonitorType::default()
    )]
    monitor_new_items: NewItemMonitorType,
    #[arg(
      long,
      help = "Default tag IDs for authors in this root folder",
      value_parser,
      action = ArgAction::Append
    )]
    tag: Vec<i64>,
  },
  #[command(about = "Add new tag")]
  Tag {
    #[arg(long, help = "The name of the tag to be added", required = true)]
    name: String,
  },
}

impl From<ReadarrAddCommand> for Command {
  fn from(value: ReadarrAddCommand) -> Self {
    Command::Readarr(ReadarrCommand::Add(value))
  }
}

pub(super) struct ReadarrAddCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrAddCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrAddCommand> for ReadarrAddCommandHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrAddCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrAddCommandHandler {
      _app: app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      ReadarrAddCommand::Author {
        foreign_author_id,
        author_name,
        root_folder_path,
        quality_profile_id,
        metadata_profile_id,
        disable_monitoring,
        tag: tags,
        monitor,
        monitor_new_items,
        no_search_for_missing_books,
      } => {
        let body = AddAuthorBody {
          foreign_author_id,
          author_name,
          monitored: !disable_monitoring,
          root_folder_path,
          quality_profile_id,
          metadata_profile_id,
          tags,
          tag_input_string: None,
          add_options: AddAuthorOptions {
            monitor,
            monitor_new_items,
            search_for_missing_books: !no_search_for_missing_books,
          },
        };
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::AddAuthor(body).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrAddCommand::RootFolder {
        name,
        root_folder_path,
        quality_profile_id,
        metadata_profile_id,
        monitor,
        monitor_new_items,
        tag: tags,
      } => {
        let add_root_folder_body = AddReadarrRootFolderBody {
          name,
          path: root_folder_path,
          default_quality_profile_id: quality_profile_id,
          default_metadata_profile_id: metadata_profile_id,
          default_monitor_option: monitor,
          default_new_item_monitor_option: monitor_new_items,
          default_tags: tags,
          tag_input_string: None,
        };
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::AddRootFolder(add_root_folder_body).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrAddCommand::Tag { name } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::AddTag(name).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
