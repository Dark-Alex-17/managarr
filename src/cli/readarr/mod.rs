use std::sync::Arc;

use add_command_handler::{ReadarrAddCommand, ReadarrAddCommandHandler};
use anyhow::Result;
use clap::Subcommand;
use delete_command_handler::{ReadarrDeleteCommand, ReadarrDeleteCommandHandler};
use edit_command_handler::{ReadarrEditCommand, ReadarrEditCommandHandler};
use get_command_handler::{ReadarrGetCommand, ReadarrGetCommandHandler};
use list_command_handler::{ReadarrListCommand, ReadarrListCommandHandler};
use tokio::sync::Mutex;

use super::{CliCommandHandler, Command};
use crate::models::readarr_models::ReadarrTaskName;
use crate::network::readarr_network::ReadarrEvent;
use crate::{app::App, network::NetworkTrait};

mod add_command_handler;
mod delete_command_handler;
mod edit_command_handler;
mod get_command_handler;
mod list_command_handler;

#[cfg(test)]
#[path = "readarr_command_tests.rs"]
mod readarr_command_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrCommand {
  #[command(
    subcommand,
    about = "Commands to add or create new resources within your Readarr instance"
  )]
  Add(ReadarrAddCommand),
  #[command(
    subcommand,
    about = "Commands to delete resources from your Readarr instance"
  )]
  Delete(ReadarrDeleteCommand),
  #[command(
    subcommand,
    about = "Commands to edit resources in your Readarr instance"
  )]
  Edit(ReadarrEditCommand),
  #[command(
    subcommand,
    about = "Commands to fetch details of the resources in your Readarr instance"
  )]
  Get(ReadarrGetCommand),
  #[command(
    subcommand,
    about = "Commands to list attributes from your Readarr instance"
  )]
  List(ReadarrListCommand),
  #[command(about = "Search for a new author to add to Readarr")]
  SearchNewAuthor {
    #[arg(
      long,
      help = "The name of the author you want to search for",
      required = true
    )]
    query: String,
  },
  #[command(about = "Start the specified Readarr task")]
  StartTask {
    #[arg(
      long,
      help = "The name of the task to trigger",
      value_enum,
      required = true
    )]
    task_name: ReadarrTaskName,
  },
  #[command(
    about = "Toggle monitoring for the specified author corresponding to the given author ID"
  )]
  ToggleAuthorMonitoring {
    #[arg(
      long,
      help = "The Readarr ID of the author to toggle monitoring on",
      required = true
    )]
    author_id: i64,
  },
}

impl From<ReadarrCommand> for Command {
  fn from(readarr_command: ReadarrCommand) -> Command {
    Command::Readarr(readarr_command)
  }
}

pub(super) struct ReadarrCliHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrCommand> for ReadarrCliHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrCliHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      ReadarrCommand::Add(add_command) => {
        ReadarrAddCommandHandler::with(self.app, add_command, self.network)
          .handle()
          .await?
      }
      ReadarrCommand::Delete(delete_command) => {
        ReadarrDeleteCommandHandler::with(self.app, delete_command, self.network)
          .handle()
          .await?
      }
      ReadarrCommand::Edit(edit_command) => {
        ReadarrEditCommandHandler::with(self.app, edit_command, self.network)
          .handle()
          .await?
      }
      ReadarrCommand::Get(get_command) => {
        ReadarrGetCommandHandler::with(self.app, get_command, self.network)
          .handle()
          .await?
      }
      ReadarrCommand::List(list_command) => {
        ReadarrListCommandHandler::with(self.app, list_command, self.network)
          .handle()
          .await?
      }
      ReadarrCommand::SearchNewAuthor { query } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::SearchNewAuthor(query).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrCommand::StartTask { task_name } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::StartTask(task_name).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrCommand::ToggleAuthorMonitoring { author_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::ToggleAuthorMonitoring(author_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
