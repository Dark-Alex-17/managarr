use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use super::ReadarrCommand;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::readarr_models::DeleteParams,
  network::{NetworkTrait, readarr_network::ReadarrEvent},
};

#[cfg(test)]
#[path = "delete_command_handler_tests.rs"]
mod delete_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrDeleteCommand {
  #[command(about = "Delete an author from your Readarr library")]
  Author {
    #[arg(long, help = "The ID of the author to delete", required = true)]
    author_id: i64,
    #[arg(long, help = "Delete the author files from disk as well")]
    delete_files_from_disk: bool,
    #[arg(long, help = "Add a list exclusion for this author")]
    add_list_exclusion: bool,
  },
  #[command(about = "Delete a book from your Readarr library")]
  Book {
    #[arg(long, help = "The ID of the book to delete", required = true)]
    book_id: i64,
    #[arg(long, help = "Delete the book files from disk as well")]
    delete_files_from_disk: bool,
    #[arg(long, help = "Add a list exclusion for this book")]
    add_list_exclusion: bool,
  },
  #[command(about = "Delete the specified book file from disk")]
  BookFile {
    #[arg(long, help = "The ID of the book file to delete", required = true)]
    book_file_id: i64,
  },
  #[command(about = "Delete the root folder with the given ID")]
  RootFolder {
    #[arg(long, help = "The ID of the root folder to delete", required = true)]
    root_folder_id: i64,
  },
  #[command(about = "Delete the tag with the specified ID")]
  Tag {
    #[arg(long, help = "The ID of the tag to delete", required = true)]
    tag_id: i64,
  },
}

impl From<ReadarrDeleteCommand> for Command {
  fn from(value: ReadarrDeleteCommand) -> Self {
    Command::Readarr(ReadarrCommand::Delete(value))
  }
}

pub(super) struct ReadarrDeleteCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrDeleteCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrDeleteCommand>
  for ReadarrDeleteCommandHandler<'a, 'b>
{
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrDeleteCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrDeleteCommandHandler {
      _app: app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      ReadarrDeleteCommand::Author {
        author_id,
        delete_files_from_disk,
        add_list_exclusion,
      } => {
        let delete_author_params = DeleteParams {
          id: author_id,
          delete_files: delete_files_from_disk,
          add_import_list_exclusion: add_list_exclusion,
        };
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::DeleteAuthor(delete_author_params).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrDeleteCommand::Book {
        book_id,
        delete_files_from_disk,
        add_list_exclusion,
      } => {
        let delete_book_params = DeleteParams {
          id: book_id,
          delete_files: delete_files_from_disk,
          add_import_list_exclusion: add_list_exclusion,
        };
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::DeleteBook(delete_book_params).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrDeleteCommand::BookFile { book_file_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::DeleteBookFile(book_file_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrDeleteCommand::RootFolder { root_folder_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::DeleteRootFolder(root_folder_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrDeleteCommand::Tag { tag_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::DeleteTag(tag_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
