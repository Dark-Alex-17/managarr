use crate::ui::THEME;
use ratatui::style::{Style, Styled};

#[cfg(test)]
#[path = "styles_tests.rs"]
mod styles_tests;

pub fn awaiting_import_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .awaiting_import
        .expect("awaiting_import style must be defined in theme")
        .color
        .expect("awaiting_import color must be defined"),
    )
  })
}

pub fn indeterminate_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .indeterminate
        .expect("indeterminate style must be defined in theme")
        .color
        .expect("indeterminate color must be defined"),
    )
  })
}

pub fn default_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .default
        .expect("default style must be defined in theme")
        .color
        .expect("default color must be defined"),
    )
  })
}

pub fn downloaded_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .downloaded
        .expect("downloaded style must be defined in theme")
        .color
        .expect("downloaded color must be defined"),
    )
  })
}

pub fn downloading_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .downloading
        .expect("downloading style must be defined in theme")
        .color
        .expect("downloading color must be defined"),
    )
  })
}

pub fn failure_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .failure
        .expect("failure style must be defined in theme")
        .color
        .expect("failure color must be defined"),
    )
  })
}

pub fn help_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .help
        .expect("help style must be defined in theme")
        .color
        .expect("help color must be defined"),
    )
  })
}

pub fn highlight_style() -> Style {
  Style::new().reversed()
}

pub fn missing_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .missing
        .expect("missing style must be defined in theme")
        .color
        .expect("missing color must be defined"),
    )
  })
}

pub fn primary_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .primary
        .expect("primary style must be defined in theme")
        .color
        .expect("primary color must be defined"),
    )
  })
}

pub fn secondary_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .secondary
        .expect("secondary style must be defined in theme")
        .color
        .expect("secondary color must be defined"),
    )
  })
}

pub fn success_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .success
        .expect("success style must be defined in theme")
        .color
        .expect("success color must be defined"),
    )
  })
}

pub fn system_function_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .system_function
        .expect("system_function style must be defined in theme")
        .color
        .expect("system_function color must be defined"),
    )
  })
}

pub fn unmonitored_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .unmonitored
        .expect("unmonitored style must be defined in theme")
        .color
        .expect("unmonitored color must be defined"),
    )
  })
}

pub fn unmonitored_missing_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .unmonitored_missing
        .expect("unmonitored_missing style must be defined in theme")
        .color
        .expect("unmonitored_missing color must be defined"),
    )
  })
}

pub fn unreleased_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .unreleased
        .expect("unreleased style must be defined in theme")
        .color
        .expect("unreleased color must be defined"),
    )
  })
}

pub fn warning_style() -> Style {
  THEME.with(|theme| {
    Style::new().fg(
      theme
        .get()
        .warning
        .expect("warning style must be defined in theme")
        .color
        .expect("warning color must be defined"),
    )
  })
}

pub trait ManagarrStyle: Styled {
  fn awaiting_import(self) -> Self::Item;
  fn indeterminate(self) -> Self::Item;
  fn default_color(self) -> Self::Item;
  fn downloaded(self) -> Self::Item;
  fn downloading(self) -> Self::Item;
  fn failure(self) -> Self::Item;
  fn help(self) -> Self::Item;
  fn missing(self) -> Self::Item;
  fn primary(self) -> Self::Item;
  fn secondary(self) -> Self::Item;
  fn success(self) -> Self::Item;
  fn system_function(self) -> Self::Item;
  fn unmonitored(self) -> Self::Item;
  fn unmonitored_missing(self) -> Self::Item;
  fn unreleased(self) -> Self::Item;
  fn warning(self) -> Self::Item;
}

impl<T: Styled> ManagarrStyle for T {
  fn awaiting_import(self) -> Self::Item {
    self.set_style(awaiting_import_style())
  }

  fn indeterminate(self) -> Self::Item {
    self.set_style(indeterminate_style())
  }

  fn default_color(self) -> Self::Item {
    self.set_style(default_style())
  }

  fn downloaded(self) -> Self::Item {
    self.set_style(downloaded_style())
  }

  fn downloading(self) -> Self::Item {
    self.set_style(downloading_style())
  }

  fn failure(self) -> Self::Item {
    self.set_style(failure_style())
  }

  fn help(self) -> Self::Item {
    self.set_style(help_style())
  }

  fn missing(self) -> Self::Item {
    self.set_style(missing_style())
  }

  fn primary(self) -> Self::Item {
    self.set_style(primary_style())
  }

  fn secondary(self) -> Self::Item {
    self.set_style(secondary_style())
  }

  fn success(self) -> Self::Item {
    self.set_style(success_style())
  }

  fn system_function(self) -> Self::Item {
    self.set_style(system_function_style())
  }

  fn unmonitored(self) -> Self::Item {
    self.set_style(unmonitored_style())
  }

  fn unmonitored_missing(self) -> Self::Item {
    self.set_style(unmonitored_missing_style())
  }

  fn unreleased(self) -> Self::Item {
    self.set_style(unreleased_style())
  }

  fn warning(self) -> Self::Item {
    self.set_style(warning_style())
  }
}
