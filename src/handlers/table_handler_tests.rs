#[cfg(test)]
mod tests {
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handle_table_events;
  use crate::handlers::table_handler::TableHandlingConfig;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::Movie;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::servarr_models::Language;
  use crate::models::stateful_table::SortOption;
  use rstest::rstest;

  struct TableHandlerUnit<'a, 'b> {
    key: Key,
    app: &'a mut App<'b>,
    active_radarr_block: ActiveRadarrBlock,
    _context: Option<ActiveRadarrBlock>,
  }

  impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for TableHandlerUnit<'a, 'b> {
    fn handle(&mut self) {
      let movie_table_handling_config = TableHandlingConfig::new(ActiveRadarrBlock::Movies.into())
        .sorting_block(ActiveRadarrBlock::MoviesSortPrompt.into())
        .sort_by_fn(|a: &Movie, b: &Movie| a.id.cmp(&b.id))
        .sort_options(sort_options())
        .searching_block(ActiveRadarrBlock::SearchMovie.into())
        .search_error_block(ActiveRadarrBlock::SearchMovieError.into())
        .search_field_fn(|movie| &movie.title.text)
        .filtering_block(ActiveRadarrBlock::FilterMovies.into())
        .filter_error_block(ActiveRadarrBlock::FilterMoviesError.into())
        .filter_field_fn(|movie| &movie.title.text);
      let minimal_movie_table_handling_config =
        TableHandlingConfig::new(ActiveRadarrBlock::Movies.into());

      match self.active_radarr_block {
        ActiveRadarrBlock::MovieDetails => {
          self.handle_movies_table_events(minimal_movie_table_handling_config);
        }
        _ => {
          self.handle_movies_table_events(movie_table_handling_config);
        }
      }
    }

    fn accepts(_: ActiveRadarrBlock) -> bool {
      true
    }

    fn ignore_special_keys(&self) -> bool {
      self.app.ignore_special_keys_for_textbox_input
    }

    fn new(
      key: Key,
      app: &'a mut App<'b>,
      active_block: ActiveRadarrBlock,
      _context: Option<ActiveRadarrBlock>,
    ) -> Self {
      Self {
        key,
        app,
        active_radarr_block: active_block,
        _context,
      }
    }

    fn get_key(&self) -> Key {
      self.key
    }

    fn is_ready(&self) -> bool {
      !self.app.is_loading
    }

    fn handle_scroll_up(&mut self) {}

    fn handle_scroll_down(&mut self) {}

    fn handle_home(&mut self) {}

    fn handle_end(&mut self) {}

    fn handle_delete(&mut self) {}

    fn handle_left_right_action(&mut self) {}

    fn handle_submit(&mut self) {}

    fn handle_esc(&mut self) {}

    fn handle_char_key_event(&mut self) {}
  }

  impl TableHandlerUnit<'_, '_> {
    handle_table_events!(self, movies, self.app.data.radarr_data.movies, Movie);
  }

  mod test_handle_scroll_up_and_down {
    use super::*;
    use crate::models::HorizontallyScrollableText;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};
    use pretty_assertions::assert_str_eq;

    test_iterable_scroll!(
      test_table_scroll,
      TableHandlerUnit,
      radarr_data,
      movies,
      simple_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    #[rstest]
    fn test_table_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .radarr_data
        .movies
        .set_items(simple_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));

      TableHandlerUnit::new(key, &mut app, ActiveRadarrBlock::Movies, None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movies
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );

      TableHandlerUnit::new(key, &mut app, ActiveRadarrBlock::Movies, None).handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_table_sort_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let movie_field_vec = sort_options();
      let mut app = App::test_default();
      app.data.radarr_data.movies.sorting(sort_options());

      if key == Key::Up {
        for i in (0..movie_field_vec.len()).rev() {
          TableHandlerUnit::new(key, &mut app, ActiveRadarrBlock::MoviesSortPrompt, None).handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .movies
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &movie_field_vec[i]
          );
        }
      } else {
        for i in 0..movie_field_vec.len() {
          TableHandlerUnit::new(key, &mut app, ActiveRadarrBlock::MoviesSortPrompt, None).handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .movies
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &movie_field_vec[(i + 1) % movie_field_vec.len()]
          );
        }
      }
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use std::sync::atomic::Ordering::SeqCst;

    use super::*;
    use crate::models::HorizontallyScrollableText;
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    test_iterable_home_and_end!(
      test_table_home_end,
      TableHandlerUnit,
      radarr_data,
      movies,
      extended_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    #[test]
    fn test_table_home_end_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movies
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movies
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }

    #[test]
    fn test_movie_search_box_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.data.radarr_data.movies.search = Some("Test".into());

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::SearchMovie,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        4
      );

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::SearchMovie,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_movie_filter_box_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.data.radarr_data.movies.filter = Some("Test".into());

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::FilterMovies,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        4
      );

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::FilterMovies,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_table_sort_home_end() {
      let movie_field_vec = sort_options();
      let mut app = App::test_default();
      app.data.radarr_data.movies.sorting(sort_options());

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::MoviesSortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &movie_field_vec[movie_field_vec.len() - 1]
      );

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::MoviesSortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &movie_field_vec[0]
      );
    }
  }

  mod test_handle_pagination_scroll {
    use super::*;
    use crate::handlers::table_handler::table_handler_tests::tests::TableHandlerUnit;
    use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
    use pretty_assertions::assert_str_eq;
    use rstest::rstest;
    use std::iter;

    #[rstest]
    fn test_table_pagination_scroll(
      #[values(DEFAULT_KEYBINDINGS.pg_up.key, DEFAULT_KEYBINDINGS.pg_down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      let mut curr = 0;
      let movies_vec = iter::repeat_with(|| {
        let tmp = curr;
        curr += 1;
        Movie {
          title: format!("Test {tmp}").into(),
          ..Movie::default()
        }
      })
      .take(100)
      .collect();
      app.data.radarr_data.movies.set_items(movies_vec);
      TableHandlerUnit::new(key, &mut app, ActiveRadarrBlock::Movies, None).handle();

      if key == Key::PgUp {
        assert_str_eq!(
          app.data.radarr_data.movies.current_selection().title.text,
          "Test 79"
        );
      } else {
        assert_str_eq!(
          app.data.radarr_data.movies.current_selection().title.text,
          "Test 20"
        );
      }
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use std::sync::atomic::Ordering::SeqCst;

    use super::*;

    #[test]
    fn test_movie_search_box_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.data.radarr_data.movies.search = Some("Test".into());

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::SearchMovie,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        1
      );

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::SearchMovie,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_movie_filter_box_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.data.radarr_data.movies.filter = Some("Test".into());

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::FilterMovies,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        1
      );

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::FilterMovies,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::extended_stateful_iterable_vec;
    use crate::models::HorizontallyScrollableText;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_search_movie_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.search = Some("Test 2".into());

      TableHandlerUnit::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::SearchMovie, None).handle();

      assert_str_eq!(
        app.data.radarr_data.movies.current_selection().title.text,
        "Test 2"
      );
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_search_movie_submit_error_on_no_search_hits() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.search = Some("Test 5".into());

      TableHandlerUnit::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::SearchMovie, None).handle();

      assert_str_eq!(
        app.data.radarr_data.movies.current_selection().title.text,
        "Test 1"
      );
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::SearchMovieError.into()
      );
    }

    #[test]
    fn test_search_filtered_table_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app
        .data
        .radarr_data
        .movies
        .set_filtered_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.search = Some("Test 2".into());

      TableHandlerUnit::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::SearchMovie, None).handle();

      assert_str_eq!(
        app.data.radarr_data.movies.current_selection().title.text,
        "Test 2"
      );
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_filter_table_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.filter = Some("Test".into());

      TableHandlerUnit::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::FilterMovies, None).handle();

      assert!(app.data.radarr_data.movies.filtered_items.is_some());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .filtered_items
          .as_ref()
          .unwrap()
          .len(),
        3
      );
      assert_str_eq!(
        app.data.radarr_data.movies.current_selection().title.text,
        "Test 1"
      );
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_filter_table_submit_error_on_no_filter_matches() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.filter = Some("Test 5".into());

      TableHandlerUnit::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::FilterMovies, None).handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(app.data.radarr_data.movies.filtered_items.is_none());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::FilterMoviesError.into()
      );
    }

    #[test]
    fn test_table_sort_prompt_submit() {
      let mut app = App::test_default();
      app.data.radarr_data.movies.sort_asc = true;
      app.data.radarr_data.movies.sorting(sort_options());
      app.data.radarr_data.movies.set_items(movies_vec());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::MoviesSortPrompt.into());

      let mut expected_vec = movies_vec();
      expected_vec.sort_by(|a, b| a.id.cmp(&b.id));
      expected_vec.reverse();

      TableHandlerUnit::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::MoviesSortPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.movies.items, expected_vec);
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;

    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::stateful_table::StatefulTable;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_search_movie_block_esc(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchMovieError)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.movies.search = Some("Test".into());

      TableHandlerUnit::new(ESC_KEY, &mut app, active_radarr_block, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_eq!(app.data.radarr_data.movies.search, None);
    }

    #[rstest]
    fn test_filter_table_block_esc(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterMoviesError)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.movies = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(ESC_KEY, &mut app, active_radarr_block, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_eq!(app.data.radarr_data.movies.filter, None);
      assert_eq!(app.data.radarr_data.movies.filtered_items, None);
      assert_eq!(app.data.radarr_data.movies.filtered_state, None);
    }

    #[test]
    fn test_table_sort_prompt_block_esc() {
      let mut app = App::test_default();
      app.data.radarr_data.movies.set_items(movies_vec());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::MoviesSortPrompt.into());

      TableHandlerUnit::new(ESC_KEY, &mut app, ActiveRadarrBlock::MoviesSortPrompt, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_key_char {
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::HorizontallyScrollableText;
    use pretty_assertions::{assert_eq, assert_str_eq};

    use super::*;

    #[test]
    fn test_search_table_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::SearchMovie.into()
      );
      assert!(app.ignore_special_keys_for_textbox_input);
      assert_eq!(
        app.data.radarr_data.movies.search,
        Some(HorizontallyScrollableText::default())
      );
    }

    #[test]
    fn test_search_table_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_eq!(app.data.radarr_data.movies.search, None);
    }

    #[test]
    fn test_search_table_key_no_op_when_search_table_block_is_not_defined() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveRadarrBlock::MovieDetails,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_eq!(app.data.radarr_data.movies.search, None);
    }

    #[test]
    fn test_filter_table_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::FilterMovies.into()
      );
      assert!(app.ignore_special_keys_for_textbox_input);
      assert!(app.data.radarr_data.movies.filter.is_some());
    }

    #[test]
    fn test_filter_table_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(app.data.radarr_data.movies.filter.is_none());
    }

    #[test]
    fn test_filter_table_key_resets_previous_filter() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.data.radarr_data = create_test_radarr_data();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.data.radarr_data.movies.filter = Some("Test".into());

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::FilterMovies.into()
      );
      assert!(app.ignore_special_keys_for_textbox_input);
      assert_eq!(
        app.data.radarr_data.movies.filter,
        Some(HorizontallyScrollableText::default())
      );
      assert!(app.data.radarr_data.movies.filtered_items.is_none());
      assert!(app.data.radarr_data.movies.filtered_state.is_none());
    }

    #[test]
    fn test_filter_table_key_no_op_when_filter_table_block_is_not_defined() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveRadarrBlock::MovieDetails,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_eq!(app.data.radarr_data.movies.filter, None);
    }

    #[test]
    fn test_search_table_box_backspace_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app.data.radarr_data.movies.search = Some("Test".into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::SearchMovie,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.search.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_filter_table_box_backspace_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.data.radarr_data.movies.filter = Some("Test".into());

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::FilterMovies,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.filter.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_search_table_box_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.data.radarr_data.movies.search = Some(HorizontallyScrollableText::default());

      TableHandlerUnit::new(
        Key::Char('a'),
        &mut app,
        ActiveRadarrBlock::SearchMovie,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.search.as_ref().unwrap().text,
        "a"
      );
    }

    #[test]
    fn test_filter_table_box_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.data.radarr_data.movies.filter = Some(HorizontallyScrollableText::default());

      TableHandlerUnit::new(
        Key::Char('a'),
        &mut app,
        ActiveRadarrBlock::FilterMovies,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.filter.as_ref().unwrap().text,
        "a"
      );
    }

    #[test]
    fn test_sort_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::MoviesSortPrompt.into()
      );
      assert_eq!(
        app.data.radarr_data.movies.sort.as_ref().unwrap().items,
        sort_options()
      );
      assert!(!app.data.radarr_data.movies.sort_asc);
    }

    #[test]
    fn test_sort_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(app.data.radarr_data.movies.sort.is_none());
    }

    #[test]
    fn test_sort_key_no_op_when_sort_table_block_is_undefined() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      TableHandlerUnit::new(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveRadarrBlock::MovieDetails,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(app.data.radarr_data.movies.sort.is_none());
    }
  }

  fn movies_vec() -> Vec<Movie> {
    vec![
      Movie {
        id: 3,
        title: "test 1".into(),
        original_language: Language {
          id: 1,
          name: "English".to_owned(),
        },
        size_on_disk: 1024,
        studio: Some("Studio 1".to_owned()),
        year: 2024,
        monitored: false,
        runtime: 12.into(),
        quality_profile_id: 1,
        certification: Some("PG-13".to_owned()),
        tags: vec![1.into(), 2.into()],
        ..Movie::default()
      },
      Movie {
        id: 2,
        title: "test 2".into(),
        original_language: Language {
          id: 2,
          name: "Chinese".to_owned(),
        },
        size_on_disk: 2048,
        studio: Some("Studio 2".to_owned()),
        year: 1998,
        monitored: false,
        runtime: 60.into(),
        quality_profile_id: 2,
        certification: Some("R".to_owned()),
        tags: vec![1.into(), 3.into()],
        ..Movie::default()
      },
      Movie {
        id: 1,
        title: "test 3".into(),
        original_language: Language {
          id: 3,
          name: "Japanese".to_owned(),
        },
        size_on_disk: 512,
        studio: Some("studio 3".to_owned()),
        year: 1954,
        monitored: true,
        runtime: 120.into(),
        quality_profile_id: 3,
        certification: Some("G".to_owned()),
        tags: vec![2.into(), 3.into()],
        ..Movie::default()
      },
    ]
  }

  fn sort_options() -> Vec<SortOption<Movie>> {
    vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| {
        b.title
          .text
          .to_lowercase()
          .cmp(&a.title.text.to_lowercase())
      }),
    }]
  }
}
