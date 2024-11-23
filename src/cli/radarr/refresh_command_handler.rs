use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{radarr_network::RadarrEvent, NetworkTrait},
};

use super::RadarrCommand;

#[cfg(test)]
#[path = "refresh_command_handler_tests.rs"]
mod refresh_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum RadarrRefreshCommand {
  #[command(about = "Refresh all movie data for all movies in your Radarr library")]
  AllMovies,
  #[command(about = "Refresh movie data and scan disk for the movie with the given ID")]
  Movie {
    #[arg(
      long,
      help = "The ID of the movie to refresh information on and to scan the disk for",
      required = true
    )]
    movie_id: i64,
  },
  #[command(about = "Refresh all collection data for all collections in your library")]
  Collections,
  #[command(about = "Refresh all downloads in Radarr")]
  Downloads,
}

impl From<RadarrRefreshCommand> for Command {
  fn from(value: RadarrRefreshCommand) -> Self {
    Command::Radarr(RadarrCommand::Refresh(value))
  }
}

pub(super) struct RadarrRefreshCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: RadarrRefreshCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, RadarrRefreshCommand>
  for RadarrRefreshCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: RadarrRefreshCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    RadarrRefreshCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      RadarrRefreshCommand::AllMovies => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::UpdateAllMovies.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrRefreshCommand::Collections => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::UpdateCollections.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrRefreshCommand::Downloads => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::UpdateDownloads.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrRefreshCommand::Movie { movie_id } => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::UpdateAndScan(Some(movie_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
