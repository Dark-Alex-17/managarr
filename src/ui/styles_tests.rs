#[cfg(test)]
mod test {
  use crate::ui::styles::ManagarrStyle;
  use pretty_assertions::assert_eq;
  use ratatui::prelude::Modifier;
  use ratatui::style::{Color, Style, Stylize};

  #[test]
  fn test_new() {
    assert_eq!(Style::new(), <Style as Default>::default())
  }

  #[test]
  fn test_style_awaiting_import() {
    assert_eq!(
      Style::new().awaiting_import(),
      Style::new().fg(Color::Rgb(255, 170, 66))
    );
  }

  #[test]
  fn test_style_indeterminate() {
    assert_eq!(
      Style::new().indeterminate(),
      Style::new().fg(Color::Rgb(255, 170, 66))
    );
  }

  #[test]
  fn test_style_default() {
    assert_eq!(Style::new().default(), Style::new().white());
  }

  #[test]
  fn test_style_downloaded() {
    assert_eq!(Style::new().downloaded(), Style::new().green());
  }

  #[test]
  fn test_style_downloading() {
    assert_eq!(Style::new().downloading(), Style::new().magenta());
  }

  #[test]
  fn test_style_failure() {
    assert_eq!(Style::new().failure(), Style::new().red());
  }

  #[test]
  fn test_style_help() {
    assert_eq!(Style::new().help(), Style::new().light_blue());
  }

  #[test]
  fn test_style_highlight() {
    assert_eq!(
      Style::new().highlight(),
      Style::new().add_modifier(Modifier::REVERSED)
    );
  }

  #[test]
  fn test_style_missing() {
    assert_eq!(Style::new().missing(), Style::new().red());
  }

  #[test]
  fn test_style_primary() {
    assert_eq!(Style::new().primary(), Style::new().cyan());
  }

  #[test]
  fn test_style_secondary() {
    assert_eq!(Style::new().secondary(), Style::new().yellow());
  }

  #[test]
  fn test_style_success() {
    assert_eq!(Style::new().success(), Style::new().green());
  }

  #[test]
  fn test_style_system_function() {
    assert_eq!(Style::new().system_function(), Style::new().yellow());
  }

  #[test]
  fn test_style_unmonitored() {
    assert_eq!(Style::new().unmonitored(), Style::new().gray());
  }

  #[test]
  fn test_style_unmonitored_missing() {
    assert_eq!(Style::new().unmonitored_missing(), Style::new().yellow());
  }

  #[test]
  fn test_style_unreleased() {
    assert_eq!(Style::new().unreleased(), Style::new().light_cyan());
  }

  #[test]
  fn test_style_warning() {
    assert_eq!(Style::new().warning(), Style::new().magenta());
  }
}
