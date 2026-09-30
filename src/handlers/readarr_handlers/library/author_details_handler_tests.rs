#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::Number;
  use std::cmp::Ordering;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::library::author_details_handler::{
    AuthorDetailsHandler, releases_sorting_options,
  };
  use crate::models::HorizontallyScrollableText;
  use crate::models::readarr_models::{ReadarrHistoryItem, ReadarrRelease};
  use crate::models::servarr_data::readarr::readarr_data::{
    AUTHOR_DETAILS_BLOCKS, AUTHOR_OVERVIEW_BLOCKS, ActiveReadarrBlock,
  };
  use crate::models::servarr_models::{Quality, QualityWrapper};
  use crate::test_handler_delegation;

  mod test_handle_delete {
    use super::*;
    use crate::assert_delete_prompt;
    use crate::event::Key;
    use crate::models::readarr_models::Book;
    use crate::models::servarr_data::readarr::readarr_data::DELETE_BOOK_SELECTION_BLOCKS;
    use pretty_assertions::assert_eq;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_book_delete() {
      let mut app = App::test_default();
      app.data.readarr_data.books.set_items(vec![Book::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      assert_delete_prompt!(
        AuthorDetailsHandler,
        app,
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::DeleteBookPrompt
      );
      assert_eq!(
        app.data.readarr_data.selected_block.blocks,
        DELETE_BOOK_SELECTION_BLOCKS
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use crate::app::App;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::assert_navigation_pushed;
    use crate::event::Key;
    use crate::handlers::KeyEventHandler;
    use crate::handlers::readarr_handlers::library::author_details_handler::AuthorDetailsHandler;
    use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
    use pretty_assertions::assert_eq;

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveReadarrBlock::UpdateAndScanAuthorPrompt,
        ActiveReadarrBlock::AutomaticallySearchAuthorPrompt,
        ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(active_readarr_block.into());

      AuthorDetailsHandler::new(key, &mut app, active_readarr_block, None).handle();

      assert!(app.data.readarr_data.prompt_confirm);

      AuthorDetailsHandler::new(key, &mut app, active_readarr_block, None).handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(ActiveReadarrBlock::AuthorDetails, ActiveReadarrBlock::AuthorHistory)]
    #[case(
      ActiveReadarrBlock::AuthorHistory,
      ActiveReadarrBlock::ManualAuthorSearch
    )]
    #[case(
      ActiveReadarrBlock::ManualAuthorSearch,
      ActiveReadarrBlock::AuthorDetails
    )]
    fn test_author_details_tabs_left_right_action(
      #[case] left_block: ActiveReadarrBlock,
      #[case] right_block: ActiveReadarrBlock,
      #[values(true, false)] is_loading: bool,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = is_loading;
      app.push_navigation_stack(right_block.into());
      app.data.readarr_data.author_info_tabs.index = app
        .data
        .readarr_data
        .author_info_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      AuthorDetailsHandler::new(DEFAULT_KEYBINDINGS.left.key, &mut app, right_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app.data.readarr_data.author_info_tabs.get_active_route()
      );
      assert_navigation_pushed!(app, left_block.into());

      AuthorDetailsHandler::new(DEFAULT_KEYBINDINGS.right.key, &mut app, left_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app.data.readarr_data.author_info_tabs.get_active_route()
      );
      assert_navigation_pushed!(app, right_block.into());
    }
  }

  mod test_handle_submit {
    use crate::app::App;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::event::Key;
    use crate::handlers::KeyEventHandler;
    use crate::handlers::readarr_handlers::library::author_details_handler::AuthorDetailsHandler;
    use crate::models::readarr_models::{Author, ReadarrHistoryItem};
    use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
    use crate::models::servarr_models::ReleaseDownloadBody;
    use crate::network::readarr_network::ReadarrEvent;
    use crate::network::readarr_network::readarr_network_test_utils::test_utils::torrent_release;
    use crate::{assert_navigation_popped, assert_navigation_pushed};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    #[case(
      ActiveReadarrBlock::AutomaticallySearchAuthorPrompt,
      ReadarrEvent::TriggerAutomaticAuthorSearch(1)
    )]
    #[case(
      ActiveReadarrBlock::UpdateAndScanAuthorPrompt,
      ReadarrEvent::UpdateAndScanAuthor(1)
    )]
    fn test_author_details_prompt_confirm_submit(
      #[case] prompt_block: ActiveReadarrBlock,
      #[case] expected_action: ReadarrEvent,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.authors.set_items(vec![Author {
        id: 1,
        ..Author::default()
      }]);
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(prompt_block.into());

      AuthorDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveReadarrBlock::AuthorDetails.into());
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[rstest]
    fn test_author_details_prompt_decline_submit(
      #[values(
        ActiveReadarrBlock::AutomaticallySearchAuthorPrompt,
        ActiveReadarrBlock::UpdateAndScanAuthorPrompt
      )]
      prompt_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(prompt_block.into());

      AuthorDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.readarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveReadarrBlock::AuthorDetails.into());
      assert_none!(app.data.readarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_author_history_submit() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .author_history
        .set_items(vec![ReadarrHistoryItem::default()]);

      AuthorDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AuthorHistory,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AuthorHistoryDetails.into());
    }

    #[test]
    fn test_author_history_submit_no_op_when_author_history_is_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorHistory.into());

      AuthorDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AuthorHistory,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AuthorHistory.into()
      );
    }

    #[test]
    fn test_author_history_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

      AuthorDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AuthorHistory,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
    }

    #[test]
    fn test_manual_author_search_submit() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .author_releases
        .set_items(vec![torrent_release()]);
      app.push_navigation_stack(ActiveReadarrBlock::ManualAuthorSearch.into());

      AuthorDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::ManualAuthorSearch,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt.into()
      );
    }

    #[test]
    fn test_manual_author_search_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::ManualAuthorSearch.into());

      AuthorDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::ManualAuthorSearch,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::ManualAuthorSearch.into()
      );
    }

    #[test]
    fn test_manual_author_search_confirm_prompt_confirm_submit() {
      let mut app = App::test_default();
      let release = torrent_release();
      let mut decoy_release = torrent_release();
      decoy_release.guid = "decoy-guid".to_owned();
      decoy_release.indexer_id = 999;
      app
        .data
        .readarr_data
        .author_releases
        .set_items(vec![decoy_release, release.clone()]);
      app.data.readarr_data.author_releases.select_index(Some(1));
      app.data.readarr_data.prompt_confirm = true;
      let expected_action = ReadarrEvent::DownloadRelease(ReleaseDownloadBody {
        guid: release.guid,
        indexer_id: release.indexer_id,
      });
      app.push_navigation_stack(ActiveReadarrBlock::ManualAuthorSearch.into());
      app.push_navigation_stack(ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt.into());

      AuthorDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveReadarrBlock::ManualAuthorSearch.into());
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[test]
    fn test_manual_author_search_confirm_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .author_releases
        .set_items(vec![torrent_release()]);
      app.push_navigation_stack(ActiveReadarrBlock::ManualAuthorSearch.into());
      app.push_navigation_stack(ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt.into());

      AuthorDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveReadarrBlock::ManualAuthorSearch.into());
      assert_none!(app.data.readarr_data.prompt_confirm_action);
    }
  }

  mod test_handle_esc {
    use crate::app::App;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::assert_navigation_popped;
    use crate::event::Key;
    use crate::handlers::KeyEventHandler;
    use crate::handlers::readarr_handlers::library::author_details_handler::AuthorDetailsHandler;
    use crate::models::readarr_models::ReadarrHistoryItem;
    use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_author_history_details_block_esc() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorHistory.into());
      app.push_navigation_stack(ActiveReadarrBlock::AuthorHistoryDetails.into());

      AuthorDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::AuthorHistoryDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AuthorHistory.into());
    }

    #[test]
    fn test_author_history_esc_resets_filter_if_one_is_set_instead_of_closing_the_window() {
      let mut app = App::test_default();
      app.data.readarr_data.author_history = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(vec![ReadarrHistoryItem::default()]),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AuthorHistory.into());

      AuthorDetailsHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::AuthorHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AuthorHistory.into()
      );
      assert_none!(app.data.readarr_data.author_history.filter);
      assert_none!(app.data.readarr_data.author_history.filtered_items);
      assert_none!(app.data.readarr_data.author_history.filtered_state);
    }

    #[rstest]
    fn test_author_details_esc(
      #[values(
        ActiveReadarrBlock::AutomaticallySearchAuthorPrompt,
        ActiveReadarrBlock::UpdateAndScanAuthorPrompt,
        ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt
      )]
      prompt_block: ActiveReadarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.readarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(prompt_block.into());

      AuthorDetailsHandler::new(ESC_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.readarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveReadarrBlock::AuthorDetails.into());
    }

    #[rstest]
    fn test_author_details_blocks_esc(
      #[values(
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::AuthorHistory,
        ActiveReadarrBlock::ManualAuthorSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.author_history.filter = None;
      app.data.readarr_data.author_history.filtered_items = None;
      app.data.readarr_data.author_history.filtered_state = None;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(active_readarr_block.into());

      AuthorDetailsHandler::new(ESC_KEY, &mut app, active_readarr_block, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_is_empty!(app.data.readarr_data.books);
      assert_is_empty!(app.data.readarr_data.author_releases);
      assert_is_empty!(app.data.readarr_data.author_history);
      assert_eq!(app.data.readarr_data.author_info_tabs.index, 0);
    }
  }

  mod test_handle_char_key_event {
    use crate::app::App;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::assert_navigation_pushed;
    use crate::handlers::KeyEventHandler;
    use crate::handlers::readarr_handlers::library::author_details_handler::AuthorDetailsHandler;
    use crate::handlers::readarr_handlers::library::author_overview_handler::AuthorOverviewHandler;
    use crate::models::readarr_models::Author;
    use crate::models::servarr_data::readarr::readarr_data::{
      ActiveReadarrBlock, EDIT_AUTHOR_SELECTION_BLOCKS,
    };
    use crate::models::servarr_models::ReleaseDownloadBody;
    use crate::network::readarr_network::ReadarrEvent;
    use crate::network::readarr_network::readarr_network_test_utils::test_utils::torrent_release;
    use crate::{assert_modal_absent, assert_modal_present, assert_navigation_popped};
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    #[rstest]
    fn test_author_details_edit_key(
      #[values(
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::AuthorHistory,
        ActiveReadarrBlock::ManualAuthorSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(active_readarr_block.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        (
          ActiveReadarrBlock::EditAuthorPrompt,
          Some(active_readarr_block)
        )
          .into()
      );
      assert_modal_present!(app.data.readarr_data.edit_author_modal);
      assert_eq!(
        app.data.readarr_data.selected_block.blocks,
        EDIT_AUTHOR_SELECTION_BLOCKS
      );
    }

    #[rstest]
    fn test_author_details_edit_key_no_op_when_not_ready(
      #[values(
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::AuthorHistory,
        ActiveReadarrBlock::ManualAuthorSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(active_readarr_block.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_readarr_block.into());
      assert_modal_absent!(app.data.readarr_data.edit_author_modal);
    }

    #[test]
    fn test_author_details_view_key_opens_the_author_overview() {
      let mut app = App::test_default_fully_populated();
      app.data.readarr_data.author_overview_modal = None;
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.view.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AuthorOverview.into());
      assert_modal_present!(app.data.readarr_data.author_overview_modal);
    }

    #[test]
    fn test_author_details_view_key_builds_the_overview_from_the_selected_author() {
      let mut app = App::test_default_fully_populated();
      app.data.readarr_data.author_overview_modal = None;
      let author = app.data.readarr_data.authors.current_selection().clone();
      let mut decoy_author = author.clone();
      decoy_author.id = 999;
      decoy_author.overview = Some("decoy author overview".to_owned());
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![decoy_author, author]);
      app.data.readarr_data.authors.select_index(Some(1));
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.view.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();

      let overview = &app
        .data
        .readarr_data
        .author_overview_modal
        .as_ref()
        .unwrap()
        .overview;
      assert_eq!(overview.offset, 0);
      assert_eq!(
        overview.get_text(),
        app
          .data
          .readarr_data
          .authors
          .current_selection()
          .overview
          .clone()
          .unwrap()
      );
    }

    #[test]
    fn test_author_details_view_key_for_an_author_with_no_overview() {
      let mut app = App::test_default_fully_populated();
      app.data.readarr_data.author_overview_modal = None;
      let mut author = app.data.readarr_data.authors.current_selection().clone();
      author.overview = None;
      app.data.readarr_data.authors.set_items(vec![author]);
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.view.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AuthorOverview.into());
      assert_str_eq!(
        app
          .data
          .readarr_data
          .author_overview_modal
          .as_ref()
          .unwrap()
          .overview
          .get_text(),
        ""
      );
    }

    #[test]
    fn test_author_details_view_key_reopens_the_author_overview_at_the_top() {
      let mut app = App::test_default_fully_populated();
      app.data.readarr_data.author_overview_modal = None;
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.view.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();
      AuthorOverviewHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AuthorOverview,
        None,
      )
      .handle();
      AuthorOverviewHandler::new(
        DEFAULT_KEYBINDINGS.esc.key,
        &mut app,
        ActiveReadarrBlock::AuthorOverview,
        None,
      )
      .handle();

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.view.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AuthorOverview.into());
      assert_eq!(
        app
          .data
          .readarr_data
          .author_overview_modal
          .as_ref()
          .unwrap()
          .overview
          .offset,
        0
      );
    }

    #[test]
    fn test_author_details_view_key_no_op_when_not_ready() {
      let mut app = App::test_default_fully_populated();
      app.data.readarr_data.author_overview_modal = None;
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.view.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AuthorDetails.into()
      );
      assert_modal_absent!(app.data.readarr_data.author_overview_modal);
    }

    #[test]
    fn test_author_details_toggle_monitoring_key() {
      let mut app = App::test_default_fully_populated();
      let book = app.data.readarr_data.books.current_selection().clone();
      let mut decoy_book = book.clone();
      decoy_book.id = 999;
      app
        .data
        .readarr_data
        .books
        .set_items(vec![decoy_book, book]);
      app.data.readarr_data.books.select_index(Some(1));
      app.is_routing = false;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AuthorDetails.into()
      );
      assert!(app.data.readarr_data.prompt_confirm);
      assert!(app.is_routing);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &ReadarrEvent::ToggleBookMonitoring(1)
      );
    }

    #[test]
    fn test_author_details_toggle_monitoring_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.prompt_confirm = false;
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AuthorDetails.into()
      );
      assert!(!app.data.readarr_data.prompt_confirm);
      assert_none!(app.data.readarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_author_details_toggle_monitoring_key_no_op_when_books_empty() {
      let mut app = App::test_default();
      app.data.readarr_data.authors.set_items(vec![Author {
        id: 1,
        ..Author::default()
      }]);
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
      assert_none!(app.data.readarr_data.prompt_confirm_action);
    }

    #[rstest]
    fn test_author_details_auto_search_key(
      #[values(
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::AuthorHistory,
        ActiveReadarrBlock::ManualAuthorSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveReadarrBlock::AutomaticallySearchAuthorPrompt.into()
      );
    }

    #[rstest]
    fn test_author_details_auto_search_key_no_op_when_not_ready(
      #[values(
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::AuthorHistory,
        ActiveReadarrBlock::ManualAuthorSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_readarr_block.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_readarr_block.into());
    }

    #[rstest]
    fn test_author_details_update_key(
      #[values(
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::AuthorHistory,
        ActiveReadarrBlock::ManualAuthorSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::UpdateAndScanAuthorPrompt.into());
    }

    #[rstest]
    fn test_author_details_update_key_no_op_when_not_ready(
      #[values(
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::AuthorHistory,
        ActiveReadarrBlock::ManualAuthorSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_readarr_block.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_readarr_block.into());
    }

    #[rstest]
    fn test_author_details_refresh_key(
      #[values(
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::AuthorHistory,
        ActiveReadarrBlock::ManualAuthorSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_routing = false;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(active_readarr_block.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, active_readarr_block.into());
      assert!(app.is_routing);
    }

    #[rstest]
    fn test_author_details_refresh_key_no_op_when_not_ready(
      #[values(
        ActiveReadarrBlock::AuthorDetails,
        ActiveReadarrBlock::AuthorHistory,
        ActiveReadarrBlock::ManualAuthorSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(active_readarr_block.into());
      app.is_routing = false;

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_readarr_block.into());
      assert!(!app.is_routing);
    }

    #[rstest]
    #[case(
      ActiveReadarrBlock::AutomaticallySearchAuthorPrompt,
      ReadarrEvent::TriggerAutomaticAuthorSearch(1)
    )]
    #[case(
      ActiveReadarrBlock::UpdateAndScanAuthorPrompt,
      ReadarrEvent::UpdateAndScanAuthor(1)
    )]
    fn test_author_details_prompt_confirm_key(
      #[case] prompt_block: ActiveReadarrBlock,
      #[case] expected_action: ReadarrEvent,
      #[values(ActiveReadarrBlock::AuthorDetails, ActiveReadarrBlock::AuthorHistory)]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_navigation_popped!(app, active_readarr_block.into());
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[test]
    fn test_manual_author_search_confirm_prompt_confirm_key() {
      let mut app = App::test_default();
      let release = torrent_release();
      let mut decoy_release = torrent_release();
      decoy_release.guid = "decoy-guid".to_owned();
      decoy_release.indexer_id = 999;
      app
        .data
        .readarr_data
        .author_releases
        .set_items(vec![decoy_release, release.clone()]);
      app.data.readarr_data.author_releases.select_index(Some(1));
      let expected_action = ReadarrEvent::DownloadRelease(ReleaseDownloadBody {
        guid: release.guid,
        indexer_id: release.indexer_id,
      });
      app.push_navigation_stack(ActiveReadarrBlock::ManualAuthorSearch.into());
      app.push_navigation_stack(ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveReadarrBlock::ManualAuthorSearch.into());
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[test]
    fn test_search_books_key() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveReadarrBlock::AuthorDetails,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::SearchBooks.into());
    }

    #[test]
    fn test_search_author_history_key() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorHistory.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveReadarrBlock::AuthorHistory,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::SearchAuthorHistory.into());
    }

    #[test]
    fn test_filter_author_history_key() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorHistory.into());

      AuthorDetailsHandler::new(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveReadarrBlock::AuthorHistory,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::FilterAuthorHistory.into());
    }
  }

  #[test]
  fn test_author_details_handler_accepts() {
    let mut author_details_blocks = AUTHOR_DETAILS_BLOCKS.to_vec();
    author_details_blocks.extend(AUTHOR_OVERVIEW_BLOCKS);

    ActiveReadarrBlock::iter().for_each(|readarr_block| {
      if author_details_blocks.contains(&readarr_block) {
        assert!(
          AuthorDetailsHandler::accepts(readarr_block),
          "{readarr_block} is not accepted by the AuthorDetailsHandler"
        );
      } else {
        assert!(!AuthorDetailsHandler::accepts(readarr_block));
      }
    });
  }

  #[test]
  fn test_delegates_author_overview_blocks_to_author_overview_handler() {
    test_handler_delegation!(
      AuthorDetailsHandler,
      ActiveReadarrBlock::AuthorDetails,
      ActiveReadarrBlock::AuthorOverview
    );
  }

  #[test]
  fn test_extract_author_id() {
    let mut app = App::test_default_fully_populated();
    let author = app.data.readarr_data.authors.current_selection().clone();
    let mut decoy_author = author.clone();
    decoy_author.id = 999;
    app
      .data
      .readarr_data
      .authors
      .set_items(vec![decoy_author, author]);
    app.data.readarr_data.authors.select_index(Some(1));

    let author_id = AuthorDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AuthorDetails,
      None,
    )
    .extract_author_id();

    assert_eq!(author_id, 1);
  }

  #[test]
  fn test_extract_book_id() {
    let mut app = App::test_default_fully_populated();
    let book = app.data.readarr_data.books.current_selection().clone();
    let mut decoy_book = book.clone();
    decoy_book.id = 999;
    app
      .data
      .readarr_data
      .books
      .set_items(vec![decoy_book, book]);
    app.data.readarr_data.books.select_index(Some(1));

    let book_id = AuthorDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AuthorDetails,
      None,
    )
    .extract_book_id();

    assert_eq!(book_id, 1);
  }

  #[rstest]
  fn test_author_details_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;

    let handler = AuthorDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AuthorDetails,
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_author_details_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
    app.is_loading = true;

    let handler = AuthorDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AuthorDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_author_details_handler_is_ready_when_not_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
    app.is_loading = false;

    let handler = AuthorDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AuthorDetails,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_author_details_handler_is_not_ready_when_not_loading_and_author_history_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

    let handler = AuthorDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AuthorHistory,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_author_details_handler_ready_when_not_loading_and_author_history_is_non_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
    app
      .data
      .readarr_data
      .author_history
      .set_items(vec![ReadarrHistoryItem::default()]);

    let handler = AuthorDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AuthorHistory,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_author_details_handler_is_not_ready_when_not_loading_and_author_releases_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

    let handler = AuthorDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::ManualAuthorSearch,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_author_details_handler_ready_when_not_loading_and_author_releases_is_non_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
    app
      .data
      .readarr_data
      .author_releases
      .set_items(vec![ReadarrRelease::default()]);

    let handler = AuthorDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::ManualAuthorSearch,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_releases_sorting_options_source() {
    let expected_cmp_fn: fn(&ReadarrRelease, &ReadarrRelease) -> Ordering =
      |a, b| a.protocol.cmp(&b.protocol);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[0].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Source");
  }

  #[test]
  fn test_releases_sorting_options_age() {
    let expected_cmp_fn: fn(&ReadarrRelease, &ReadarrRelease) -> Ordering =
      |a, b| a.age.cmp(&b.age);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[1].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Age");
  }

  #[test]
  fn test_releases_sorting_options_rejected() {
    let expected_cmp_fn: fn(&ReadarrRelease, &ReadarrRelease) -> Ordering =
      |a, b| a.rejected.cmp(&b.rejected);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[2].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Rejected");
  }

  #[test]
  fn test_releases_sorting_options_title() {
    let expected_cmp_fn: fn(&ReadarrRelease, &ReadarrRelease) -> Ordering = |a, b| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[3].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Title");
  }

  #[test]
  fn test_releases_sorting_options_indexer() {
    let expected_cmp_fn: fn(&ReadarrRelease, &ReadarrRelease) -> Ordering =
      |a, b| a.indexer.to_lowercase().cmp(&b.indexer.to_lowercase());
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[4].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Indexer");
  }

  #[test]
  fn test_releases_sorting_options_size() {
    let expected_cmp_fn: fn(&ReadarrRelease, &ReadarrRelease) -> Ordering =
      |a, b| a.size.cmp(&b.size);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[5].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Size");
  }

  #[test]
  fn test_releases_sorting_options_peers() {
    let expected_cmp_fn: fn(&ReadarrRelease, &ReadarrRelease) -> Ordering = |a, b| {
      let default_number = Number::from(i64::MAX);
      let seeder_a = a
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();
      let seeder_b = b
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();

      seeder_a.cmp(&seeder_b)
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[6].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Peers");
  }

  #[test]
  fn test_releases_sorting_options_quality() {
    let expected_cmp_fn: fn(&ReadarrRelease, &ReadarrRelease) -> Ordering =
      |a, b| a.quality.cmp(&b.quality);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[7].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  fn release_vec() -> Vec<ReadarrRelease> {
    let release_a = ReadarrRelease {
      protocol: "Protocol A".to_owned(),
      age: 1,
      title: HorizontallyScrollableText::from("Title A"),
      indexer: "Indexer A".to_owned(),
      size: 1,
      rejected: true,
      seeders: Some(Number::from(1)),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality A".to_owned(),
        },
      },
      ..ReadarrRelease::default()
    };
    let release_b = ReadarrRelease {
      protocol: "Protocol B".to_owned(),
      age: 2,
      title: HorizontallyScrollableText::from("title B"),
      indexer: "indexer B".to_owned(),
      size: 2,
      rejected: false,
      seeders: Some(Number::from(2)),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality B".to_owned(),
        },
      },
      ..ReadarrRelease::default()
    };
    let release_c = ReadarrRelease {
      protocol: "Protocol C".to_owned(),
      age: 3,
      title: HorizontallyScrollableText::from("Title C"),
      indexer: "Indexer C".to_owned(),
      size: 3,
      rejected: false,
      seeders: None,
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality C".to_owned(),
        },
      },
      ..ReadarrRelease::default()
    };

    vec![release_a, release_b, release_c]
  }
}
