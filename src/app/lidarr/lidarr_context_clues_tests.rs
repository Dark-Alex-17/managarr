#[cfg(test)]
mod tests {
  use crate::app::context_clues::ContextClueProvider;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::lidarr::lidarr_context_clues::LidarrContextClueProvider;
  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ActiveLidarrBlock, ARTISTS_CONTEXT_CLUES,
  };
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;

  #[test]
  fn test_artists_context_clues() {
    let mut artists_context_clues_iter = ARTISTS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      artists_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
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
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter")
    );
    assert_none!(artists_context_clues_iter.next());
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
}
