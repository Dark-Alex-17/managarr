#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::{LidarrHandler, handle_change_tab_left_right_keys};
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::{assert_navigation_pushed, test_handler_delegation};
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  #[rstest]
  fn test_lidarr_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = LidarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_lidarr_handler_is_ready() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = LidarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_lidarr_handler_accepts() {
    for lidarr_block in ActiveLidarrBlock::iter() {
      assert!(LidarrHandler::accepts(lidarr_block));
    }
  }

  #[rstest]
  #[case(0, ActiveLidarrBlock::System, ActiveLidarrBlock::Downloads)]
  #[case(1, ActiveLidarrBlock::Artists, ActiveLidarrBlock::Blocklist)]
  #[case(2, ActiveLidarrBlock::Downloads, ActiveLidarrBlock::History)]
  #[case(3, ActiveLidarrBlock::Blocklist, ActiveLidarrBlock::RootFolders)]
  #[case(4, ActiveLidarrBlock::History, ActiveLidarrBlock::Indexers)]
  #[case(5, ActiveLidarrBlock::RootFolders, ActiveLidarrBlock::System)]
  #[case(6, ActiveLidarrBlock::Indexers, ActiveLidarrBlock::Artists)]
  fn test_lidarr_handler_change_tab_left_right_keys(
    #[case] index: usize,
    #[case] left_block: ActiveLidarrBlock,
    #[case] right_block: ActiveLidarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.lidarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.key);

    assert_eq!(
      app.data.lidarr_data.main_tabs.get_active_route(),
      left_block.into()
    );
    assert_navigation_pushed!(app, left_block.into());

    app.data.lidarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.key);

    assert_eq!(
      app.data.lidarr_data.main_tabs.get_active_route(),
      right_block.into()
    );
    assert_navigation_pushed!(app, right_block.into());
  }

  #[rstest]
  #[case(0, ActiveLidarrBlock::System, ActiveLidarrBlock::Downloads)]
  #[case(1, ActiveLidarrBlock::Artists, ActiveLidarrBlock::Blocklist)]
  #[case(2, ActiveLidarrBlock::Downloads, ActiveLidarrBlock::History)]
  #[case(3, ActiveLidarrBlock::Blocklist, ActiveLidarrBlock::RootFolders)]
  #[case(4, ActiveLidarrBlock::History, ActiveLidarrBlock::Indexers)]
  #[case(5, ActiveLidarrBlock::RootFolders, ActiveLidarrBlock::System)]
  #[case(6, ActiveLidarrBlock::Indexers, ActiveLidarrBlock::Artists)]
  fn test_lidarr_handler_change_tab_left_right_keys_alt_navigation(
    #[case] index: usize,
    #[case] left_block: ActiveLidarrBlock,
    #[case] right_block: ActiveLidarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.lidarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.alt.unwrap());

    assert_eq!(
      app.data.lidarr_data.main_tabs.get_active_route(),
      left_block.into()
    );
    assert_navigation_pushed!(app, left_block.into());

    app.data.lidarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.alt.unwrap());

    assert_eq!(
      app.data.lidarr_data.main_tabs.get_active_route(),
      right_block.into()
    );
    assert_navigation_pushed!(app, right_block.into());
  }

  #[rstest]
  #[case(0, ActiveLidarrBlock::Artists)]
  #[case(1, ActiveLidarrBlock::Downloads)]
  #[case(2, ActiveLidarrBlock::Blocklist)]
  #[case(3, ActiveLidarrBlock::History)]
  #[case(4, ActiveLidarrBlock::RootFolders)]
  #[case(5, ActiveLidarrBlock::Indexers)]
  #[case(6, ActiveLidarrBlock::System)]
  fn test_lidarr_handler_change_tab_left_right_keys_alt_navigation_no_op_when_ignoring_quit_key(
    #[case] index: usize,
    #[case] block: ActiveLidarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(block.into());
    app.ignore_special_keys_for_textbox_input = true;
    app.data.lidarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.alt.unwrap());

    assert_eq!(
      app.data.lidarr_data.main_tabs.get_active_route(),
      block.into()
    );
    assert_eq!(app.get_current_route(), block.into());

    app.data.lidarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.alt.unwrap());

    assert_eq!(
      app.data.lidarr_data.main_tabs.get_active_route(),
      block.into()
    );
    assert_eq!(app.get_current_route(), block.into());
  }

  #[rstest]
  fn test_delegates_library_blocks_to_library_handler(
    #[values(
      ActiveLidarrBlock::Artists,
      ActiveLidarrBlock::ArtistsSortPrompt,
      ActiveLidarrBlock::FilterArtists,
      ActiveLidarrBlock::FilterArtistsError,
      ActiveLidarrBlock::SearchArtists,
      ActiveLidarrBlock::SearchArtistsError,
      ActiveLidarrBlock::UpdateAllArtistsPrompt,
      ActiveLidarrBlock::DeleteArtistPrompt,
      ActiveLidarrBlock::EditArtistPrompt,
      ActiveLidarrBlock::EditArtistPathInput,
      ActiveLidarrBlock::EditArtistSelectMetadataProfile,
      ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
      ActiveLidarrBlock::EditArtistSelectQualityProfile,
      ActiveLidarrBlock::EditArtistTagsInput
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    test_handler_delegation!(
      LidarrHandler,
      ActiveLidarrBlock::Artists,
      active_lidarr_block
    );
  }

  #[rstest]
  fn test_delegates_downloads_blocks_to_downloads_handler(
    #[values(
      ActiveLidarrBlock::Downloads,
      ActiveLidarrBlock::DeleteDownloadPrompt,
      ActiveLidarrBlock::UpdateDownloadsPrompt
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    test_handler_delegation!(
      LidarrHandler,
      ActiveLidarrBlock::Downloads,
      active_lidarr_block
    );
  }

  #[rstest]
  fn test_delegates_blocklist_blocks_to_blocklist_handler(
    #[values(
      ActiveLidarrBlock::Blocklist,
      ActiveLidarrBlock::BlocklistItemDetails,
      ActiveLidarrBlock::DeleteBlocklistItemPrompt,
      ActiveLidarrBlock::BlocklistClearAllItemsPrompt,
      ActiveLidarrBlock::BlocklistSortPrompt
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    test_handler_delegation!(
      LidarrHandler,
      ActiveLidarrBlock::Blocklist,
      active_lidarr_block
    );
  }

  #[rstest]
  fn test_delegates_history_blocks_to_history_handler(
    #[values(
      ActiveLidarrBlock::History,
      ActiveLidarrBlock::HistoryItemDetails,
      ActiveLidarrBlock::HistorySortPrompt,
      ActiveLidarrBlock::FilterHistory,
      ActiveLidarrBlock::FilterHistoryError,
      ActiveLidarrBlock::SearchHistory,
      ActiveLidarrBlock::SearchHistoryError
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    test_handler_delegation!(
      LidarrHandler,
      ActiveLidarrBlock::History,
      active_lidarr_block
    );
  }

  #[rstest]
  fn test_delegates_root_folders_blocks_to_root_folders_handler(
    #[values(
      ActiveLidarrBlock::RootFolders,
      ActiveLidarrBlock::AddRootFolderPrompt,
      ActiveLidarrBlock::DeleteRootFolderPrompt
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    test_handler_delegation!(
      LidarrHandler,
      ActiveLidarrBlock::RootFolders,
      active_lidarr_block
    );
  }

  #[rstest]
  fn test_delegates_indexers_blocks_to_indexers_handler(
    #[values(
      ActiveLidarrBlock::DeleteIndexerPrompt,
      ActiveLidarrBlock::Indexers,
      ActiveLidarrBlock::AllIndexerSettingsPrompt,
      ActiveLidarrBlock::IndexerSettingsConfirmPrompt,
      ActiveLidarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveLidarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveLidarrBlock::IndexerSettingsRetentionInput,
      ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput
    )]
    active_sonarr_block: ActiveLidarrBlock,
  ) {
    test_handler_delegation!(
      LidarrHandler,
      ActiveLidarrBlock::Indexers,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_system_blocks_to_system_handler(
    #[values(
      ActiveLidarrBlock::System,
      ActiveLidarrBlock::SystemLogs,
      ActiveLidarrBlock::SystemQueuedEvents,
      ActiveLidarrBlock::SystemTasks,
      ActiveLidarrBlock::SystemTaskStartConfirmPrompt,
      ActiveLidarrBlock::SystemUpdates
    )]
    active_sonarr_block: ActiveLidarrBlock,
  ) {
    test_handler_delegation!(
      LidarrHandler,
      ActiveLidarrBlock::System,
      active_sonarr_block
    );
  }
}
