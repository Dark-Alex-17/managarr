use std::sync::Arc;

use anyhow::Result;
use clap::{command, Subcommand};
use clap_complete::Shell;
use radarr::{RadarrCliHandler, RadarrCommand};
use tokio::sync::Mutex;

use crate::{app::App, network::NetworkTrait};

pub mod radarr;

#[cfg(test)]
#[path = "cli_tests.rs"]
mod cli_tests;

#[derive(Debug, Clone, Subcommand, PartialEq, Eq)]
pub enum Command {
  #[command(subcommand, about = "Commands for manging your Radarr instance")]
  Radarr(RadarrCommand),

  #[command(
    arg_required_else_help = true,
    about = "Generate shell completions for the Managarr CLI"
  )]
  Completions {
    #[arg(value_enum)]
    shell: Shell,
  },
}

pub trait CliCommandHandler<'a, 'b, T: Into<Command>> {
  fn with(app: &'a Arc<Mutex<App<'b>>>, command: T, network: &'a mut dyn NetworkTrait) -> Self;
  async fn handle(self) -> Result<()>;
}

pub(crate) async fn handle_command(
  app: &Arc<Mutex<App<'_>>>,
  command: Command,
  network: &mut dyn NetworkTrait,
) -> Result<()> {
  if let Command::Radarr(radarr_command) = command {
    RadarrCliHandler::with(app, radarr_command, network)
      .handle()
      .await?
  }
  Ok(())
}

#[inline]
pub fn mutex_flags_or_option(positive: bool, negative: bool) -> Option<bool> {
  if positive {
    Some(true)
  } else if negative {
    Some(false)
  } else {
    None
  }
}

#[inline]
pub fn mutex_flags_or_default(positive: bool, negative: bool, default_value: bool) -> bool {
  if positive {
    true
  } else if negative {
    false
  } else {
    default_value
  }
}

#[macro_export]
macro_rules! execute_network_event {
  ($self:ident, $event:expr) => {
    let resp = $self.network.handle_network_event($event.into()).await?;
    let json = serde_json::to_string_pretty(&resp)?;
    println!("{}", json);
  };
  ($self:ident, $event:expr, $happy_output:expr) => {
    $self.network.handle_network_event($event.into()).await?;
    println!("{}", $happy_output);
  };
}
