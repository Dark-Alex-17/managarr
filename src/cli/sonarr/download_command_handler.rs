use std::sync::Arc;

use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  models::sonarr_models::SonarrReleaseDownloadBody,
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "download_command_handler_tests.rs"]
mod download_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrDownloadCommand {
  #[command(about = "Manually download the given series release for the specified series ID")]
  Series {
    #[arg(long, help = "The GUID of the release to download", required = true)]
    guid: String,
    #[arg(
      long,
      help = "The indexer ID to download the release from",
      required = true
    )]
    indexer_id: i64,
    #[arg(
      long,
      help = "The series ID that the release is associated with",
      required = true
    )]
    series_id: i64,
  },
  #[command(
    about = "Manually download the given season release corresponding to the series specified with the series ID"
  )]
  Season {
    #[arg(long, help = "The GUID of the release to download", required = true)]
    guid: String,
    #[arg(
      long,
      help = "The indexer ID to download the release from",
      required = true
    )]
    indexer_id: i64,
    #[arg(
      long,
      help = "The series ID that the release is associated with",
      required = true
    )]
    series_id: i64,
    #[arg(
      long,
      help = "The season number that the release corresponds to",
      required = true
    )]
    season_number: i64,
  },
}

impl From<SonarrDownloadCommand> for Command {
  fn from(value: SonarrDownloadCommand) -> Self {
    Command::Sonarr(SonarrCommand::Download(value))
  }
}

pub(super) struct SonarrDownloadCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrDownloadCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrDownloadCommand>
  for SonarrDownloadCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrDownloadCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrDownloadCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> anyhow::Result<String> {
    let result = match self.command {
      SonarrDownloadCommand::Series {
        guid,
        indexer_id,
        series_id,
      } => {
        let params = SonarrReleaseDownloadBody {
          guid,
          indexer_id,
          series_id: Some(series_id),
          ..SonarrReleaseDownloadBody::default()
        };
        let resp = self
          .network
          .handle_network_event(SonarrEvent::DownloadRelease(params).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrDownloadCommand::Season {
        guid,
        indexer_id,
        series_id,
        season_number,
      } => {
        let params = SonarrReleaseDownloadBody {
          guid,
          indexer_id,
          series_id: Some(series_id),
          season_number: Some(season_number),
          ..SonarrReleaseDownloadBody::default()
        };
        let resp = self
          .network
          .handle_network_event(SonarrEvent::DownloadRelease(params).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
