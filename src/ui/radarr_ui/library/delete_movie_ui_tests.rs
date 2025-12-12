#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::radarr_models::Movie;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, DELETE_MOVIE_BLOCKS, DELETE_MOVIE_SELECTION_BLOCKS,
  };
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::delete_movie_ui::DeleteMovieUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_delete_movie_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if DELETE_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(DeleteMovieUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!DeleteMovieUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[test]
  fn test_delete_movie_ui_renders_delete_movie_prompt() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
    app.data.radarr_data.quality_profile_map = BiMap::from_iter(vec![(0, "Any".to_owned())]);
    app.data.radarr_data.selected_block = BlockSelectionState::new(DELETE_MOVIE_SELECTION_BLOCKS);
    app.data.radarr_data.movies = StatefulTable::default();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      title: "Test Movie".into(),
      quality_profile_id: 0,
      ..Movie::default()
    }]);

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      DeleteMovieUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
