#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::context_clues::{
    BARE_POPUP_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES, ContextClue, ContextClueProvider,
    SYSTEM_TASKS_CONTEXT_CLUES,
  };
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::lidarr::lidarr_context_clues::{
    ADD_ARTIST_SEARCH_RESULTS_CONTEXT_CLUES, ALBUM_DETAILS_CONTEXT_CLUES,
    ALBUM_HISTORY_CONTEXT_CLUES, ARTIST_DETAILS_CONTEXT_CLUES, ARTIST_HISTORY_CONTEXT_CLUES,
    ARTISTS_CONTEXT_CLUES, LidarrContextClueProvider, MANUAL_ALBUM_SEARCH_CONTEXT_CLUES,
    MANUAL_ARTIST_SEARCH_CONTEXT_CLUES,
  };
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ActiveLidarrBlock, EDIT_ARTIST_BLOCKS, EDIT_INDEXER_BLOCKS,
    INDEXER_SETTINGS_BLOCKS, LidarrData,
  };
  use crate::models::servarr_data::lidarr::modals::AlbumDetailsModal;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use rstest::rstest;

  #[test]
  fn test_artists_context_clues() {
    let mut artists_context_clues_iter = ARTISTS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc)
    );
    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.toggle_monitoring,
        DEFAULT_KEYBINDINGS.toggle_monitoring.desc
      )
    );
    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc)
    );
    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc)
    );
    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, "update all")
    );
    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter")
    );
    assert_none!(artists_context_clues_iter.next());
  }

  #[test]
  fn test_artist_details_context_clues() {
    let mut artist_details_context_clues_iter = ARTIST_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      artist_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc,
      )
    );
    assert_some_eq_x!(
      artist_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, "edit artist")
    );
    assert_some_eq_x!(
      artist_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, "delete album")
    );
    assert_some_eq_x!(
      artist_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.toggle_monitoring,
        "toggle album monitoring",
      )
    );
    assert_some_eq_x!(
      artist_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      artist_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      artist_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc,
      )
    );
    assert_some_eq_x!(
      artist_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(artist_details_context_clues_iter.next());
  }

  #[test]
  fn test_add_artist_search_results_context_clues() {
    let mut add_artist_search_results_context_clues_iter =
      ADD_ARTIST_SEARCH_RESULTS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      add_artist_search_results_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      add_artist_search_results_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "edit search")
    );
    assert_none!(add_artist_search_results_context_clues_iter.next());
  }

  #[test]
  #[should_panic(
    expected = "LidarrContextClueProvider::get_context_clues called with non-Lidarr route"
  )]
  fn test_lidarr_context_clue_provider_get_context_clues_non_lidarr_route() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    LidarrContextClueProvider::get_context_clues(&mut app);
  }

  #[test]
  fn test_artist_history_context_clues() {
    let mut artist_history_context_clues_iter = ARTIST_HISTORY_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      artist_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      artist_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, "edit artist")
    );
    assert_some_eq_x!(
      artist_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      artist_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      artist_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      artist_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      artist_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      artist_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      artist_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter/close")
    );
    assert_none!(artist_history_context_clues_iter.next());
  }

  #[test]
  fn test_manual_artist_search_context_clues() {
    let mut manual_artist_search_context_clues_iter = MANUAL_ARTIST_SEARCH_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      manual_artist_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      manual_artist_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, "edit artist")
    );
    assert_some_eq_x!(
      manual_artist_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      manual_artist_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      manual_artist_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      manual_artist_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      manual_artist_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(manual_artist_search_context_clues_iter.next());
  }

  #[test]
  fn test_album_details_context_clues() {
    let mut album_details_context_clues_iter = ALBUM_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      album_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      album_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.toggle_monitoring,
        DEFAULT_KEYBINDINGS.toggle_monitoring.desc
      )
    );
    assert_some_eq_x!(
      album_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      album_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      album_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_some_eq_x!(
      album_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "episode details")
    );
    assert_some_eq_x!(
      album_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, "delete episode")
    );
    assert_none!(album_details_context_clues_iter.next());
  }

  #[test]
  fn test_album_history_context_clues() {
    let mut album_history_context_clues_iter = ALBUM_HISTORY_CONTEXT_CLUES.iter();
    assert_some_eq_x!(
      album_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      album_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      album_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      album_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      album_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      album_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      album_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter/close")
    );
    assert_none!(album_history_context_clues_iter.next());
  }

  #[test]
  fn test_manual_album_search_context_clues() {
    let mut manual_album_search_context_clues_iter = MANUAL_ALBUM_SEARCH_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      manual_album_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      manual_album_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      manual_album_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      manual_album_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      manual_album_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(manual_album_search_context_clues_iter.next());
  }

  #[rstest]
  #[case(0, ActiveLidarrBlock::ArtistDetails, &ARTIST_DETAILS_CONTEXT_CLUES)]
  #[case(1, ActiveLidarrBlock::ArtistHistory, &ARTIST_HISTORY_CONTEXT_CLUES)]
  #[case(2, ActiveLidarrBlock::ManualArtistSearch, &MANUAL_ARTIST_SEARCH_CONTEXT_CLUES)]
  fn test_lidarr_context_clue_provider_artist_info_tabs(
    #[case] index: usize,
    #[case] active_lidarr_block: ActiveLidarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    app.data.lidarr_data = LidarrData::default();
    app.data.lidarr_data.artist_info_tabs.set_index(index);
    app.push_navigation_stack(active_lidarr_block.into());

    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }

  #[rstest]
  #[case(0, ActiveLidarrBlock::AlbumDetails, &ALBUM_DETAILS_CONTEXT_CLUES)]
  #[case(1, ActiveLidarrBlock::AlbumHistory, &ALBUM_HISTORY_CONTEXT_CLUES)]
  #[case(2, ActiveLidarrBlock::ManualAlbumSearch, &MANUAL_ALBUM_SEARCH_CONTEXT_CLUES)]
  fn test_lidarr_context_clue_provider_album_details_tabs(
    #[case] index: usize,
    #[case] active_lidarr_block: ActiveLidarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    let mut album_details_modal = AlbumDetailsModal::default();
    album_details_modal.album_details_tabs.set_index(index);
    let lidarr_data = LidarrData {
      album_details_modal: Some(album_details_modal),
      ..LidarrData::default()
    };
    app.data.lidarr_data = lidarr_data;
    app.push_navigation_stack(active_lidarr_block.into());

    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }

  #[test]
  fn test_lidarr_context_clue_provider_artists_block() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());

    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &ARTISTS_CONTEXT_CLUES);
  }

  #[test]
  fn test_lidarr_context_clue_provider_artists_sort_prompt_block() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::ArtistsSortPrompt.into());

    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &ARTISTS_CONTEXT_CLUES);
  }

  #[test]
  fn test_lidarr_context_clue_provider_search_artists_block() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::SearchArtists.into());

    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &ARTISTS_CONTEXT_CLUES);
  }

  #[test]
  fn test_lidarr_context_clue_provider_filter_artists_block() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::FilterArtists.into());

    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &ARTISTS_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_lidarr_context_clue_provider_bare_popup_context_clues(
    #[values(
      ActiveLidarrBlock::AddArtistSearchInput,
      ActiveLidarrBlock::AddArtistEmptySearchResults,
      ActiveLidarrBlock::TestAllIndexers
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_lidarr_block.into());

    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &BARE_POPUP_CONTEXT_CLUES);
  }

  #[test]
  fn test_lidarr_context_clue_provider_confirmation_prompt_popup_clues_edit_indexer_blocks() {
    let mut blocks = EDIT_ARTIST_BLOCKS.to_vec();
    blocks.extend(ADD_ROOT_FOLDER_BLOCKS);
    blocks.extend(INDEXER_SETTINGS_BLOCKS);
    blocks.extend(EDIT_INDEXER_BLOCKS);

    for active_lidarr_block in blocks {
      let mut app = App::test_default();
      app.push_navigation_stack(active_lidarr_block.into());

      let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

      assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
    }
  }

  #[test]
  fn test_lidarr_context_clue_provider_add_artist_search_results_context_clues() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());

    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &ADD_ARTIST_SEARCH_RESULTS_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_lidarr_context_clue_provider_confirmation_prompt_context_clues_add_artist_blocks(
    #[values(
      ActiveLidarrBlock::AddArtistPrompt,
      ActiveLidarrBlock::AddArtistSelectMonitor,
      ActiveLidarrBlock::AddArtistSelectMonitorNewItems,
      ActiveLidarrBlock::AddArtistSelectQualityProfile,
      ActiveLidarrBlock::AddArtistSelectMetadataProfile,
      ActiveLidarrBlock::AddArtistSelectRootFolder,
      ActiveLidarrBlock::AddArtistTagsInput,
      ActiveLidarrBlock::AddArtistAlreadyInLibrary
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_lidarr_block.into());

    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[test]
  fn test_lidarr_context_clue_provider_system_tasks_clues() {
    let mut app = App::test_default();

    app.push_navigation_stack(ActiveLidarrBlock::SystemTasks.into());
    let context_clues = LidarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &SYSTEM_TASKS_CONTEXT_CLUES);
  }
}
