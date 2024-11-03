#[cfg(test)]
mod tests {
  use core::sync::atomic::Ordering::SeqCst;
  use std::cmp::Ordering;
  use std::iter;

  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::collections::{
    collections_sorting_options, CollectionsHandler,
  };
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{Collection, CollectionMovie};
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, COLLECTIONS_BLOCKS, COLLECTION_DETAILS_BLOCKS, EDIT_COLLECTION_BLOCKS,
  };
  use crate::models::stateful_table::SortOption;
  use crate::models::HorizontallyScrollableText;
  use crate::{extended_stateful_iterable_vec, test_handler_delegation};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
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

    #[rstest]
    fn test_collections_scroll_no_op_when_not_ready(
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
        .collections
        .set_items(simple_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));

      CollectionsHandler::with(&key, &mut app, &ActiveRadarrBlock::Collections, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collections
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );

      CollectionsHandler::with(&key, &mut app, &ActiveRadarrBlock::Collections, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collections
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_collections_sort_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let collection_field_vec = sort_options();
      let mut app = App::default();
      app.data.radarr_data.collections.sorting(sort_options());

      if key == Key::Up {
        for i in (0..collection_field_vec.len()).rev() {
          CollectionsHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::CollectionsSortPrompt,
            &None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .collections
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &collection_field_vec[i]
          );
        }
      } else {
        for i in 0..collection_field_vec.len() {
          CollectionsHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::CollectionsSortPrompt,
            &None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .collections
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &collection_field_vec[(i + 1) % collection_field_vec.len()]
          );
        }
      }
    }
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

    #[test]
    fn test_collections_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
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
          .to_string(),
        "Test 1"
      );

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
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
          .to_string(),
        "Test 1"
      );
    }

    #[test]
    fn test_collection_search_box_home_end_keys() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.data.radarr_data.collections.search = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .collections
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
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
        app
          .data
          .radarr_data
          .collections
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_collection_filter_box_home_end_keys() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.data.radarr_data.collections.filter = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .collections
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
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
        app
          .data
          .radarr_data
          .collections
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_collections_sort_home_end() {
      let collection_field_vec = sort_options();
      let mut app = App::default();
      app.data.radarr_data.collections.sorting(sort_options());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::CollectionsSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .collections
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &collection_field_vec[collection_field_vec.len() - 1]
      );

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::CollectionsSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .collections
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &collection_field_vec[0]
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_collections_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(1);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::Movies.into()
      );
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }

    #[rstest]
    fn test_collections_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(1);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
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
      app.data.radarr_data.collections.search = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .collections
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
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
        app
          .data
          .radarr_data
          .collections
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_collection_filter_box_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.collections.filter = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .collections
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
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
        app
          .data
          .radarr_data
          .collections
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
    use pretty_assertions::assert_eq;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

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
    fn test_collections_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }

    #[test]
    fn test_search_collections_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchCollection.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.collections.search = Some("Test 2".into());

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
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }

    #[test]
    fn test_search_collections_submit_error_on_no_search_hits() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchCollection.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.collections.search = Some("Test 5".into());

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
        "Test 1"
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SearchCollectionError.into()
      );
    }

    #[test]
    fn test_search_filtered_collections_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchCollection.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.collections.search = Some("Test 2".into());

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
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }

    #[test]
    fn test_filter_collections_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::FilterCollections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.collections.filter = Some("Test".into());

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(app.data.radarr_data.collections.filtered_items.is_some());
      assert_eq!(
        app
          .data
          .radarr_data
          .collections
          .filtered_items
          .as_ref()
          .unwrap()
          .len(),
        3
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .collections
          .current_selection()
          .title
          .text,
        "Test 1"
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }

    #[test]
    fn test_filter_collections_submit_error_on_no_filter_matches() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::FilterCollections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.collections.filter = Some("Test 5".into());

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(app.data.radarr_data.collections.filtered_items.is_none());
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::FilterCollectionsError.into()
      );
    }

    #[test]
    fn test_update_all_collections_prompt_confirm_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
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
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
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

    #[test]
    fn test_collections_sort_prompt_submit() {
      let mut app = App::default();
      app.data.radarr_data.collections.sort_asc = true;
      app.data.radarr_data.collections.sorting(sort_options());
      app
        .data
        .radarr_data
        .collections
        .set_items(collections_vec());
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::CollectionsSortPrompt.into());

      let mut expected_vec = collections_vec();
      expected_vec.sort_by(|a, b| a.id.cmp(&b.id));
      expected_vec.reverse();

      CollectionsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::CollectionsSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert_eq!(app.data.radarr_data.collections.items, expected_vec);
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
    fn test_search_collection_block_esc(
      #[values(
        ActiveRadarrBlock::SearchCollection,
        ActiveRadarrBlock::SearchCollectionError
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.collections.search = Some("Test".into());

      CollectionsHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.radarr_data.collections.search, None);
    }

    #[rstest]
    fn test_filter_collections_block_esc(
      #[values(
        ActiveRadarrBlock::FilterCollections,
        ActiveRadarrBlock::FilterCollectionsError
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.collections = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };

      CollectionsHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.radarr_data.collections.filter, None);
      assert_eq!(app.data.radarr_data.collections.filtered_items, None);
      assert_eq!(app.data.radarr_data.collections.filtered_state, None);
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
    fn test_collections_sort_prompt_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::CollectionsSortPrompt.into());

      CollectionsHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::CollectionsSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.collections = StatefulTable {
        search: Some("Test".into()),
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };

      CollectionsHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::Collections, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(app.error.text.is_empty());
      assert_eq!(app.data.radarr_data.collections.search, None);
      assert_eq!(app.data.radarr_data.collections.filter, None);
      assert_eq!(app.data.radarr_data.collections.filtered_items, None);
      assert_eq!(app.data.radarr_data.collections.filtered_state, None);
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use strum::IntoEnumIterator;

    use crate::models::radarr_models::MinimumAvailability;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::servarr_data::radarr::radarr_data::{
      RadarrData, EDIT_COLLECTION_SELECTION_BLOCKS,
    };
    use crate::network::radarr_network::RadarrEvent;
    use crate::test_edit_collection_key;

    use super::*;

    #[test]
    fn test_search_collections_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

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
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.radarr_data.collections.search,
        Some(HorizontallyScrollableText::default())
      );
    }

    #[test]
    fn test_search_collections_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.radarr_data.collections.search, None);
    }

    #[test]
    fn test_filter_collections_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

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
      assert!(app.should_ignore_quit_key);
      assert!(app.data.radarr_data.collections.filter.is_some());
    }

    #[test]
    fn test_filter_collections_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert!(app.data.radarr_data.collections.filter.is_none());
    }

    #[test]
    fn test_filter_collections_key_resets_previous_filter() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.data.radarr_data.collections.filter = Some("Test".into());

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
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.radarr_data.collections.filter,
        Some(HorizontallyScrollableText::default())
      );
      assert!(app.data.radarr_data.collections.filtered_items.is_none());
      assert!(app.data.radarr_data.collections.filtered_state.is_none());
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
    fn test_collection_edit_key_no_op_when_not_ready() {
      let mut app = App::default();
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

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(app.data.radarr_data.edit_collection_modal.is_none());
    }

    #[test]
    fn test_update_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

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
    fn test_update_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }

    #[test]
    fn test_refresh_collections_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_collections_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_search_collections_box_backspace_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.data.radarr_data.collections.search = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
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
          .search
          .as_ref()
          .unwrap()
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_filter_collections_box_backspace_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.data.radarr_data.collections.filter = Some("Test".into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collections
          .filter
          .as_ref()
          .unwrap()
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_search_collections_box_char_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.data.radarr_data.collections.search = Some(HorizontallyScrollableText::default());

      CollectionsHandler::with(
        &Key::Char('h'),
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
          .search
          .as_ref()
          .unwrap()
          .text,
        "h"
      );
    }

    #[test]
    fn test_filter_collections_box_char_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.data.radarr_data.collections.filter = Some(HorizontallyScrollableText::default());

      CollectionsHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collections
          .filter
          .as_ref()
          .unwrap()
          .text,
        "h"
      );
    }

    #[test]
    fn test_sort_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::CollectionsSortPrompt.into()
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .collections
          .sort
          .as_ref()
          .unwrap()
          .items,
        collections_sorting_options()
      );
      assert!(!app.data.radarr_data.collections.sort_asc);
    }

    #[test]
    fn test_sort_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        &ActiveRadarrBlock::Collections,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(app.data.radarr_data.collections.sort.is_none());
      assert!(!app.data.radarr_data.collections.sort_asc);
    }

    #[test]
    fn test_update_all_collections_prompt_confirm_confirm() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(vec![Collection::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());

      CollectionsHandler::with(
        &DEFAULT_KEYBINDINGS.confirm.key,
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
        assert!(CollectionsHandler::accepts(&active_radarr_block));
      } else {
        assert!(!CollectionsHandler::accepts(&active_radarr_block));
      }
    });
  }

  #[test]
  fn test_collections_handler_not_ready_when_loading() {
    let mut app = App::default();
    app.is_loading = true;

    let handler = CollectionsHandler::with(
      &DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      &ActiveRadarrBlock::Collections,
      &None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_collections_handler_not_ready_when_collections_is_empty() {
    let mut app = App::default();
    app.is_loading = false;

    let handler = CollectionsHandler::with(
      &DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      &ActiveRadarrBlock::Collections,
      &None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_collections_handler_ready_when_not_loading_and_collections_is_not_empty() {
    let mut app = App::default();
    app.is_loading = false;
    app
      .data
      .radarr_data
      .collections
      .set_items(vec![Collection::default()]);

    let handler = CollectionsHandler::with(
      &DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      &ActiveRadarrBlock::Collections,
      &None,
    );

    assert!(handler.is_ready());
  }

  fn collections_vec() -> Vec<Collection> {
    vec![
      Collection {
        id: 3,
        title: "test 1".into(),
        movies: Some(iter::repeat(CollectionMovie::default()).take(3).collect()),
        root_folder_path: Some("/nfs/movies".into()),
        quality_profile_id: 1,
        search_on_add: false,
        monitored: true,
        ..Collection::default()
      },
      Collection {
        id: 2,
        title: "test 2".into(),
        movies: Some(iter::repeat(CollectionMovie::default()).take(7).collect()),
        root_folder_path: Some("/htpc/movies".into()),
        quality_profile_id: 3,
        search_on_add: true,
        monitored: true,
        ..Collection::default()
      },
      Collection {
        id: 1,
        title: "test 3".into(),
        movies: Some(iter::repeat(CollectionMovie::default()).take(1).collect()),
        root_folder_path: Some("/nfs/some/stupidly/long/path/to/test/with".into()),
        quality_profile_id: 1,
        search_on_add: false,
        monitored: false,
        ..Collection::default()
      },
    ]
  }

  fn sort_options() -> Vec<SortOption<Collection>> {
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
