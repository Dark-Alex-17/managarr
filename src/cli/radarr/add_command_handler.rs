use std::sync::Arc;

use anyhow::Result;
use clap::{arg, command, ArgAction, Subcommand};
use tokio::sync::Mutex;

use super::RadarrCommand;
use crate::models::servarr_models::AddRootFolderBody;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::radarr_models::{AddMovieBody, AddMovieOptions, MinimumAvailability, MovieMonitor},
  network::{radarr_network::RadarrEvent, NetworkTrait},
};

#[cfg(test)]
#[path = "add_command_handler_tests.rs"]
mod add_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum RadarrAddCommand {
  #[command(about = "Add a new movie to your Radarr library")]
  Movie {
    #[arg(
      long,
      help = "The TMDB ID of the film you wish to add to your library",
      required = true
    )]
    tmdb_id: i64,
    #[arg(
      long,
      help = "The root folder path where all film data and metadata should live",
      required = true
    )]
    root_folder_path: String,
    #[arg(
      long,
      help = "The ID of the quality profile to use for this movie",
      required = true
    )]
    quality_profile_id: i64,
    #[arg(
      long,
      help = "The minimum availability to monitor for this film",
      value_enum,
      default_value_t = MinimumAvailability::default()
    )]
    minimum_availability: MinimumAvailability,
    #[arg(long, help = "Disable monitoring for this film")]
    disable_monitoring: bool,
    #[arg(
      long,
      help = "Tag IDs to tag the film with", 
      value_parser,
      action = ArgAction::Append
    )]
    tag: Vec<i64>,
    #[arg(
      long,
      help = "What Radarr should monitor", 
      value_enum,
      default_value_t = MovieMonitor::default()
    )]
    monitor: MovieMonitor,
    #[arg(
      long,
      help = "Tell Radarr to not start a search for this film once it's added to your library"
    )]
    no_search_for_movie: bool,
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

impl From<RadarrAddCommand> for Command {
  fn from(value: RadarrAddCommand) -> Self {
    Command::Radarr(RadarrCommand::Add(value))
  }
}

pub(super) struct RadarrAddCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: RadarrAddCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, RadarrAddCommand> for RadarrAddCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: RadarrAddCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    RadarrAddCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      RadarrAddCommand::Movie {
        tmdb_id,
        root_folder_path,
        quality_profile_id,
        minimum_availability,
        disable_monitoring,
        tag: tags,
        monitor,
        no_search_for_movie,
      } => {
        let body = AddMovieBody {
          tmdb_id,
          title: String::new(),
          root_folder_path,
          quality_profile_id,
          minimum_availability: minimum_availability.to_string(),
          monitored: !disable_monitoring,
          tags,
          tag_input_string: None,
          add_options: AddMovieOptions {
            monitor: monitor.to_string(),
            search_for_movie: !no_search_for_movie,
          },
        };
        let resp = self
          .network
          .handle_network_event(RadarrEvent::AddMovie(body).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrAddCommand::RootFolder { root_folder_path } => {
        let add_root_folder_body = AddRootFolderBody { path: root_folder_path };
        let resp = self
          .network
          .handle_network_event(RadarrEvent::AddRootFolder(add_root_folder_body).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrAddCommand::Tag { name } => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::AddTag(name).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
