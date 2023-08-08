#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::collections::CollectionsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::Collection;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, COLLECTIONS_BLOCKS, COLLECTION_DETAILS_BLOCKS, EDIT_COLLECTION_BLOCKS,
  };
  use crate::models::HorizontallyScrollableText;
  use crate::{extended_stateful_iterable_vec, test_handler_delegation};

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_collections_scroll,
      CollectionsHandler,
      collections,
      simple_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_scroll!(
      test_filtered_collections_scroll,
      CollectionsHandler,
      filtered_collections,
      simple_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_collections_home_end,
      CollectionsHandler,
      collections,
      extended_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_home_and_end!(
      test_filtered_collections_home_end,
      CollectionsHandler,
      filtered_collections,
      extended_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    #[test]
    fn test_collection_search_box_home_end_keys() {
      let mut app = App::default();
      app.data.radarr_data.search = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        4
      );

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }

    #[test]
    fn test_collection_filter_box_home_end_keys() {
      let mut app = App::default();
      app.data.radarr_data.filter = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .filter
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        4
      );

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .filter
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_collections_tab_left() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(2);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::Downloads.into()
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Downloads.into()
      );
    }

    #[test]
    fn test_collections_tab_right() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(2);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
    }

    #[rstest]
    fn test_left_right_update_all_collections_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();

      CollectionsHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      CollectionsHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_collection_search_box_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.search = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        1
      );

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }

    #[test]
    fn test_collection_filter_box_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.filter = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .filter
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        1
      );

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .filter
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_collections_submit() {
      let mut app = App::default();

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::CollectionDetails.into()
      );
    }

    #[test]
    fn test_search_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = Some("Test 2".into());

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collections
          .current_selection()
          .title
          .text,
        "Test 2"
      );
    }

    #[test]
    fn test_search_filtered_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = Some("Test 2".into());

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title
          .text,
        "Test 2"
      );
    }

    #[test]
    fn test_filter_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.filter = Some("Test".into());

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.filtered_collections.items.len(), 3);
      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title
          .text,
        "Test 1"
      );
    }

    #[test]
    fn test_update_all_collections_prompt_confirm_submit() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::UpdateCollections)
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }

    #[test]
    fn test_update_all_collections_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::{assert_filter_reset, assert_search_reset};

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_search_collection_block_esc() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchCollection.into());
      app.data.radarr_data = create_test_radarr_data();

      CollectionsHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_search_reset!(app.data.radarr_data);
    }

    #[test]
    fn test_filter_collections_block_esc() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::FilterCollections.into());
      app.data.radarr_data = create_test_radarr_data();

      CollectionsHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_filter_reset!(app.data.radarr_data);
    }

    #[test]
    fn test_update_all_collections_prompt_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      CollectionsHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_default_esc() {
      let mut app = App::default();
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.data.radarr_data = create_test_radarr_data();

      CollectionsHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::Collections, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(app.error.text.is_empty());
      assert_search_reset!(app.data.radarr_data);
      assert_filter_reset!(app.data.radarr_data);
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::models::radarr_models::MinimumAvailability;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::servarr_data::radarr::radarr_data::{
      RadarrData, EDIT_COLLECTION_SELECTION_BLOCKS,
    };

    use crate::models::StatefulTable;
    use crate::{assert_refresh_key, test_edit_collection_key};

    use super::*;

    #[test]
    fn test_search_collections_key() {
      let mut app = App::default();

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SearchCollection.into()
      );
      assert!(app.data.radarr_data.is_searching);
      assert!(app.should_ignore_quit_key);
      assert!(app.data.radarr_data.search.is_some());
    }

    #[test]
    fn test_filter_collections_key() {
      let mut app = App::default();

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::FilterCollections.into()
      );
      assert!(app.data.radarr_data.is_filtering);
      assert!(app.should_ignore_quit_key);
      assert!(app.data.radarr_data.filter.is_some());
    }

    #[test]
    fn test_collection_edit_key() {
      test_edit_collection_key!(
        CollectionsHandler,
        ActiveRadarrBlock::Collections,
        ActiveRadarrBlock::Collections
      );
    }

    #[test]
    fn test_update_key() {
      let mut app = App::default();

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::UpdateAllCollectionsPrompt.into()
      );
    }

    #[test]
    fn test_refresh_collections_key() {
      assert_refresh_key!(CollectionsHandler, ActiveRadarrBlock::Collections);
    }

    #[test]
    fn test_search_collections_box_backspace_key() {
      let mut app = App::default();
      app.data.radarr_data.search = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.as_ref().unwrap().text, "Tes");
    }

    #[test]
    fn test_filter_collections_box_backspace_key() {
      let mut app = App::default();
      app.data.radarr_data.filter = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.filter.as_ref().unwrap().text, "Tes");
    }

    #[test]
    fn test_search_collections_box_char_key() {
      let mut app = App::default();
      app.data.radarr_data.search = Some(HorizontallyScrollableText::default());

      CollectionsHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.as_ref().unwrap().text, "h");
    }

    #[test]
    fn test_filter_collections_box_char_key() {
      let mut app = App::default();
      app.data.radarr_data.filter = Some(HorizontallyScrollableText::default());

      CollectionsHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.filter.as_ref().unwrap().text, "h");
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
  fn test_collections_handler_accepts() {
    let mut collections_handler_blocks = Vec::new();
    collections_handler_blocks.extend(COLLECTIONS_BLOCKS);
    collections_handler_blocks.extend(COLLECTION_DETAILS_BLOCKS);
    collections_handler_blocks.extend(EDIT_COLLECTION_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if collections_handler_blocks.contains(&active_radarr_block) {
        assert!(CollectionsHandler::accepts(&active_radarr_block));
      } else {
        assert!(!CollectionsHandler::accepts(&active_radarr_block));
      }
    });
  }
}
