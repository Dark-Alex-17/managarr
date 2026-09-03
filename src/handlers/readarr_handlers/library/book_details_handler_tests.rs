#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::library::book_details_handler::BookDetailsHandler;
  use crate::models::HorizontallyScrollableText;
  use crate::models::readarr_models::{
    Book, BookFile, Edition, Ratings, ReadarrHistoryItem, ReadarrRelease,
  };
  use crate::models::servarr_data::readarr::modals::BookDetailsModal;
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, BOOK_DETAILS_BLOCKS, EDITION_DETAILS_BLOCKS,
  };
  use crate::models::servarr_models::{Quality, QualityWrapper};
  use crate::test_handler_delegation;

  mod test_handle_scroll_up_and_down {
    use super::*;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::event::Key;
    use pretty_assertions::assert_str_eq;
    use rstest::rstest;

    #[rstest]
    fn test_editions_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(key, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_str_eq!(selected_edition(&app).title, "Test Edition 2");
    }

    #[rstest]
    fn test_editions_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(key, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_str_eq!(selected_edition(&app).title, "Test Edition 1");
    }

    #[rstest]
    fn test_book_history_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookHistory.into());

      BookDetailsHandler::new(key, &mut app, ActiveReadarrBlock::BookHistory, None).handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .unwrap()
          .book_history
          .current_selection()
          .source_title
          .text,
        "Test History 2"
      );
    }

    #[rstest]
    fn test_book_releases_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::ManualBookSearch.into());

      BookDetailsHandler::new(key, &mut app, ActiveReadarrBlock::ManualBookSearch, None).handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .unwrap()
          .book_releases
          .current_selection()
          .title
          .text,
        "Test Release 2"
      );
    }
  }

  mod test_handle_home_end {
    use super::*;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_editions_home_end() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::BookDetails,
        None,
      )
      .handle();

      assert_str_eq!(selected_edition(&app).title, "Test Edition 2");

      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::BookDetails,
        None,
      )
      .handle();

      assert_str_eq!(selected_edition(&app).title, "Test Edition 1");
    }

    #[test]
    fn test_editions_home_end_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::BookDetails,
        None,
      )
      .handle();

      assert_str_eq!(selected_edition(&app).title, "Test Edition 1");
    }
  }

  mod test_handle_delete {
    use super::*;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::assert_navigation_pushed;
    use crate::event::Key;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_book_file_prompt() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::DeleteBookFilePrompt.into());
    }

    #[test]
    fn test_delete_book_file_prompt_is_a_no_op_when_book_files_are_empty() {
      let mut app = App::test_default();
      let mut modal = book_details_modal();
      modal.book_files.set_items(Vec::new());
      app.data.readarr_data.book_details_modal = Some(modal);
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
    }

    #[test]
    fn test_delete_book_file_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
    }
  }

  mod test_handle_left_right_action {
    use super::*;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::assert_navigation_pushed;
    use crate::event::Key;
    use rstest::rstest;

    #[rstest]
    #[case(ActiveReadarrBlock::BookDetails, ActiveReadarrBlock::BookHistory)]
    #[case(ActiveReadarrBlock::BookHistory, ActiveReadarrBlock::BookFileInfo)]
    #[case(ActiveReadarrBlock::BookFileInfo, ActiveReadarrBlock::ManualBookSearch)]
    #[case(ActiveReadarrBlock::ManualBookSearch, ActiveReadarrBlock::BookDetails)]
    fn test_book_details_tabs_left_right_action(
      #[case] left_block: ActiveReadarrBlock,
      #[case] right_block: ActiveReadarrBlock,
      #[values(true, false)] is_loading: bool,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = is_loading;
      app.push_navigation_stack(right_block.into());
      let tab_index = app
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .unwrap()
        .book_details_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();
      app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .unwrap()
        .book_details_tabs
        .index = tab_index;

      BookDetailsHandler::new(DEFAULT_KEYBINDINGS.left.key, &mut app, right_block, None).handle();

      assert_navigation_pushed!(app, left_block.into());

      BookDetailsHandler::new(DEFAULT_KEYBINDINGS.right.key, &mut app, left_block, None).handle();

      assert_navigation_pushed!(app, right_block.into());
    }

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveReadarrBlock::AutomaticallySearchBookPrompt,
        ActiveReadarrBlock::ManualBookSearchConfirmPrompt,
        ActiveReadarrBlock::DeleteBookFilePrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(active_readarr_block.into());

      BookDetailsHandler::new(key, &mut app, active_readarr_block, None).handle();

      assert!(app.data.readarr_data.prompt_confirm);

      BookDetailsHandler::new(key, &mut app, active_readarr_block, None).handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use super::*;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::event::Key;
    use crate::models::servarr_models::ReleaseDownloadBody;
    use crate::network::readarr_network::ReadarrEvent;
    use crate::{assert_modal_absent, assert_modal_present, assert_navigation_pushed};
    use pretty_assertions::assert_eq;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_book_details_submit_builds_the_edition_details_modal_and_pushes_the_popup() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookEditionDetails.into());
      assert_modal_present!(
        app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .unwrap()
          .edition_details_modal
      );
      assert!(edition_details_text(&app).contains("Title: Test Edition 1"));
      assert!(edition_details_text(&app).contains("Format: Paperback"));
      assert!(edition_details_text(&app).contains("ISBN13: isbn-1"));
      assert!(edition_details_text(&app).contains("ASIN: asin-1"));
      assert!(edition_details_text(&app).contains("Overview: Overview 1"));
    }

    #[test]
    fn test_book_details_submit_dispatches_no_network_event() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_none!(app.data.readarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_book_details_submit_rebuilds_the_modal_when_a_different_edition_is_selected() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert!(edition_details_text(&app).contains("Title: Test Edition 1"));
      assert!(edition_details_text(&app).contains("Format: Paperback"));

      app.pop_navigation_stack();
      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveReadarrBlock::BookDetails,
        None,
      )
      .handle();

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert!(edition_details_text(&app).contains("Title: Test Edition 2"));
      assert!(edition_details_text(&app).contains("Format: Hardcover"));
      assert!(edition_details_text(&app).contains("ISBN13: isbn-2"));
      assert!(!edition_details_text(&app).contains("Test Edition 1"));
    }

    #[test]
    fn test_book_details_submit_is_a_no_op_when_the_editions_table_is_empty() {
      let mut app = App::test_default();
      let mut modal = book_details_modal();
      modal.editions.set_items(Vec::new());
      app.data.readarr_data.book_details_modal = Some(modal);
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
      assert_modal_absent!(
        app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .unwrap()
          .edition_details_modal
      );
    }

    #[test]
    fn test_book_details_submit_is_a_no_op_when_the_book_details_modal_is_absent() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
      assert_modal_absent!(app.data.readarr_data.book_details_modal);
    }

    #[test]
    fn test_book_details_submit_with_a_single_edition() {
      let mut app = App::test_default();
      let mut modal = book_details_modal();
      modal.editions.set_items(vec![editions_vec()[0].clone()]);
      app.data.readarr_data.book_details_modal = Some(modal);
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookEditionDetails.into());
      assert!(edition_details_text(&app).contains("Title: Test Edition 1"));
    }

    #[test]
    fn test_book_details_submit_renders_every_optional_field_when_absent() {
      let mut app = App::test_default();
      let mut modal = book_details_modal();
      modal.editions.set_items(vec![Edition {
        title: "Bare Edition".to_owned(),
        ..Edition::default()
      }]);
      app.data.readarr_data.book_details_modal = Some(modal);
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookEditionDetails.into());
      assert!(edition_details_text(&app).contains("Title: Bare Edition"));
      assert!(edition_details_text(&app).contains("Format: \n"));
      assert!(edition_details_text(&app).contains("Ratings: \n"));
    }

    #[test]
    fn test_book_details_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookDetails, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
      assert_modal_absent!(
        app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .unwrap()
          .edition_details_modal
      );
    }

    #[test]
    fn test_book_history_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookHistory.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookHistory, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookHistoryDetails.into());
    }

    #[test]
    fn test_book_history_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookHistory.into());

      BookDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::BookHistory, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookHistory.into());
    }

    #[test]
    fn test_manual_book_search_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::ManualBookSearch.into());

      BookDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::ManualBookSearch,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveReadarrBlock::ManualBookSearchConfirmPrompt.into()
      );
    }

    #[test]
    fn test_manual_book_search_confirm_prompt_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::ManualBookSearch.into());
      app.push_navigation_stack(ActiveReadarrBlock::ManualBookSearchConfirmPrompt.into());

      BookDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::ManualBookSearchConfirmPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::DownloadRelease(ReleaseDownloadBody {
          guid: "guid-1".to_owned(),
          indexer_id: 1,
        }))
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::ManualBookSearch.into());
    }

    #[test]
    fn test_manual_book_search_confirm_prompt_submit_no_confirmation() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::ManualBookSearch.into());
      app.push_navigation_stack(ActiveReadarrBlock::ManualBookSearchConfirmPrompt.into());

      BookDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::ManualBookSearchConfirmPrompt,
        None,
      )
      .handle();

      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert_navigation_pushed!(app, ActiveReadarrBlock::ManualBookSearch.into());
    }

    #[test]
    fn test_automatically_search_book_prompt_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.books.set_items(books_vec());
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::AutomaticallySearchBookPrompt.into());

      BookDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AutomaticallySearchBookPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::TriggerAutomaticBookSearch(7))
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
    }

    #[test]
    fn test_automatically_search_book_prompt_submit_no_confirmation() {
      let mut app = App::test_default();
      app.data.readarr_data.books.set_items(books_vec());
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::AutomaticallySearchBookPrompt.into());

      BookDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AutomaticallySearchBookPrompt,
        None,
      )
      .handle();

      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
    }

    #[test]
    fn test_delete_book_file_prompt_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteBookFilePrompt.into());

      BookDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteBookFilePrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::DeleteBookFile(11))
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
    }

    #[test]
    fn test_delete_book_file_prompt_submit_no_confirmation() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteBookFilePrompt.into());

      BookDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteBookFilePrompt,
        None,
      )
      .handle();

      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::event::Key;
    use crate::{assert_modal_absent, assert_modal_present, assert_navigation_popped};
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_book_details_tabs_esc_tears_down_the_book_details_modal(
      #[values(
        ActiveReadarrBlock::BookDetails,
        ActiveReadarrBlock::BookFileInfo,
        ActiveReadarrBlock::ManualBookSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(active_readarr_block.into());

      BookDetailsHandler::new(ESC_KEY, &mut app, active_readarr_block, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AuthorDetails.into());
      assert_modal_absent!(app.data.readarr_data.book_details_modal);
    }

    #[test]
    fn test_book_history_esc_tears_down_the_book_details_modal() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::BookHistory.into());

      BookDetailsHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::BookHistory, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AuthorDetails.into());
      assert_modal_absent!(app.data.readarr_data.book_details_modal);
    }

    #[test]
    fn test_book_history_esc_resets_the_filter_first() {
      let mut app = App::test_default();
      let mut modal = book_details_modal();
      modal.book_history.filtered_items = Some(history_vec());
      app.data.readarr_data.book_details_modal = Some(modal);
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::BookHistory.into());

      BookDetailsHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::BookHistory, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::BookHistory.into());
      assert_none!(
        app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .unwrap()
          .book_history
          .filtered_items
      );
    }

    #[test]
    fn test_book_history_details_esc() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookHistory.into());
      app.push_navigation_stack(ActiveReadarrBlock::BookHistoryDetails.into());

      BookDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::BookHistoryDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::BookHistory.into());
      assert_modal_present!(app.data.readarr_data.book_details_modal);
    }

    #[rstest]
    fn test_prompt_esc(
      #[values(
        ActiveReadarrBlock::AutomaticallySearchBookPrompt,
        ActiveReadarrBlock::ManualBookSearchConfirmPrompt,
        ActiveReadarrBlock::DeleteBookFilePrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app.push_navigation_stack(active_readarr_block.into());

      BookDetailsHandler::new(ESC_KEY, &mut app, active_readarr_block, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::BookDetails.into());
      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::models::servarr_models::ReleaseDownloadBody;
    use crate::network::readarr_network::ReadarrEvent;
    use crate::{assert_navigation_popped, assert_navigation_pushed};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveReadarrBlock::BookDetails,
        ActiveReadarrBlock::BookHistory,
        ActiveReadarrBlock::ManualBookSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(active_readarr_block.into());

      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, active_readarr_block.into());
    }

    #[rstest]
    fn test_auto_search_key(
      #[values(
        ActiveReadarrBlock::BookDetails,
        ActiveReadarrBlock::BookHistory,
        ActiveReadarrBlock::ManualBookSearch
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(active_readarr_block.into());

      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveReadarrBlock::AutomaticallySearchBookPrompt.into()
      );
    }

    #[test]
    fn test_auto_search_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        ActiveReadarrBlock::BookDetails,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookDetails.into());
    }

    #[test]
    fn test_automatically_search_book_prompt_confirm() {
      let mut app = App::test_default();
      app.data.readarr_data.books.set_items(books_vec());
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::AutomaticallySearchBookPrompt.into());

      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::AutomaticallySearchBookPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::TriggerAutomaticBookSearch(7))
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::BookDetails.into());
    }

    #[test]
    fn test_delete_book_file_prompt_confirm() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteBookFilePrompt.into());

      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::DeleteBookFilePrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::DeleteBookFile(11))
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::BookDetails.into());
    }

    #[test]
    fn test_manual_book_search_confirm_prompt_confirm() {
      let mut app = App::test_default();
      app.data.readarr_data.book_details_modal = Some(book_details_modal());
      app.push_navigation_stack(ActiveReadarrBlock::ManualBookSearch.into());
      app.push_navigation_stack(ActiveReadarrBlock::ManualBookSearchConfirmPrompt.into());

      BookDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::ManualBookSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::DownloadRelease(ReleaseDownloadBody {
          guid: "guid-1".to_owned(),
          indexer_id: 1,
        }))
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::ManualBookSearch.into());
    }
  }

  #[test]
  fn test_book_details_handler_accepts() {
    let mut book_details_handler_blocks = Vec::new();
    book_details_handler_blocks.extend(BOOK_DETAILS_BLOCKS);
    book_details_handler_blocks.extend(EDITION_DETAILS_BLOCKS);

    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if book_details_handler_blocks.contains(&active_readarr_block) {
        assert!(
          BookDetailsHandler::accepts(active_readarr_block),
          "{active_readarr_block} is not accepted by the BookDetailsHandler"
        );
      } else {
        assert!(
          !BookDetailsHandler::accepts(active_readarr_block),
          "{active_readarr_block} is wrongly accepted by the BookDetailsHandler"
        );
      }
    });
  }

  #[test]
  fn test_delegates_edition_details_blocks_to_edition_details_handler() {
    test_handler_delegation!(
      BookDetailsHandler,
      ActiveReadarrBlock::BookDetails,
      ActiveReadarrBlock::BookEditionDetails
    );
  }

  #[test]
  fn test_book_details_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.data.readarr_data.book_details_modal = Some(book_details_modal());

    let handler =
      BookDetailsHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::BookDetails, None);

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_book_details_handler_is_not_ready_when_the_modal_is_absent() {
    let mut app = App::test_default();

    let handler =
      BookDetailsHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::BookDetails, None);

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_book_details_handler_is_ready_per_tab() {
    let mut app = App::test_default();
    app.data.readarr_data.book_details_modal = Some(book_details_modal());

    for active_readarr_block in [
      ActiveReadarrBlock::BookDetails,
      ActiveReadarrBlock::BookHistory,
      ActiveReadarrBlock::BookFileInfo,
      ActiveReadarrBlock::ManualBookSearch,
    ] {
      let handler = BookDetailsHandler::new(DELETE_KEY, &mut app, active_readarr_block, None);

      assert!(handler.is_ready(), "{active_readarr_block} should be ready");
    }
  }

  #[test]
  fn test_book_details_handler_is_not_ready_when_its_tab_table_is_empty() {
    let mut app = App::test_default();
    app.data.readarr_data.book_details_modal = Some(BookDetailsModal::default());

    for active_readarr_block in [
      ActiveReadarrBlock::BookDetails,
      ActiveReadarrBlock::BookHistory,
      ActiveReadarrBlock::BookFileInfo,
      ActiveReadarrBlock::ManualBookSearch,
    ] {
      let handler = BookDetailsHandler::new(DELETE_KEY, &mut app, active_readarr_block, None);

      assert!(
        !handler.is_ready(),
        "{active_readarr_block} should not be ready"
      );
    }
  }

  #[test]
  fn test_book_details_handler_ignores_special_keys_for_textbox_input() {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = true;

    let handler =
      BookDetailsHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::BookDetails, None);

    assert!(handler.ignore_special_keys());
  }

  const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

  fn selected_edition(app: &App<'_>) -> Edition {
    app
      .data
      .readarr_data
      .book_details_modal
      .as_ref()
      .unwrap()
      .editions
      .current_selection()
      .clone()
  }

  fn edition_details_text(app: &App<'_>) -> String {
    app
      .data
      .readarr_data
      .book_details_modal
      .as_ref()
      .unwrap()
      .edition_details_modal
      .as_ref()
      .unwrap()
      .edition_details
      .get_text()
  }

  fn book_details_modal() -> BookDetailsModal {
    let mut book_details_modal = BookDetailsModal::default();
    book_details_modal.editions.set_items(editions_vec());
    book_details_modal.book_files.set_items(book_files_vec());
    book_details_modal.book_history.set_items(history_vec());
    book_details_modal.book_releases.set_items(release_vec());

    book_details_modal
  }

  fn editions_vec() -> Vec<Edition> {
    vec![
      Edition {
        id: 1,
        book_id: 1,
        title: "Test Edition 1".to_owned(),
        format: Some("Paperback".to_owned()),
        isbn13: Some("isbn-1".to_owned()),
        asin: Some("asin-1".to_owned()),
        publisher: Some("Publisher 1".to_owned()),
        language: Some("eng".to_owned()),
        overview: Some("Overview 1".to_owned()),
        page_count: Some(662),
        monitored: true,
        ratings: Some(Ratings {
          votes: 10,
          value: 4.5,
          popularity: None,
        }),
        ..Edition::default()
      },
      Edition {
        id: 2,
        book_id: 1,
        title: "Test Edition 2".to_owned(),
        format: Some("Hardcover".to_owned()),
        isbn13: Some("isbn-2".to_owned()),
        asin: Some("asin-2".to_owned()),
        publisher: Some("Publisher 2".to_owned()),
        language: Some("eng".to_owned()),
        overview: Some("Overview 2".to_owned()),
        page_count: Some(722),
        monitored: false,
        ..Edition::default()
      },
    ]
  }

  fn book_files_vec() -> Vec<BookFile> {
    vec![BookFile {
      id: 11,
      author_id: 1,
      book_id: 1,
      path: "/nfs/books/Test Author/Test Book.epub".to_owned(),
      size: 1024,
      quality: QualityWrapper {
        quality: Quality {
          name: "EPUB".to_owned(),
        },
      },
      ..BookFile::default()
    }]
  }

  fn books_vec() -> Vec<Book> {
    vec![Book {
      id: 7,
      ..Book::default()
    }]
  }

  fn history_vec() -> Vec<ReadarrHistoryItem> {
    vec![
      ReadarrHistoryItem {
        id: 1,
        source_title: HorizontallyScrollableText::from("Test History 1"),
        ..ReadarrHistoryItem::default()
      },
      ReadarrHistoryItem {
        id: 2,
        source_title: HorizontallyScrollableText::from("Test History 2"),
        ..ReadarrHistoryItem::default()
      },
    ]
  }

  fn release_vec() -> Vec<ReadarrRelease> {
    vec![
      ReadarrRelease {
        guid: "guid-1".to_owned(),
        indexer_id: 1,
        title: HorizontallyScrollableText::from("Test Release 1"),
        ..ReadarrRelease::default()
      },
      ReadarrRelease {
        guid: "guid-2".to_owned(),
        indexer_id: 2,
        title: HorizontallyScrollableText::from("Test Release 2"),
        ..ReadarrRelease::default()
      },
    ]
  }
}
