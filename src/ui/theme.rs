use crate::builtin_themes::get_builtin_themes;
use anyhow::Result;
use derivative::Derivative;
use ratatui::style::Color;
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use validate_theme_derive::ValidateTheme;

#[cfg(test)]
#[path = "theme_tests.rs"]
mod theme_tests;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Derivative)]
#[derivative(Default)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Background {
  #[serde(
    deserialize_with = "deserialize_color_str",
    serialize_with = "serialize_color_str",
    default = "default_background_color"
  )]
  #[derivative(Default(value = "Some(Color::Rgb(35, 50, 55))"))]
  pub color: Option<Color>,
  #[derivative(Default(value = "Some(true)"))]
  #[serde(default = "default_background_enabled")]
  pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Style {
  #[serde(
    deserialize_with = "deserialize_color_str",
    serialize_with = "serialize_color_str"
  )]
  pub color: Option<Color>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Derivative, ValidateTheme)]
#[derivative(Default)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Theme {
  #[serde(default = "default_background")]
  #[derivative(Default(
    value = "Some(Background { color: Some(Color::Rgb(35, 50, 55)), enabled: Some(true) })"
  ))]
  pub background: Option<Background>,
  #[validate]
  #[serde(default = "default_awaiting_import_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Rgb(255, 170, 66)) })"))]
  pub awaiting_import: Option<Style>,
  #[validate]
  #[serde(default = "default_indeterminate_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Rgb(255, 170, 66)) })"))]
  pub indeterminate: Option<Style>,
  #[validate]
  #[serde(default = "default_default_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::White) })"))]
  pub default: Option<Style>,
  #[validate]
  #[serde(default = "default_downloaded_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Green) })"))]
  pub downloaded: Option<Style>,
  #[validate]
  #[serde(default = "default_downloading_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Magenta) })"))]
  pub downloading: Option<Style>,
  #[validate]
  #[serde(default = "default_failure_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Red) })"))]
  pub failure: Option<Style>,
  #[validate]
  #[serde(default = "default_help_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::LightBlue) })"))]
  pub help: Option<Style>,
  #[validate]
  #[serde(default = "default_missing_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Red) })"))]
  pub missing: Option<Style>,
  #[validate]
  #[serde(default = "default_primary_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Cyan) })"))]
  pub primary: Option<Style>,
  #[validate]
  #[serde(default = "default_secondary_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Yellow) })"))]
  pub secondary: Option<Style>,
  #[validate]
  #[serde(default = "default_success_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Green) })"))]
  pub success: Option<Style>,
  #[validate]
  #[serde(default = "default_system_function_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Yellow) })"))]
  pub system_function: Option<Style>,
  #[validate]
  #[serde(default = "default_unmonitored_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Gray) })"))]
  pub unmonitored: Option<Style>,
  #[validate]
  #[serde(default = "default_unmonitored_missing_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Yellow) })"))]
  pub unmonitored_missing: Option<Style>,
  #[validate]
  #[serde(default = "default_unreleased_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::LightCyan) })"))]
  pub unreleased: Option<Style>,
  #[validate]
  #[serde(default = "default_warning_style")]
  #[derivative(Default(value = "Some(Style { color: Some(Color::Magenta) })"))]
  pub warning: Option<Style>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ThemeDefinition {
  pub name: String,
  #[serde(default)]
  pub theme: Theme,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ThemeDefinitionsWrapper {
  pub theme_definitions: Vec<ThemeDefinition>,
}

impl Default for ThemeDefinitionsWrapper {
  fn default() -> Self {
    Self {
      theme_definitions: get_builtin_themes(),
    }
  }
}

fn default_background_color() -> Option<Color> {
  Some(Color::Rgb(35, 50, 55))
}

fn default_background_enabled() -> Option<bool> {
  Some(true)
}

fn default_background() -> Option<Background> {
  Some(Background {
    color: default_background_color(),
    enabled: Some(true),
  })
}

fn default_awaiting_import_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Rgb(255, 170, 66)),
  })
}

fn default_indeterminate_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Rgb(255, 170, 66)),
  })
}

fn default_default_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::White),
  })
}

fn default_downloaded_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Green),
  })
}

fn default_downloading_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Magenta),
  })
}

fn default_failure_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Red),
  })
}

fn default_help_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::LightBlue),
  })
}

fn default_missing_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Red),
  })
}

fn default_primary_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Cyan),
  })
}

fn default_secondary_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Yellow),
  })
}

fn default_success_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Green),
  })
}

fn default_system_function_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Yellow),
  })
}

fn default_unmonitored_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Gray),
  })
}

fn default_unmonitored_missing_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Yellow),
  })
}

fn default_unreleased_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::LightCyan),
  })
}

fn default_warning_style() -> Option<Style> {
  Some(Style {
    color: Some(Color::Magenta),
  })
}

impl<'de> Deserialize<'de> for ThemeDefinitionsWrapper {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let theme_definitions = Vec::<ThemeDefinition>::deserialize(deserializer)?;
    Ok(ThemeDefinitionsWrapper { theme_definitions })
  }
}

impl Serialize for ThemeDefinitionsWrapper {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.theme_definitions.serialize(serializer)
  }
}

fn deserialize_color_str<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
where
  D: Deserializer<'de>,
{
  let s: Option<String> = Option::deserialize(deserializer)?;
  match s {
    Some(s) => Color::from_str(&s)
      .map_err(serde::de::Error::custom)
      .map(Some),
    None => Ok(None),
  }
}

fn serialize_color_str<S>(color: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  serializer.serialize_str(&color.unwrap().to_string())
}
