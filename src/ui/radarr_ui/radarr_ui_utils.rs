use ratatui::style::{Style, Stylize};
use ratatui::widgets::ListItem;

use crate::ui::styles::ManagarrStyle;

#[cfg(test)]
#[path = "radarr_ui_utils_tests.rs"]
mod radarr_ui_utils_tests;

pub(super) fn style_log_list_item(list_item: ListItem<'_>, level: String) -> ListItem<'_> {
  match level.to_lowercase().as_str() {
    "trace" => list_item.gray(),
    "debug" => list_item.blue(),
    "info" => list_item.style(Style::new().default()),
    "warn" => list_item.style(Style::new().secondary()),
    "error" => list_item.style(Style::new().failure()),
    "fatal" => list_item.style(Style::new().failure().bold()),
    _ => list_item.style(Style::new().default()),
  }
}

pub(super) fn convert_to_minutes_hours_days(time: i64) -> String {
  if time < 60 {
    if time == 0 {
      "now".to_owned()
    } else if time == 1 {
      format!("{time} minute")
    } else {
      format!("{time} minutes")
    }
  } else if time / 60 < 24 {
    let hours = time / 60;
    if hours == 1 {
      format!("{hours} hour")
    } else {
      format!("{hours} hours")
    }
  } else {
    let days = time / (60 * 24);
    if days == 1 {
      format!("{days} day")
    } else {
      format!("{days} days")
    }
  }
}
