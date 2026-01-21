#[cfg(test)]
mod test {
  use crate::ui::styles::{
    awaiting_import_style, default_style, downloaded_style, downloading_style, failure_style,
    help_style, highlight_style, indeterminate_style, missing_style, primary_style,
    secondary_style, success_style, system_function_style, unmonitored_missing_style,
    unmonitored_style, unreleased_style, warning_style,
  };
  use pretty_assertions::assert_eq;
  use ratatui::prelude::Modifier;
  use ratatui::style::{Color, Style};

  #[test]
  fn test_style_awaiting_import() {
    assert_eq!(
      awaiting_import_style(),
      Style::new().fg(Color::Rgb(255, 170, 66))
    );
  }

  #[test]
  fn test_style_indeterminate() {
    assert_eq!(
      indeterminate_style(),
      Style::new().fg(Color::Rgb(255, 170, 66))
    );
  }

  #[test]
  fn test_style_default() {
    assert_eq!(default_style(), Style::new().white());
  }

  #[test]
  fn test_style_downloaded() {
    assert_eq!(downloaded_style(), Style::new().green());
  }

  #[test]
  fn test_style_downloading() {
    assert_eq!(downloading_style(), Style::new().magenta());
  }

  #[test]
  fn test_style_failure() {
    assert_eq!(failure_style(), Style::new().red());
  }

  #[test]
  fn test_style_help() {
    assert_eq!(help_style(), Style::new().light_blue());
  }

  #[test]
  fn test_style_highlight() {
    assert_eq!(
      highlight_style(),
      Style::new().add_modifier(Modifier::REVERSED)
    );
  }

  #[test]
  fn test_style_missing() {
    assert_eq!(missing_style(), Style::new().red());
  }

  #[test]
  fn test_style_primary() {
    assert_eq!(primary_style(), Style::new().cyan());
  }

  #[test]
  fn test_style_secondary() {
    assert_eq!(secondary_style(), Style::new().yellow());
  }

  #[test]
  fn test_style_success() {
    assert_eq!(success_style(), Style::new().green());
  }

  #[test]
  fn test_style_system_function() {
    assert_eq!(system_function_style(), Style::new().yellow());
  }

  #[test]
  fn test_style_unmonitored() {
    assert_eq!(unmonitored_style(), Style::new().gray());
  }

  #[test]
  fn test_style_unmonitored_missing() {
    assert_eq!(unmonitored_missing_style(), Style::new().yellow());
  }

  #[test]
  fn test_style_unreleased() {
    assert_eq!(unreleased_style(), Style::new().light_cyan());
  }

  #[test]
  fn test_style_warning() {
    assert_eq!(warning_style(), Style::new().magenta());
  }
}
