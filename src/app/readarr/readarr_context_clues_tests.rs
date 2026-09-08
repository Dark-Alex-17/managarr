#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::context_clues::{
    BARE_POPUP_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES, ContextClue, ContextClueProvider,
    SYSTEM_TASKS_CONTEXT_CLUES,
  };
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::readarr::readarr_context_clues::{
    ADD_AUTHOR_SEARCH_RESULTS_CONTEXT_CLUES, AUTHOR_DETAILS_CONTEXT_CLUES,
    AUTHOR_HISTORY_CONTEXT_CLUES, AUTHORS_CONTEXT_CLUES, BOOK_DETAILS_CONTEXT_CLUES,
    BOOK_FILE_CONTEXT_CLUES, BOOK_HISTORY_CONTEXT_CLUES, EDITION_DETAILS_CONTEXT_CLUES,
    MANUAL_AUTHOR_SEARCH_CONTEXT_CLUES, MANUAL_BOOK_SEARCH_CONTEXT_CLUES,
    ReadarrContextClueProvider,
  };
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::servarr_data::readarr::modals::BookDetailsModal;
  use crate::models::servarr_data::readarr::readarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ActiveReadarrBlock, EDIT_AUTHOR_BLOCKS, EDIT_INDEXER_BLOCKS,
    INDEXER_SETTINGS_BLOCKS, ReadarrData,
  };
  use rstest::rstest;

  #[test]
  fn test_authors_context_clues() {
    let mut authors_context_clues_iter = AUTHORS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc)
    );
    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.toggle_monitoring,
        DEFAULT_KEYBINDINGS.toggle_monitoring.desc
      )
    );
    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc)
    );
    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc)
    );
    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, "update all")
    );
    assert_some_eq_x!(
      authors_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter")
    );
    assert_none!(authors_context_clues_iter.next());
  }

  #[test]
  fn test_author_details_context_clues() {
    let mut author_details_context_clues_iter = AUTHOR_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      author_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc,
      )
    );
    assert_some_eq_x!(
      author_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, "edit author")
    );
    assert_some_eq_x!(
      author_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, "delete book")
    );
    assert_some_eq_x!(
      author_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.toggle_monitoring,
        "toggle book monitoring",
      )
    );
    assert_some_eq_x!(
      author_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      author_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      author_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc,
      )
    );
    assert_some_eq_x!(
      author_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(author_details_context_clues_iter.next());
  }

  #[test]
  fn test_add_author_search_results_context_clues() {
    let mut add_author_search_results_context_clues_iter =
      ADD_AUTHOR_SEARCH_RESULTS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      add_author_search_results_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      add_author_search_results_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "edit search")
    );
    assert_none!(add_author_search_results_context_clues_iter.next());
  }

  #[test]
  #[should_panic(
    expected = "ReadarrContextClueProvider::get_context_clues called with non-Readarr route"
  )]
  fn test_readarr_context_clue_provider_get_context_clues_non_readarr_route() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    ReadarrContextClueProvider::get_context_clues(&mut app);
  }

  #[test]
  fn test_author_history_context_clues() {
    let mut author_history_context_clues_iter = AUTHOR_HISTORY_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      author_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      author_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, "edit author")
    );
    assert_some_eq_x!(
      author_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      author_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      author_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      author_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      author_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      author_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      author_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter/close")
    );
    assert_none!(author_history_context_clues_iter.next());
  }

  #[test]
  fn test_manual_author_search_context_clues() {
    let mut manual_author_search_context_clues_iter = MANUAL_AUTHOR_SEARCH_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      manual_author_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      manual_author_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, "edit author")
    );
    assert_some_eq_x!(
      manual_author_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      manual_author_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      manual_author_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      manual_author_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      manual_author_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(manual_author_search_context_clues_iter.next());
  }

  #[test]
  fn test_book_details_context_clues() {
    let mut book_details_context_clues_iter = BOOK_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      book_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      book_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      book_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      book_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_some_eq_x!(
      book_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "edition details")
    );
    assert_none!(book_details_context_clues_iter.next());
  }

  #[test]
  fn test_book_history_context_clues() {
    let mut book_history_context_clues_iter = BOOK_HISTORY_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      book_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      book_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      book_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      book_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      book_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      book_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      book_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter/close")
    );
    assert_none!(book_history_context_clues_iter.next());
  }

  #[test]
  fn test_book_file_context_clues() {
    let mut book_file_context_clues_iter = BOOK_FILE_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      book_file_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      book_file_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, "delete book file")
    );
    assert_some_eq_x!(
      book_file_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(book_file_context_clues_iter.next());
  }

  #[test]
  fn test_manual_book_search_context_clues() {
    let mut manual_book_search_context_clues_iter = MANUAL_BOOK_SEARCH_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      manual_book_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      manual_book_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      manual_book_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      manual_book_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      manual_book_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(manual_book_search_context_clues_iter.next());
  }

  #[test]
  fn test_edition_details_context_clues() {
    let mut edition_details_context_clues_iter = EDITION_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      edition_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      edition_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(edition_details_context_clues_iter.next());
  }

  #[rstest]
  #[case(0, ActiveReadarrBlock::AuthorDetails, &AUTHOR_DETAILS_CONTEXT_CLUES)]
  #[case(1, ActiveReadarrBlock::AuthorHistory, &AUTHOR_HISTORY_CONTEXT_CLUES)]
  #[case(2, ActiveReadarrBlock::ManualAuthorSearch, &MANUAL_AUTHOR_SEARCH_CONTEXT_CLUES)]
  fn test_readarr_context_clue_provider_author_info_tabs(
    #[case] index: usize,
    #[case] active_readarr_block: ActiveReadarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    app.data.readarr_data = ReadarrData::default();
    app.data.readarr_data.author_info_tabs.set_index(index);
    app.push_navigation_stack(active_readarr_block.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }

  #[rstest]
  #[case(0, ActiveReadarrBlock::BookDetails, &BOOK_DETAILS_CONTEXT_CLUES)]
  #[case(1, ActiveReadarrBlock::BookHistory, &BOOK_HISTORY_CONTEXT_CLUES)]
  #[case(2, ActiveReadarrBlock::BookFileInfo, &BOOK_FILE_CONTEXT_CLUES)]
  #[case(3, ActiveReadarrBlock::ManualBookSearch, &MANUAL_BOOK_SEARCH_CONTEXT_CLUES)]
  fn test_readarr_context_clue_provider_book_details_tabs(
    #[case] index: usize,
    #[case] active_readarr_block: ActiveReadarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    let mut book_details_modal = BookDetailsModal::default();
    book_details_modal.book_details_tabs.set_index(index);
    let readarr_data = ReadarrData {
      book_details_modal: Some(book_details_modal),
      ..ReadarrData::default()
    };
    app.data.readarr_data = readarr_data;
    app.push_navigation_stack(active_readarr_block.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }

  #[test]
  fn test_readarr_context_clue_provider_book_edition_details_block() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &EDITION_DETAILS_CONTEXT_CLUES);
  }

  #[test]
  fn test_readarr_context_clue_provider_authors_block() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &AUTHORS_CONTEXT_CLUES);
  }

  #[test]
  fn test_readarr_context_clue_provider_authors_sort_prompt_block() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::AuthorsSortPrompt.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &AUTHORS_CONTEXT_CLUES);
  }

  #[test]
  fn test_readarr_context_clue_provider_search_authors_block() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::SearchAuthors.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &AUTHORS_CONTEXT_CLUES);
  }

  #[test]
  fn test_readarr_context_clue_provider_filter_authors_block() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::FilterAuthors.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &AUTHORS_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_readarr_context_clue_provider_bare_popup_context_clues(
    #[values(
      ActiveReadarrBlock::AddAuthorSearchInput,
      ActiveReadarrBlock::AddAuthorEmptySearchResults,
      ActiveReadarrBlock::TestAllIndexers,
      ActiveReadarrBlock::SystemLogs,
      ActiveReadarrBlock::SystemUpdates
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_readarr_block.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &BARE_POPUP_CONTEXT_CLUES);
  }

  #[test]
  fn test_readarr_context_clue_provider_confirmation_prompt_popup_clues_edit_indexer_blocks() {
    let mut blocks = EDIT_AUTHOR_BLOCKS.to_vec();
    blocks.extend(ADD_ROOT_FOLDER_BLOCKS);
    blocks.extend(INDEXER_SETTINGS_BLOCKS);
    blocks.extend(EDIT_INDEXER_BLOCKS);

    for active_readarr_block in blocks {
      let mut app = App::test_default();
      app.push_navigation_stack(active_readarr_block.into());

      let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

      assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
    }
  }

  #[test]
  fn test_readarr_context_clue_provider_add_author_search_results_context_clues() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &ADD_AUTHOR_SEARCH_RESULTS_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_readarr_context_clue_provider_confirmation_prompt_context_clues_add_author_blocks(
    #[values(
      ActiveReadarrBlock::AddAuthorPrompt,
      ActiveReadarrBlock::AddAuthorSelectMonitor,
      ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
      ActiveReadarrBlock::AddAuthorSelectQualityProfile,
      ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
      ActiveReadarrBlock::AddAuthorSelectRootFolder,
      ActiveReadarrBlock::AddAuthorTagsInput,
      ActiveReadarrBlock::AddAuthorAlreadyInLibrary
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_readarr_block.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_readarr_context_clue_provider_confirmation_prompt_context_clues_delete_author_blocks(
    #[values(
      ActiveReadarrBlock::DeleteAuthorPrompt,
      ActiveReadarrBlock::DeleteAuthorConfirmPrompt,
      ActiveReadarrBlock::DeleteAuthorToggleAddListExclusion,
      ActiveReadarrBlock::DeleteAuthorToggleDeleteFile
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_readarr_block.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_readarr_context_clue_provider_confirmation_prompt_context_clues_delete_book_blocks(
    #[values(
      ActiveReadarrBlock::DeleteBookPrompt,
      ActiveReadarrBlock::DeleteBookConfirmPrompt,
      ActiveReadarrBlock::DeleteBookToggleAddListExclusion,
      ActiveReadarrBlock::DeleteBookToggleDeleteFile
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_readarr_block.into());

    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[test]
  fn test_readarr_context_clue_provider_system_tasks_clues() {
    let mut app = App::test_default();

    app.push_navigation_stack(ActiveReadarrBlock::SystemTasks.into());
    let context_clues = ReadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &SYSTEM_TASKS_CONTEXT_CLUES);
  }
}
