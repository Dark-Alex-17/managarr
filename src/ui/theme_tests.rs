mod tests {
  use crate::ui::theme::{Background, Style, Theme, ThemeDefinition, ThemeDefinitionsWrapper};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use ratatui::style::Color;
  use std::str::FromStr;

  #[test]
  fn test_background_default() {
    let expected_background = Background {
      enabled: Some(true),
      color: Some(Color::Rgb(35, 50, 55)),
    };

    assert_eq!(Background::default(), expected_background);
  }

  #[test]
  fn test_theme_default() {
    let expected_theme = Theme {
      background: Some(Background {
        enabled: Some(true),
        color: Some(Color::Rgb(35, 50, 55)),
      }),
      awaiting_import: Some(Style {
        color: Some(Color::Rgb(255, 170, 66)),
      }),
      indeterminate: Some(Style {
        color: Some(Color::Rgb(255, 170, 66)),
      }),
      default: Some(Style {
        color: Some(Color::White),
      }),
      downloaded: Some(Style {
        color: Some(Color::Green),
      }),
      downloading: Some(Style {
        color: Some(Color::Magenta),
      }),
      failure: Some(Style {
        color: Some(Color::Red),
      }),
      help: Some(Style {
        color: Some(Color::LightBlue),
      }),
      missing: Some(Style {
        color: Some(Color::Red),
      }),
      primary: Some(Style {
        color: Some(Color::Cyan),
      }),
      secondary: Some(Style {
        color: Some(Color::Yellow),
      }),
      success: Some(Style {
        color: Some(Color::Green),
      }),
      system_function: Some(Style {
        color: Some(Color::Yellow),
      }),
      unmonitored: Some(Style {
        color: Some(Color::Gray),
      }),
      unmonitored_missing: Some(Style {
        color: Some(Color::Yellow),
      }),
      unreleased: Some(Style {
        color: Some(Color::LightCyan),
      }),
      warning: Some(Style {
        color: Some(Color::Magenta),
      }),
    };

    assert_eq!(Theme::default(), expected_theme);
  }

  #[test]
  fn test_default_theme_definition() {
    let expected_theme_definition = ThemeDefinition {
      name: String::new(),
      theme: Theme::default(),
    };

    assert_eq!(ThemeDefinition::default(), expected_theme_definition);
  }

  #[test]
  fn test_deserialization_defaults_to_using_default_theme_values_when_missing() {
    let theme_yaml = r#""#;
    let theme: Theme = serde_yaml::from_str(theme_yaml).unwrap();

    assert_eq!(theme, Theme::default());
  }

  #[test]
  fn test_deserialization_does_not_overwrite_non_empty_fields_with_default_values() {
    let theme_yaml = r###"
background:
  enabled: false
  color: "#000000"
awaiting_import:
  color: "#000000"
indeterminate:
  color: "#000000"
default:
  color: "#000000"
downloaded:
  color: "#000000"
downloading:
  color: "#000000"
failure:
  color: "#000000"
help:
  color: "#000000"
missing:
  color: "#000000"
primary:
  color: "#000000"
secondary:
  color: "#000000"
success:
  color: "#000000"
system_function:
  color: "#000000"
unmonitored:
  color: "#000000"
unmonitored_missing:
  color: "#000000"
unreleased:
  color: "#000000"
warning:
  color: "#000000"
"###;
    let theme: Theme = serde_yaml::from_str(theme_yaml).unwrap();
    let expected_theme = Theme {
      background: Some(Background {
        enabled: Some(false),
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      awaiting_import: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      indeterminate: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      default: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      downloaded: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      downloading: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      failure: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      help: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      missing: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      primary: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      secondary: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      success: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      system_function: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      unmonitored: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      unmonitored_missing: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      unreleased: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
      warning: Some(Style {
        color: Some(Color::Rgb(0, 0, 0)),
      }),
    };

    assert_eq!(theme, expected_theme);
  }

  #[test]
  fn test_theme_definitions_wrapper_default() {
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
    let theme_definitions_wrapper = ThemeDefinitionsWrapper {
      theme_definitions: vec![
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
      ],
    };

    assert_eq!(
      ThemeDefinitionsWrapper::default(),
      theme_definitions_wrapper
    );
  }

  #[test]
  fn test_theme_definitions_wrapper_deserialization() {
    let theme_definitions = r###"
- name: test
  theme:
    background:
      enabled: false
      color: "#000000"
    awaiting_import:
      color: "#000000"
    indeterminate:
      color: "#000000"
    default:
      color: "#000000"
    downloaded:
      color: "#000000"
    downloading:
      color: "#000000"
    failure:
      color: "#000000"
    help:
      color: "#000000"
    missing:
      color: "#000000"
    primary:
      color: "#000000"
    secondary:
      color: "#000000"
    success:
      color: "#000000"
    system_function:
      color: "#000000"
    unmonitored:
      color: "#000000"
    unmonitored_missing:
      color: "#000000"
    unreleased:
      color: "#000000"
    warning:
      color: "#000000"
"###;
    let theme_definition_wrapper: ThemeDefinitionsWrapper =
      serde_yaml::from_str(theme_definitions).unwrap();
    let expected_theme_definitions = ThemeDefinitionsWrapper {
      theme_definitions: vec![ThemeDefinition {
        name: "test".to_owned(),
        theme: Theme {
          background: Some(Background {
            enabled: Some(false),
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          awaiting_import: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          indeterminate: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          default: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          downloaded: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          downloading: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          failure: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          help: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          missing: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          primary: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          secondary: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          success: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          system_function: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          unmonitored: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          unmonitored_missing: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          unreleased: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
          warning: Some(Style {
            color: Some(Color::Rgb(0, 0, 0)),
          }),
        },
      }],
    };

    assert_eq!(theme_definition_wrapper, expected_theme_definitions);
  }

  #[test]
  fn test_theme_definition_wrapper_serialization() {
    let theme_definition_wrapper = ThemeDefinitionsWrapper {
      theme_definitions: vec![ThemeDefinition::default()],
    };
    let expected_yaml = r###"- name: ''
  theme:
    background:
      color: '#233237'
      enabled: true
    awaiting_import:
      color: '#FFAA42'
    indeterminate:
      color: '#FFAA42'
    default:
      color: White
    downloaded:
      color: Green
    downloading:
      color: Magenta
    failure:
      color: Red
    help:
      color: LightBlue
    missing:
      color: Red
    primary:
      color: Cyan
    secondary:
      color: Yellow
    success:
      color: Green
    system_function:
      color: Yellow
    unmonitored:
      color: Gray
    unmonitored_missing:
      color: Yellow
    unreleased:
      color: LightCyan
    warning:
      color: Magenta
"###;

    let serialized_yaml = serde_yaml::to_string(&theme_definition_wrapper).unwrap();

    assert_str_eq!(serialized_yaml, expected_yaml);
  }
}
