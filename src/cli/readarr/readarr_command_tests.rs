#[cfg(test)]
mod tests {
  mod cli {
    use clap::CommandFactory;

    use crate::Cli;

    #[test]
    fn test_readarr_command_requires_a_subcommand() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr"]);

      assert_err!(&result);
    }
  }
}
