#[cfg(test)]
mod tests {
  use super::super::*;
  use pretty_assertions::assert_str_eq;
  use ratatui::prelude::Text;
  use ratatui::text::Span;

  #[test]
  fn test_determine_log_style_by_level() {
    let list_item = ListItem::new(Text::from(Span::raw("test")));

    assert_eq!(
      style_log_list_item(list_item.clone(), "trace".to_string()),
      list_item.clone().gray()
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "debug".to_string()),
      list_item.clone().blue()
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "info".to_string()),
      list_item.clone().style(Style::new().default())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "warn".to_string()),
      list_item.clone().style(Style::new().secondary())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "error".to_string()),
      list_item.clone().style(Style::new().failure())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "fatal".to_string()),
      list_item.clone().style(Style::new().failure().bold())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "".to_string()),
      list_item.style(Style::new().default())
    );
  }

  #[test]
  fn test_determine_log_style_by_level_case_insensitive() {
    let list_item = ListItem::new(Text::from(Span::raw("test")));

    assert_eq!(
      style_log_list_item(list_item.clone(), "TrAcE".to_string()),
      list_item.gray()
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
