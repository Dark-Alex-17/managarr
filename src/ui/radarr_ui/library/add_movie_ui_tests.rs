#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::HorizontallyScrollableText;
  use crate::models::servarr_data::radarr::radarr_data::{ADD_MOVIE_BLOCKS, ActiveRadarrBlock};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::add_movie_ui::AddMovieUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_add_movie_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ADD_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(AddMovieUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!AddMovieUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[test]
  fn test_add_movie_ui_renders_loading_state() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
    app.data.radarr_data.add_movie_search = Some(HorizontallyScrollableText::default());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      AddMovieUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_add_movie_ui_renders_search_input() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
    app.data.radarr_data.add_movie_search = Some(HorizontallyScrollableText::default());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      AddMovieUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
