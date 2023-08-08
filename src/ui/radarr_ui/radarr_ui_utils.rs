use crate::ui::utils::{style_default, style_failure, style_secondary};
use tui::style::{Color, Modifier, Style};

#[cfg(test)]
#[path = "radarr_ui_utils_tests.rs"]
mod radarr_ui_utils_tests;

pub(super) fn determine_log_style_by_level(level: &str) -> Style {
  match level.to_lowercase().as_str() {
    "trace" => Style::default().fg(Color::Gray),
    "debug" => Style::default().fg(Color::Blue),
    "info" => style_default(),
    "warn" => style_secondary(),
    "error" => style_failure(),
    "fatal" => style_failure().add_modifier(Modifier::BOLD),
    _ => style_default(),
  }
}

pub(super) fn convert_to_minutes_hours_days(time: i64) -> String {
  if time < 60 {
    if time == 0 {
      "now".to_owned()
    } else if time == 1 {
      format!("{} minute", time)
    } else {
      format!("{} minutes", time)
    }
  } else if time / 60 < 24 {
    let hours = time / 60;
    if hours == 1 {
      format!("{} hour", hours)
    } else {
      format!("{} hours", hours)
    }
  } else {
    let days = time / (60 * 24);
    if days == 1 {
      format!("{} day", days)
    } else {
      format!("{} days", days)
    }
  }
}
