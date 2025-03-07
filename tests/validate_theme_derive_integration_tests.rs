use validate_theme_derive::ValidateTheme;

#[test]
fn test_validate_theme_derive() {
  let theme = Theme {
    name: "test".to_string(),
    good: Some(Style {
      color: "Green".to_owned(),
    }),
    bad: Some(Style {
      color: "Red".to_owned(),
    }),
    ugly: Some(Style {
      color: "Magenta".to_owned(),
    }),
  };

  theme.validate();
}

#[allow(dead_code)]
struct Style {
  color: String,
}

#[allow(dead_code)]
#[derive(ValidateTheme)]
struct Theme {
  pub name: String,
  #[validate]
  pub good: Option<Style>,
  #[validate]
  pub bad: Option<Style>,
  pub ugly: Option<Style>,
}
