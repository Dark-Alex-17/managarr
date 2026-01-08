#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::context_clues::{
    BARE_POPUP_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES, ContextClueProvider,
  };
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::lidarr::lidarr_context_clues::{
    ADD_ARTIST_SEARCH_RESULTS_CONTEXT_CLUES, ARTISTS_CONTEXT_CLUES, LidarrContextClueProvider,
  };
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, EDIT_ARTIST_BLOCKS};
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
      ActiveLidarrBlock::AddArtistEmptySearchResults
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
    for active_lidarr_block in EDIT_ARTIST_BLOCKS {
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
}
