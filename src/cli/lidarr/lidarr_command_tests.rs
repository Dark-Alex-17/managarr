#[cfg(test)]
mod tests {
  use crate::cli::{
    lidarr::{list_command_handler::LidarrListCommand, LidarrCommand},
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_lidarr_command_from() {
    let command = LidarrCommand::List(LidarrListCommand::Artists);

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Lidarr(command));
  }

  mod cli {
    use super::*;

    #[test]
    fn test_list_artists_has_no_arg_requirements() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "list", "artists"]);

      assert_ok!(&result);
    }

    #[test]
    fn test_lidarr_list_subcommand_requires_subcommand() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "list"]);

      assert_err!(&result);
    }
  }
}
