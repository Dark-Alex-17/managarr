use ratatui::prelude::Color;
use ratatui::style::{Styled, Stylize};

pub const COLOR_ORANGE: Color = Color::Rgb(255, 170, 66);

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
    self.fg(COLOR_ORANGE)
  }

  fn indeterminate(self) -> T {
    self.fg(COLOR_ORANGE)
  }

  fn default(self) -> T {
    self.white()
  }

  fn downloaded(self) -> T {
    self.green()
  }

  fn downloading(self) -> T {
    self.magenta()
  }

  fn failure(self) -> T {
    self.red()
  }

  fn help(self) -> T {
    self.light_blue()
  }

  fn highlight(self) -> T {
    self.reversed()
  }

  fn missing(self) -> T {
    self.red()
  }

  fn primary(self) -> T {
    self.cyan()
  }

  fn secondary(self) -> T {
    self.yellow()
  }

  fn success(self) -> T {
    self.green()
  }

  fn system_function(self) -> T {
    self.yellow()
  }

  fn unmonitored(self) -> T {
    self.gray()
  }

  fn unmonitored_missing(self) -> T {
    self.yellow()
  }

  fn unreleased(self) -> T {
    self.light_cyan()
  }

  fn warning(self) -> T {
    self.magenta()
  }
}
