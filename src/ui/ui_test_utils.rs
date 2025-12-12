#[cfg(test)]
#[allow(dead_code)]
pub mod test_utils {
  use ratatui::Frame;
  use ratatui::Terminal;
  use ratatui::backend::TestBackend;
  use ratatui::buffer::Buffer;

  use crate::app::App;

  pub fn create_test_backend(width: u16, height: u16) -> TestBackend {
    TestBackend::new(width, height)
  }

  pub fn create_test_terminal(width: u16, height: u16) -> Terminal<TestBackend> {
    let backend = create_test_backend(width, height);
    Terminal::new(backend).unwrap()
  }

  /// Renders a UI component and returns the output as a string
  ///
  /// # Arguments
  /// * `width` - Terminal width in columns
  /// * `height` - Terminal height in rows
  /// * `render_fn` - Function that renders to the frame
  ///
  /// # Example
  /// ```rust
  /// let output = render_to_string(120, 30, |f| {
  ///   Block::default().title("Test").render(f.area(), f.buffer_mut());
  /// });
  /// ```
  pub fn render_to_string<F>(width: u16, height: u16, mut render_fn: F) -> String
  where
    F: FnMut(&mut Frame),
  {
    let mut terminal = create_test_terminal(width, height);

    terminal
      .draw(|f| {
        render_fn(f);
      })
      .unwrap();

    buffer_to_string(terminal.backend().buffer(), width, height)
  }

  /// Renders a UI component with an App instance and returns the output as a string
  ///
  /// This is the primary helper for testing UI components that need app state.
  ///
  /// # Arguments
  /// * `width` - Terminal width in columns (typically 120)
  /// * `height` - Terminal height in rows (typically 30)
  /// * `app` - Mutable reference to App instance
  /// * `render_fn` - Function that renders to the frame with app
  ///
  /// # Example
  /// ```rust
  /// let mut app = App::test_default();
  /// app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
  ///
  /// let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
  ///   LibraryUi::draw(f, app, f.area());
  /// });
  ///
  /// insta::assert_snapshot!(output);
  /// ```
  pub fn render_to_string_with_app<F>(
    width: u16,
    height: u16,
    app: &mut App,
    mut render_fn: F,
  ) -> String
  where
    F: FnMut(&mut Frame, &mut App),
  {
    let mut terminal = create_test_terminal(width, height);

    terminal
      .draw(|f| {
        render_fn(f, app);
      })
      .unwrap();

    buffer_to_string(terminal.backend().buffer(), width, height)
  }

  fn buffer_to_string(buffer: &Buffer, width: u16, height: u16) -> String {
    let mut result = String::new();

    for y in 0..height {
      for x in 0..width {
        let cell = buffer.cell((x, y)).expect("Cell should exist");
        result.push_str(cell.symbol());
      }
      if y < height - 1 {
        result.push('\n');
      }
    }

    result
  }

  pub fn output_contains(output: &str, text: &str) -> bool {
    output.contains(text)
  }

  pub fn output_contains_all(output: &str, texts: &[&str]) -> bool {
    texts.iter().all(|text| output.contains(text))
  }

  pub fn count_lines(output: &str) -> usize {
    output.lines().count()
  }

  pub fn verify_dimensions(output: &str, max_width: usize, max_height: usize) -> bool {
    let lines: Vec<&str> = output.lines().collect();

    if lines.len() > max_height {
      return false;
    }

    lines.iter().all(|line| line.chars().count() <= max_width)
  }
}
