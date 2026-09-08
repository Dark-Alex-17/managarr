#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::{ReadarrHandler, handle_change_tab_left_right_keys};
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use crate::{assert_navigation_pushed, test_handler_delegation};
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  #[rstest]
  fn test_readarr_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = ReadarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_readarr_handler_is_ready() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = ReadarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::default(),
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_readarr_handler_accepts() {
    for readarr_block in ActiveReadarrBlock::iter() {
      assert!(ReadarrHandler::accepts(readarr_block));
    }
  }

  #[rstest]
  #[case(0, ActiveReadarrBlock::System, ActiveReadarrBlock::Downloads)]
  #[case(1, ActiveReadarrBlock::Authors, ActiveReadarrBlock::Blocklist)]
  #[case(2, ActiveReadarrBlock::Downloads, ActiveReadarrBlock::History)]
  #[case(3, ActiveReadarrBlock::Blocklist, ActiveReadarrBlock::RootFolders)]
  #[case(4, ActiveReadarrBlock::History, ActiveReadarrBlock::Indexers)]
  #[case(5, ActiveReadarrBlock::RootFolders, ActiveReadarrBlock::System)]
  #[case(6, ActiveReadarrBlock::Indexers, ActiveReadarrBlock::Authors)]
  fn test_readarr_handler_change_tab_left_right_keys(
    #[case] index: usize,
    #[case] left_block: ActiveReadarrBlock,
    #[case] right_block: ActiveReadarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.readarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.key);

    assert_eq!(
      app.data.readarr_data.main_tabs.get_active_route(),
      left_block.into()
    );
    assert_navigation_pushed!(app, left_block.into());

    app.data.readarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.key);

    assert_eq!(
      app.data.readarr_data.main_tabs.get_active_route(),
      right_block.into()
    );
    assert_navigation_pushed!(app, right_block.into());
  }

  #[rstest]
  #[case(0, ActiveReadarrBlock::System, ActiveReadarrBlock::Downloads)]
  #[case(1, ActiveReadarrBlock::Authors, ActiveReadarrBlock::Blocklist)]
  #[case(2, ActiveReadarrBlock::Downloads, ActiveReadarrBlock::History)]
  #[case(3, ActiveReadarrBlock::Blocklist, ActiveReadarrBlock::RootFolders)]
  #[case(4, ActiveReadarrBlock::History, ActiveReadarrBlock::Indexers)]
  #[case(5, ActiveReadarrBlock::RootFolders, ActiveReadarrBlock::System)]
  #[case(6, ActiveReadarrBlock::Indexers, ActiveReadarrBlock::Authors)]
  fn test_readarr_handler_change_tab_left_right_keys_alt_navigation(
    #[case] index: usize,
    #[case] left_block: ActiveReadarrBlock,
    #[case] right_block: ActiveReadarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.readarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.alt.unwrap());

    assert_eq!(
      app.data.readarr_data.main_tabs.get_active_route(),
      left_block.into()
    );
    assert_navigation_pushed!(app, left_block.into());

    app.data.readarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.alt.unwrap());

    assert_eq!(
      app.data.readarr_data.main_tabs.get_active_route(),
      right_block.into()
    );
    assert_navigation_pushed!(app, right_block.into());
  }

  #[rstest]
  #[case(0, ActiveReadarrBlock::Authors)]
  #[case(1, ActiveReadarrBlock::Downloads)]
  #[case(2, ActiveReadarrBlock::Blocklist)]
  #[case(3, ActiveReadarrBlock::History)]
  #[case(4, ActiveReadarrBlock::RootFolders)]
  #[case(5, ActiveReadarrBlock::Indexers)]
  #[case(6, ActiveReadarrBlock::System)]
  fn test_readarr_handler_change_tab_left_right_keys_alt_navigation_no_op_when_ignoring_quit_key(
    #[case] index: usize,
    #[case] block: ActiveReadarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(block.into());
    app.ignore_special_keys_for_textbox_input = true;
    app.data.readarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.alt.unwrap());

    assert_eq!(
      app.data.readarr_data.main_tabs.get_active_route(),
      block.into()
    );
    assert_eq!(app.get_current_route(), block.into());

    app.data.readarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.alt.unwrap());

    assert_eq!(
      app.data.readarr_data.main_tabs.get_active_route(),
      block.into()
    );
    assert_eq!(app.get_current_route(), block.into());
  }

  #[rstest]
  fn test_delegates_library_blocks_to_library_handler(
    #[values(
      ActiveReadarrBlock::Authors,
      ActiveReadarrBlock::AuthorsSortPrompt,
      ActiveReadarrBlock::FilterAuthors,
      ActiveReadarrBlock::FilterAuthorsError,
      ActiveReadarrBlock::SearchAuthors,
      ActiveReadarrBlock::SearchAuthorsError,
      ActiveReadarrBlock::UpdateAllAuthorsPrompt
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      ReadarrHandler,
      ActiveReadarrBlock::Authors,
      active_readarr_block
    );
  }

  #[rstest]
  fn test_delegates_blocklist_blocks_to_blocklist_handler(
    #[values(
      ActiveReadarrBlock::Blocklist,
      ActiveReadarrBlock::BlocklistItemDetails,
      ActiveReadarrBlock::DeleteBlocklistItemPrompt,
      ActiveReadarrBlock::BlocklistClearAllItemsPrompt,
      ActiveReadarrBlock::BlocklistSortPrompt
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      ReadarrHandler,
      ActiveReadarrBlock::Blocklist,
      active_readarr_block
    );
  }

  #[rstest]
  fn test_delegates_downloads_blocks_to_downloads_handler(
    #[values(
      ActiveReadarrBlock::Downloads,
      ActiveReadarrBlock::DeleteDownloadPrompt,
      ActiveReadarrBlock::UpdateDownloadsPrompt
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      ReadarrHandler,
      ActiveReadarrBlock::Downloads,
      active_readarr_block
    );
  }

  #[rstest]
  fn test_delegates_history_blocks_to_history_handler(
    #[values(
      ActiveReadarrBlock::History,
      ActiveReadarrBlock::HistoryItemDetails,
      ActiveReadarrBlock::HistorySortPrompt,
      ActiveReadarrBlock::SearchHistory,
      ActiveReadarrBlock::SearchHistoryError,
      ActiveReadarrBlock::FilterHistory,
      ActiveReadarrBlock::FilterHistoryError
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      ReadarrHandler,
      ActiveReadarrBlock::History,
      active_readarr_block
    );
  }

  #[rstest]
  fn test_delegates_root_folders_blocks_to_root_folders_handler(
    #[values(
      ActiveReadarrBlock::RootFolders,
      ActiveReadarrBlock::AddRootFolderPrompt,
      ActiveReadarrBlock::DeleteRootFolderPrompt
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      ReadarrHandler,
      ActiveReadarrBlock::RootFolders,
      active_readarr_block
    );
  }

  #[rstest]
  fn test_delegates_indexers_blocks_to_indexers_handler(
    #[values(
      ActiveReadarrBlock::Indexers,
      ActiveReadarrBlock::DeleteIndexerPrompt,
      ActiveReadarrBlock::TestIndexer,
      ActiveReadarrBlock::TestAllIndexers,
      ActiveReadarrBlock::EditIndexerPrompt,
      ActiveReadarrBlock::AllIndexerSettingsPrompt,
      ActiveReadarrBlock::IndexerSettingsConfirmPrompt,
      ActiveReadarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveReadarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveReadarrBlock::IndexerSettingsRetentionInput,
      ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      ReadarrHandler,
      ActiveReadarrBlock::Indexers,
      active_readarr_block
    );
  }

  #[rstest]
  fn test_delegates_system_blocks_to_system_handler(
    #[values(
      ActiveReadarrBlock::System,
      ActiveReadarrBlock::SystemLogs,
      ActiveReadarrBlock::SystemQueuedEvents,
      ActiveReadarrBlock::SystemTasks,
      ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
      ActiveReadarrBlock::SystemUpdates
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      ReadarrHandler,
      ActiveReadarrBlock::System,
      active_readarr_block
    );
  }
}
