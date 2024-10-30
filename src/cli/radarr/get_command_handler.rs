use std::sync::Arc;

use anyhow::Result;
use clap::{command, Subcommand};
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  execute_network_event,
  network::{radarr_network::RadarrEvent, NetworkTrait},
};

use super::RadarrCommand;

#[cfg(test)]
#[path = "get_command_handler_tests.rs"]
mod get_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum RadarrGetCommand {
  #[command(about = "Get the shared settings for all indexers")]
  AllIndexerSettings,
  #[command(about = "Get detailed information for the movie with the given ID")]
  MovieDetails {
    #[arg(
      long,
      help = "The Radarr ID of the movie whose details you wish to fetch",
      required = true
    )]
    movie_id: i64,
  },
  #[command(about = "Get history for the given movie ID")]
  MovieHistory {
    #[arg(
      long,
      help = "The Radarr ID of the movie whose history you wish to fetch",
      required = true
    )]
    movie_id: i64,
  },
  #[command(about = "Get the system status")]
  SystemStatus,
}

impl From<RadarrGetCommand> for Command {
  fn from(value: RadarrGetCommand) -> Self {
    Command::Radarr(RadarrCommand::Get(value))
  }
}

pub(super) struct RadarrGetCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: RadarrGetCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, RadarrGetCommand> for RadarrGetCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: RadarrGetCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    RadarrGetCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<()> {
    match self.command {
      RadarrGetCommand::AllIndexerSettings => {
        execute_network_event!(self, RadarrEvent::GetAllIndexerSettings);
      }
      RadarrGetCommand::MovieDetails { movie_id } => {
        execute_network_event!(self, RadarrEvent::GetMovieDetails(Some(movie_id)));
      }
      RadarrGetCommand::MovieHistory { movie_id } => {
        execute_network_event!(self, RadarrEvent::GetMovieHistory(Some(movie_id)));
      }
      RadarrGetCommand::SystemStatus => {
        execute_network_event!(self, RadarrEvent::GetStatus);
      }
    }

    Ok(())
  }
}
