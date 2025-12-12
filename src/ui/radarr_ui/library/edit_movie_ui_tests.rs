#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::radarr_models::Movie;
  use crate::models::servarr_data::radarr::modals::EditMovieModal;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_MOVIE_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS,
  };
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::edit_movie_ui::EditMovieUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_edit_movie_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(EditMovieUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!EditMovieUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[test]
  fn test_edit_movie_ui_renders_edit_movie_modal() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::EditMoviePathInput.into());
    app.data.radarr_data.quality_profile_map =
      BiMap::from_iter(vec![(1, "HD - 1080p".to_owned()), (2, "Any".to_owned())]);
    app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
    app.data.radarr_data.movies = StatefulTable::default();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      title: "Test Movie".into(),
      path: "/movies/test".into(),
      quality_profile_id: 1,
      ..Movie::default()
    }]);
    app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::from(&app.data.radarr_data));

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      EditMovieUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
