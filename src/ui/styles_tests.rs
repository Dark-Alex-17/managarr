#[cfg(test)]
mod test {
  use crate::ui::styles::{ManagarrStyle, COLOR_ORANGE};
  use pretty_assertions::assert_eq;
  use ratatui::prelude::Modifier;
  use ratatui::style::{Style, Stylize};

  #[test]
  fn test_new() {
    assert_eq!(Style::new(), <Style as Default>::default())
  }

  #[test]
  fn test_style_awaiting_import() {
    assert_eq!(
      Style::new().awaiting_import(),
      Style::new().fg(COLOR_ORANGE)
    );
  }

  #[test]
  fn test_style_default() {
    assert_eq!(Style::new().default(), Style::new().white());
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
    assert_eq!(Style::new().unmonitored(), Style::new().white());
  }

  #[test]
  fn test_style_warning() {
    assert_eq!(Style::new().warning(), Style::new().magenta());
  }
}
