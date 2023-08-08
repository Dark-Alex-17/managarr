#[cfg(test)]
mod tests {
  use super::super::*;
  use pretty_assertions::assert_str_eq;

  #[test]
  fn test_determine_log_style_by_level() {
    assert_eq!(
      determine_log_style_by_level("trace"),
      Style::default().fg(Color::Gray)
    );
    assert_eq!(
      determine_log_style_by_level("debug"),
      Style::default().fg(Color::Blue)
    );
    assert_eq!(determine_log_style_by_level("info"), style_default());
    assert_eq!(determine_log_style_by_level("warn"), style_secondary());
    assert_eq!(determine_log_style_by_level("error"), style_failure());
    assert_eq!(
      determine_log_style_by_level("fatal"),
      style_failure().add_modifier(Modifier::BOLD)
    );
    assert_eq!(determine_log_style_by_level(""), style_default());
  }

  #[test]
  fn test_determine_log_style_by_level_case_insensitive() {
    assert_eq!(
      determine_log_style_by_level("TrAcE"),
      Style::default().fg(Color::Gray)
    );
  }

  #[test]
  fn test_convert_to_minutes_hours_days_minutes() {
    assert_str_eq!(convert_to_minutes_hours_days(0), "now");
    assert_str_eq!(convert_to_minutes_hours_days(1), "1 minute");
    assert_str_eq!(convert_to_minutes_hours_days(2), "2 minutes");
  }

  #[test]
  fn test_convert_to_minutes_hours_days_hours() {
    assert_str_eq!(convert_to_minutes_hours_days(60), "1 hour");
    assert_str_eq!(convert_to_minutes_hours_days(120), "2 hours");
  }

  #[test]
  fn test_convert_to_minutes_hours_days_days() {
    assert_str_eq!(convert_to_minutes_hours_days(1440), "1 day");
    assert_str_eq!(convert_to_minutes_hours_days(2880), "2 days");
  }
}
