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
    THEME.with(|theme| self.fg(theme.get().awaiting_import.unwrap().color.unwrap()))
  }

  fn indeterminate(self) -> T {
    THEME.with(|theme| self.fg(theme.get().indeterminate.unwrap().color.unwrap()))
  }

  fn default(self) -> T {
    THEME.with(|theme| self.fg(theme.get().default.unwrap().color.unwrap()))
  }

  fn downloaded(self) -> T {
    THEME.with(|theme| self.fg(theme.get().downloaded.unwrap().color.unwrap()))
  }

  fn downloading(self) -> T {
    THEME.with(|theme| self.fg(theme.get().downloading.unwrap().color.unwrap()))
  }

  fn failure(self) -> T {
    THEME.with(|theme| self.fg(theme.get().failure.unwrap().color.unwrap()))
  }

  fn help(self) -> T {
    THEME.with(|theme| self.fg(theme.get().help.unwrap().color.unwrap()))
  }

  fn highlight(self) -> T {
    self.reversed()
  }

  fn missing(self) -> T {
    THEME.with(|theme| self.fg(theme.get().missing.unwrap().color.unwrap()))
  }

  fn primary(self) -> T {
    THEME.with(|theme| self.fg(theme.get().primary.unwrap().color.unwrap()))
  }

  fn secondary(self) -> T {
    THEME.with(|theme| self.fg(theme.get().secondary.unwrap().color.unwrap()))
  }

  fn success(self) -> T {
    THEME.with(|theme| self.fg(theme.get().success.unwrap().color.unwrap()))
  }

  fn system_function(self) -> T {
    THEME.with(|theme| self.fg(theme.get().system_function.unwrap().color.unwrap()))
  }

  fn unmonitored(self) -> T {
    THEME.with(|theme| self.fg(theme.get().unmonitored.unwrap().color.unwrap()))
  }

  fn unmonitored_missing(self) -> T {
    THEME.with(|theme| self.fg(theme.get().unmonitored_missing.unwrap().color.unwrap()))
  }

  fn unreleased(self) -> T {
    THEME.with(|theme| self.fg(theme.get().unreleased.unwrap().color.unwrap()))
  }

  fn warning(self) -> T {
    THEME.with(|theme| self.fg(theme.get().warning.unwrap().color.unwrap()))
  }
}
