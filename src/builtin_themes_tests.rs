#[cfg(test)]
mod test {
  use crate::builtin_themes::get_builtin_themes;
  use crate::ui::theme::{Background, Style, Theme, ThemeDefinition};
  use pretty_assertions::assert_eq;
  use ratatui::prelude::Color;
  use std::str::FromStr;

  #[test]
  fn test_builtin_themes() {
    let watermelon_dark = Theme {
      background: Some(Background {
        enabled: Some(false),
        color: Some(Color::from_str("#233237").unwrap()),
      }),
      default: Some(Style {
        color: Some(Color::from_str("#00FF00").unwrap()),
      }),
      downloaded: Some(Style {
        color: Some(Color::from_str("#80ffbf").unwrap()),
      }),
      failure: Some(Style {
        color: Some(Color::from_str("#ff8080").unwrap()),
      }),
      missing: Some(Style {
        color: Some(Color::from_str("#ff8080").unwrap()),
      }),
      primary: Some(Style {
        color: Some(Color::from_str("#ff19d9").unwrap()),
      }),
      secondary: Some(Style {
        color: Some(Color::from_str("#8c19ff").unwrap()),
      }),
      ..Theme::default()
    };
    let dracula = Theme {
      background: Some(Background {
        enabled: Some(false),
        color: Some(Color::from_str("#233237").unwrap()),
      }),
      default: Some(Style {
        color: Some(Color::from_str("#f8f8f2").unwrap()),
      }),
      downloaded: Some(Style {
        color: Some(Color::from_str("#50fa7b").unwrap()),
      }),
      downloading: Some(Style {
        color: Some(Color::from_str("#f1fa8c").unwrap()),
      }),
      failure: Some(Style {
        color: Some(Color::from_str("#ff5555").unwrap()),
      }),
      missing: Some(Style {
        color: Some(Color::from_str("#ffb86c").unwrap()),
      }),
      primary: Some(Style {
        color: Some(Color::from_str("#ff79c6").unwrap()),
      }),
      secondary: Some(Style {
        color: Some(Color::from_str("#ff79c6").unwrap()),
      }),
      unmonitored_missing: Some(Style {
        color: Some(Color::from_str("#6272a4").unwrap()),
      }),
      help: Some(Style {
        color: Some(Color::from_str("#6272a4").unwrap()),
      }),
      success: Some(Style {
        color: Some(Color::from_str("#50fa7b").unwrap()),
      }),
      warning: Some(Style {
        color: Some(Color::from_str("#f1fa8c").unwrap()),
      }),
      unreleased: Some(Style {
        color: Some(Color::from_str("#f8f8f2").unwrap()),
      }),
      ..Theme::default()
    };
    let expected_themes = vec![
      ThemeDefinition {
        name: "default".to_owned(),
        theme: Theme::default(),
      },
      ThemeDefinition {
        name: "watermelon-dark".to_owned(),
        theme: watermelon_dark,
      },
      ThemeDefinition {
        name: "dracula".to_owned(),
        theme: dracula,
      },
    ];

    assert_eq!(expected_themes, get_builtin_themes());
  }
}
