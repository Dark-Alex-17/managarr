use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  execute_network_event,
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "list_command_handler_tests.rs"]
mod list_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrListCommand {
  #[command(about = "List all items in the Sonarr blocklist")]
  Blocklist,
  #[command(about = "Fetch Sonarr logs")]
  Logs {
    #[arg(long, help = "How many log events to fetch", default_value_t = 500)]
    events: u64,
    #[arg(
      long,
      help = "Output the logs in the same format as they appear in the log files"
    )]
    output_in_log_format: bool,
  },
  #[command(about = "List all series in your Sonarr library")]
  Series,
}

impl From<SonarrListCommand> for Command {
  fn from(value: SonarrListCommand) -> Self {
    Command::Sonarr(SonarrCommand::List(value))
  }
}

pub(super) struct SonarrListCommandHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrListCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrListCommand> for SonarrListCommandHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrListCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrListCommandHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<()> {
    match self.command {
      SonarrListCommand::Blocklist => {
        execute_network_event!(self, SonarrEvent::GetBlocklist);
      }
      SonarrListCommand::Logs {
        events,
        output_in_log_format,
      } => {
        let logs = self
          .network
          .handle_network_event(SonarrEvent::GetLogs(Some(events)).into())
          .await?;

        if output_in_log_format {
          let log_lines = self.app.lock().await.data.sonarr_data.logs.items.clone();

          let json = serde_json::to_string_pretty(&log_lines)?;
          println!("{}", json);
        } else {
          let json = serde_json::to_string_pretty(&logs)?;
          println!("{}", json);
        }
      }
      SonarrListCommand::Series => {
        execute_network_event!(self, SonarrEvent::ListSeries);
      }
    }

    Ok(())
  }
}