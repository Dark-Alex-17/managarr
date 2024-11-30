#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::collections::collection_details_handler::CollectionDetailsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::CollectionMovie;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS,
  };
  use crate::models::HorizontallyScrollableText;

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_collection_details_scroll,
      CollectionDetailsHandler,
      collection_movies,
      simple_stateful_iterable_vec!(CollectionMovie, HorizontallyScrollableText),
      ActiveRadarrBlock::CollectionDetails,
      None,
      title,
      to_string
    );

    #[rstest]
    fn test_collection_details_scroll_no_op_when_not_ready(
      #[values(
			DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key
		)]
      key: Key,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(simple_stateful_iterable_vec!(
          CollectionMovie,
          HorizontallyScrollableText
        ));

      CollectionDetailsHandler::with(key, &mut app, ActiveRadarrBlock::CollectionDetails, None)
        .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collection_movies
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );

      CollectionDetailsHandler::with(key, &mut app, ActiveRadarrBlock::CollectionDetails, None)
        .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collection_movies
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }
  }

  mod test_handle_home_end {
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_collection_details_home_end,
      CollectionDetailsHandler,
      collection_movies,
      extended_stateful_iterable_vec!(CollectionMovie, HorizontallyScrollableText),
      ActiveRadarrBlock::CollectionDetails,
      None,
      title,
      to_string
    );

    #[test]
    fn test_collection_details_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(extended_stateful_iterable_vec!(
          CollectionMovie,
          HorizontallyScrollableText
        ));

      CollectionDetailsHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::CollectionDetails,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collection_movies
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );

      CollectionDetailsHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::CollectionDetails,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collection_movies
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }
  }

  mod test_handle_submit {
    use bimap::BiMap;
    use pretty_assertions::assert_eq;

    use crate::models::radarr_models::Movie;
    use crate::models::servarr_data::radarr::radarr_data::ADD_MOVIE_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_collection_details_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(vec![CollectionMovie::default()]);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(1, "B - Test 2".to_owned()), (0, "A - Test 1".to_owned())]);
      app.data.radarr_data.selected_block = BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, ADD_MOVIE_SELECTION_BLOCKS.len() - 1);

      CollectionDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::CollectionDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        (
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails)
        )
          .into()
      );
      assert!(!app
        .data
        .radarr_data
        .add_movie_modal
        .as_ref()
        .unwrap()
        .monitor_list
        .items
        .is_empty());
      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::AddMovieSelectRootFolder
      );
      assert!(!app
        .data
        .radarr_data
        .add_movie_modal
        .as_ref()
        .unwrap()
        .minimum_availability_list
        .items
        .is_empty());
      assert!(!app
        .data
        .radarr_data
        .add_movie_modal
        .as_ref()
        .unwrap()
        .quality_profile_list
        .items
        .is_empty());
      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "A - Test 1"
      );
    }

    #[test]
    fn test_collection_details_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(vec![CollectionMovie::default()]);

      CollectionDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::CollectionDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::CollectionDetails.into()
      );
      assert!(app.data.radarr_data.add_movie_modal.is_none());
    }

    #[test]
    fn test_collection_details_submit_movie_already_in_library() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(vec![CollectionMovie::default()]);
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      CollectionDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::CollectionDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::ViewMovieOverview.into()
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_esc_collection_details(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(vec![CollectionMovie::default()]);

      CollectionDetailsHandler::with(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::CollectionDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::Collections.into()
      );
      assert!(app.data.radarr_data.collection_movies.items.is_empty());
    }

    #[test]
    fn test_esc_view_movie_overview() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
      app.push_navigation_stack(ActiveRadarrBlock::ViewMovieOverview.into());

      CollectionDetailsHandler::with(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::CollectionDetails.into()
      );
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use strum::IntoEnumIterator;

    use crate::models::radarr_models::{Collection, MinimumAvailability};
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::servarr_data::radarr::radarr_data::{
      RadarrData, EDIT_COLLECTION_SELECTION_BLOCKS,
    };
    use crate::test_edit_collection_key;

    use super::*;

    #[test]
    fn test_edit_key() {
      test_edit_collection_key!(
        CollectionDetailsHandler,
        ActiveRadarrBlock::CollectionDetails,
        ActiveRadarrBlock::CollectionDetails
      );
    }

    #[test]
    fn test_edit_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
      let mut radarr_data = create_test_radarr_data();
      radarr_data.collections.set_items(vec![Collection {
        root_folder_path: "/nfs/movies/Test".to_owned().into(),
        monitored: true,
        search_on_add: true,
        quality_profile_id: 2222,
        minimum_availability: MinimumAvailability::Released,
        ..Collection::default()
      }]);
      app.data.radarr_data = radarr_data;

      CollectionDetailsHandler::with(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        ActiveRadarrBlock::CollectionDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::CollectionDetails.into()
      );
      assert!(app.data.radarr_data.edit_collection_modal.is_none());
    }
  }

  #[test]
  fn test_collection_details_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if COLLECTION_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(CollectionDetailsHandler::accepts(active_radarr_block));
      } else {
        assert!(!CollectionDetailsHandler::accepts(active_radarr_block));
      }
    });
  }

  #[test]
  fn test_collection_details_handler_not_ready_when_loading() {
    let mut app = App::default();
    app.is_loading = true;

    let handler = CollectionDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::CollectionDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_collection_details_handler_not_ready_when_collection_movies_is_empty() {
    let mut app = App::default();
    app.is_loading = false;

    let handler = CollectionDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::CollectionDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_collection_details_handler_ready_when_not_loading_and_collection_movies_is_not_empty() {
    let mut app = App::default();
    app.is_loading = false;
    app
      .data
      .radarr_data
      .collection_movies
      .set_items(vec![CollectionMovie::default()]);

    let handler = CollectionDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::CollectionDetails,
      None,
    );

    assert!(handler.is_ready());
  }
}
