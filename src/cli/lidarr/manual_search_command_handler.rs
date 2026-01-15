use crate::app::App;
use crate::cli::lidarr::LidarrCommand;
use crate::cli::{CliCommandHandler, Command};
use crate::network::NetworkTrait;
use crate::network::lidarr_network::LidarrEvent;
use anyhow::Result;
use clap::Subcommand;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(test)]
#[path = "manual_search_command_handler_tests.rs"]
mod manual_search_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrManualSearchCommand {
  #[command(
    about = "Trigger a manual search of discography releases for the given artist corresponding to the artist with the given ID.\nNote that when downloading a discography release, ensure that the release includes 'discography: true', otherwise you'll run into issues"
  )]
  Discography {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose discography releases you wish to fetch and list",
      required = true
    )]
    artist_id: i64,
  },
}

impl From<LidarrManualSearchCommand> for Command {
  fn from(value: LidarrManualSearchCommand) -> Self {
    Command::Lidarr(LidarrCommand::ManualSearch(value))
  }
}

pub(super) struct LidarrManualSearchCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrManualSearchCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrManualSearchCommand>
  for LidarrManualSearchCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrManualSearchCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrManualSearchCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrManualSearchCommand::Discography { artist_id } => {
        println!("Searching for artist discography releases. This may take a minute...");
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetDiscographyReleases(artist_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
