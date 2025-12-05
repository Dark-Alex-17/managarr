#[cfg(test)]
mod tests {
  use std::cmp::Ordering;
  use std::iter;

  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_modal_absent;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::radarr_handlers::collections::{
    CollectionsHandler, collections_sorting_options,
  };
  use crate::models::radarr_models::{Collection, CollectionMovie};
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS, COLLECTIONS_BLOCKS, EDIT_COLLECTION_BLOCKS,
  };
  use crate::test_handler_delegation;

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_pushed;

    #[rstest]
    fn test_collections_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(1);

      CollectionsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::Collections,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::Movies.into()
      );
      assert_navigation_pushed!(app, ActiveRadarrBlock::Movies.into());
    }

    #[rstest]
    fn test_collections_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(1);

      CollectionsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::Collections,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::Downloads.into()
      );
      assert_navigation_pushed!(app, ActiveRadarrBlock::Downloads.into());
    }

    #[rstest]
    fn test_left_right_update_all_collections_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();

      CollectionsHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      CollectionsHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use crate::assert_navigation_popped;
    use crate::network::radarr_network::RadarrEvent;
    use pretty_assertions::assert_eq;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_collections_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::Collections, None).handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::CollectionDetails.into());
    }

    #[test]
    fn test_collections_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::Collections, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::Collections.into()
      );
    }

    #[test]
    fn test_update_all_collections_prompt_confirm_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());

      CollectionsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::UpdateCollections)
      );
      assert_navigation_popped!(app, ActiveRadarrBlock::Collections.into());
    }

    #[test]
    fn test_update_all_collections_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());

      CollectionsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_navigation_popped!(app, ActiveRadarrBlock::Collections.into());
    }
  }

  mod test_handle_esc {
    use crate::assert_navigation_popped;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use pretty_assertions::assert_eq;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_update_all_collections_prompt_block_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      CollectionsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::Collections.into());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.data.radarr_data = create_test_radarr_data();

      CollectionsHandler::new(ESC_KEY, &mut app, ActiveRadarrBlock::Collections, None).handle();

      assert_navigation_popped!(app,
        ActiveRadarrBlock::Collections.into()
      );
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use strum::IntoEnumIterator;

    use crate::models::radarr_models::MinimumAvailability;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::servarr_data::radarr::radarr_data::{
      EDIT_COLLECTION_SELECTION_BLOCKS, RadarrData,
    };
    use crate::network::radarr_network::RadarrEvent;
    use crate::{assert_navigation_popped, test_edit_collection_key};

    use super::*;

    #[test]
    fn test_collection_edit_key() {
      test_edit_collection_key!(CollectionsHandler, ActiveRadarrBlock::Collections, None);
    }

    #[test]
    fn test_collection_edit_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
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

      CollectionsHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        ActiveRadarrBlock::Collections,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::Collections.into()
      );
      assert_modal_absent!(app.data.radarr_data.edit_collection_modal);
    }

    #[test]
    fn test_update_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveRadarrBlock::Collections,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());
    }

    #[test]
    fn test_update_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveRadarrBlock::Collections,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::Collections.into()
      );
    }

    #[test]
    fn test_refresh_collections_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::Collections,
        None,
      )
      .handle();

      assert_navigation_pushed!(app,
        ActiveRadarrBlock::Collections.into()
      );
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_collections_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::Collections,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::Collections.into()
      );
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_update_all_collections_prompt_confirm_confirm() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());

      CollectionsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::UpdateCollections)
      );
      assert_navigation_popped!(app, ActiveRadarrBlock::Collections.into());
    }
  }

  #[rstest]
  fn test_delegate_collection_details_blocks_to_collection_details_handler(
    #[values(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::ViewMovieOverview
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      CollectionsHandler,
      ActiveRadarrBlock::Collections,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegate_edit_collection_blocks_to_edit_collection_handler(
    #[values(
      ActiveRadarrBlock::EditCollectionPrompt,
      ActiveRadarrBlock::EditCollectionRootFolderPathInput,
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
      ActiveRadarrBlock::EditCollectionSelectQualityProfile
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      CollectionsHandler,
      ActiveRadarrBlock::Collections,
      active_radarr_block
    );
  }

  #[test]
  fn test_collections_sorting_options_collection() {
    let expected_cmp_fn: fn(&Collection, &Collection) -> Ordering = |a, b| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let mut expected_collections_vec = collections_vec();
    expected_collections_vec.sort_by(expected_cmp_fn);

    let sort_option = collections_sorting_options()[0].clone();
    let mut sorted_collections_vec = collections_vec();
    sorted_collections_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_collections_vec, expected_collections_vec);
    assert_str_eq!(sort_option.name, "Collection");
  }

  #[test]
  fn test_collections_sorting_options_number_of_movies() {
    let expected_cmp_fn: fn(&Collection, &Collection) -> Ordering = |a, b| {
      let a_movie_count = a.movies.as_ref().unwrap_or(&Vec::new()).len();
      let b_movie_count = b.movies.as_ref().unwrap_or(&Vec::new()).len();

      a_movie_count.cmp(&b_movie_count)
    };
    let mut expected_collections_vec = collections_vec();
    expected_collections_vec.sort_by(expected_cmp_fn);

    let sort_option = collections_sorting_options()[1].clone();
    let mut sorted_collections_vec = collections_vec();
    sorted_collections_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_collections_vec, expected_collections_vec);
    assert_str_eq!(sort_option.name, "Number of Movies");
  }

  #[test]
  fn test_collections_sorting_options_root_folder_path() {
    let expected_cmp_fn: fn(&Collection, &Collection) -> Ordering = |a, b| {
      let a_root_folder = a
        .root_folder_path
        .as_ref()
        .unwrap_or(&String::new())
        .to_owned();
      let b_root_folder = b
        .root_folder_path
        .as_ref()
        .unwrap_or(&String::new())
        .to_owned();

      a_root_folder.cmp(&b_root_folder)
    };
    let mut expected_collections_vec = collections_vec();
    expected_collections_vec.sort_by(expected_cmp_fn);

    let sort_option = collections_sorting_options()[2].clone();
    let mut sorted_collections_vec = collections_vec();
    sorted_collections_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_collections_vec, expected_collections_vec);
    assert_str_eq!(sort_option.name, "Root Folder Path");
  }

  #[test]
  fn test_collections_sorting_options_quality_profile() {
    let expected_cmp_fn: fn(&Collection, &Collection) -> Ordering =
      |a, b| a.quality_profile_id.cmp(&b.quality_profile_id);
    let mut expected_collections_vec = collections_vec();
    expected_collections_vec.sort_by(expected_cmp_fn);

    let sort_option = collections_sorting_options()[3].clone();
    let mut sorted_collections_vec = collections_vec();
    sorted_collections_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_collections_vec, expected_collections_vec);
    assert_str_eq!(sort_option.name, "Quality Profile");
  }

  #[test]
  fn test_collections_sorting_options_search_on_add() {
    let expected_cmp_fn: fn(&Collection, &Collection) -> Ordering =
      |a, b| a.search_on_add.cmp(&b.search_on_add);
    let mut expected_collections_vec = collections_vec();
    expected_collections_vec.sort_by(expected_cmp_fn);

    let sort_option = collections_sorting_options()[4].clone();
    let mut sorted_collections_vec = collections_vec();
    sorted_collections_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_collections_vec, expected_collections_vec);
    assert_str_eq!(sort_option.name, "Search on Add");
  }

  #[test]
  fn test_collections_sorting_options_monitored() {
    let expected_cmp_fn: fn(&Collection, &Collection) -> Ordering =
      |a, b| a.monitored.cmp(&b.monitored);
    let mut expected_collections_vec = collections_vec();
    expected_collections_vec.sort_by(expected_cmp_fn);

    let sort_option = collections_sorting_options()[5].clone();
    let mut sorted_collections_vec = collections_vec();
    sorted_collections_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_collections_vec, expected_collections_vec);
    assert_str_eq!(sort_option.name, "Monitored");
  }

  #[test]
  fn test_collections_handler_accepts() {
    let mut collections_handler_blocks = Vec::new();
    collections_handler_blocks.extend(COLLECTIONS_BLOCKS);
    collections_handler_blocks.extend(COLLECTION_DETAILS_BLOCKS);
    collections_handler_blocks.extend(EDIT_COLLECTION_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if collections_handler_blocks.contains(&active_radarr_block) {
        assert!(CollectionsHandler::accepts(active_radarr_block));
      } else {
        assert!(!CollectionsHandler::accepts(active_radarr_block));
      }
    });
  }

  #[rstest]
  fn test_collections_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = CollectionsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_collections_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = CollectionsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Collections,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_collections_handler_not_ready_when_collections_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = CollectionsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Collections,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_collections_handler_ready_when_not_loading_and_collections_is_not_empty() {
    let mut app = App::test_default();
    app.is_loading = false;
    app
      .data
      .radarr_data
      .collections
      .set_items(vec![Collection::default()]);

    let handler = CollectionsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Collections,
      None,
    );

    assert!(handler.is_ready());
  }

  fn collections_vec() -> Vec<Collection> {
    vec![
      Collection {
        id: 3,
        title: "test 1".into(),
        movies: Some(iter::repeat_n(CollectionMovie::default(), 3).collect()),
        root_folder_path: Some("/nfs/movies".into()),
        quality_profile_id: 1,
        search_on_add: false,
        monitored: true,
        ..Collection::default()
      },
      Collection {
        id: 2,
        title: "test 2".into(),
        movies: Some(iter::repeat_n(CollectionMovie::default(), 7).collect()),
        root_folder_path: Some("/htpc/movies".into()),
        quality_profile_id: 3,
        search_on_add: true,
        monitored: true,
        ..Collection::default()
      },
      Collection {
        id: 1,
        title: "test 3".into(),
        movies: Some(iter::repeat_n(CollectionMovie::default(), 1).collect()),
        root_folder_path: Some("/nfs/some/stupidly/long/path/to/test/with".into()),
        quality_profile_id: 1,
        search_on_add: false,
        monitored: false,
        ..Collection::default()
      },
    ]
  }
}
