use std::sync::Arc;

use anyhow::Result;
use clap::{ArgAction, Subcommand};
use tokio::sync::Mutex;

use super::ReadarrCommand;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::readarr_models::{AddReadarrRootFolderBody, MonitorType, NewItemMonitorType},
  network::{NetworkTrait, readarr_network::ReadarrEvent},
};

#[cfg(test)]
#[path = "add_command_handler_tests.rs"]
mod add_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrAddCommand {
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
