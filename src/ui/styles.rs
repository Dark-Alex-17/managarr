use crate::ui::THEME;
use ratatui::style::{Styled, Stylize};

#[cfg(test)]
#[path = "styles_tests.rs"]
mod styles_tests;

pub trait ManagarrStyle<'a, T>: Stylize<'a, T>
where
  T: Default,
{
  #[allow(clippy::new_ret_no_self)]
  fn new() -> T;
  fn awaiting_import(self) -> T;
  fn indeterminate(self) -> T;
  fn default(self) -> T;
  fn downloaded(self) -> T;
  fn downloading(self) -> T;
  fn failure(self) -> T;
  fn help(self) -> T;
  fn highlight(self) -> T;
  fn missing(self) -> T;
  fn primary(self) -> T;
  fn secondary(self) -> T;
  fn success(self) -> T;
  fn system_function(self) -> T;
  fn unmonitored(self) -> T;
  fn unmonitored_missing(self) -> T;
  fn unreleased(self) -> T;
  fn warning(self) -> T;
}

impl<T, U> ManagarrStyle<'_, T> for U
where
  U: Styled<Item = T>,
  T: Default,
{
  fn new() -> T {
    T::default()
  }

  fn awaiting_import(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .awaiting_import
          .expect("awaiting_import style must be defined in theme")
          .color
          .expect("awaiting_import color must be defined"),
      )
    })
  }

  fn indeterminate(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .indeterminate
          .expect("indeterminate style must be defined in theme")
          .color
          .expect("indeterminate color must be defined"),
      )
    })
  }

  fn default(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .default
          .expect("default style must be defined in theme")
          .color
          .expect("default color must be defined"),
      )
    })
  }

  fn downloaded(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .downloaded
          .expect("downloaded style must be defined in theme")
          .color
          .expect("downloaded color must be defined"),
      )
    })
  }

  fn downloading(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .downloading
          .expect("downloading style must be defined in theme")
          .color
          .expect("downloading color must be defined"),
      )
    })
  }

  fn failure(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .failure
          .expect("failure style must be defined in theme")
          .color
          .expect("failure color must be defined"),
      )
    })
  }

  fn help(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .help
          .expect("help style must be defined in theme")
          .color
          .expect("help color must be defined"),
      )
    })
  }

  fn highlight(self) -> T {
    self.reversed()
  }

  fn missing(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .missing
          .expect("missing style must be defined in theme")
          .color
          .expect("missing color must be defined"),
      )
    })
  }

  fn primary(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .primary
          .expect("primary style must be defined in theme")
          .color
          .expect("primary color must be defined"),
      )
    })
  }

  fn secondary(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .secondary
          .expect("secondary style must be defined in theme")
          .color
          .expect("secondary color must be defined"),
      )
    })
  }

  fn success(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .success
          .expect("success style must be defined in theme")
          .color
          .expect("success color must be defined"),
      )
    })
  }

  fn system_function(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .system_function
          .expect("system_function style must be defined in theme")
          .color
          .expect("system_function color must be defined"),
      )
    })
  }

  fn unmonitored(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .unmonitored
          .expect("unmonitored style must be defined in theme")
          .color
          .expect("unmonitored color must be defined"),
      )
    })
  }

  fn unmonitored_missing(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .unmonitored_missing
          .expect("unmonitored_missing style must be defined in theme")
          .color
          .expect("unmonitored_missing color must be defined"),
      )
    })
  }

  fn unreleased(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .unreleased
          .expect("unreleased style must be defined in theme")
          .color
          .expect("unreleased color must be defined"),
      )
    })
  }

  fn warning(self) -> T {
    THEME.with(|theme| {
      self.fg(
        theme
          .get()
          .warning
          .expect("warning style must be defined in theme")
          .color
          .expect("warning color must be defined"),
      )
    })
  }
}
