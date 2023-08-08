#[cfg(test)]
pub mod utils {
  use crate::models::radarr_models::{
    AddMovieSearchResult, Collection, CollectionMovie, Credit, Movie, MovieHistoryItem, Release,
    ReleaseField,
  };
  use crate::models::servarr_data::radarr::radarr_data::RadarrData;
  use crate::models::{HorizontallyScrollableText, ScrollableText};

  pub fn create_test_radarr_data<'a>() -> RadarrData<'a> {
    let mut radarr_data = RadarrData {
      is_searching: true,
      is_filtering: true,
      delete_movie_files: true,
      add_list_exclusion: true,
      search: Some("test search".into()),
      filter: Some("test filter".into()),
      edit_root_folder: Some("test path".into()),
      file_details: "test file details".to_owned(),
      audio_details: "test audio details".to_owned(),
      video_details: "test video details".to_owned(),
      movie_details: ScrollableText::with_string("test movie details".to_owned()),
      ..RadarrData::default()
    };
    radarr_data
      .movie_history
      .set_items(vec![MovieHistoryItem::default()]);
    radarr_data.movie_cast.set_items(vec![Credit::default()]);
    radarr_data.movie_crew.set_items(vec![Credit::default()]);
    radarr_data
      .movie_releases
      .set_items(vec![Release::default()]);
    radarr_data.movie_info_tabs.index = 1;
    radarr_data
      .movie_releases_sort
      .set_items(vec![ReleaseField::default()]);
    radarr_data.sort_ascending = Some(true);
    radarr_data
      .filtered_movies
      .set_items(vec![Movie::default()]);
    radarr_data
      .filtered_collections
      .set_items(vec![Collection::default()]);
    radarr_data
      .add_searched_movies
      .set_items(vec![AddMovieSearchResult::default()]);
    radarr_data
      .collection_movies
      .set_items(vec![CollectionMovie::default()]);
    radarr_data
      .log_details
      .set_items(vec![HorizontallyScrollableText::default()]);

    radarr_data
  }

  #[macro_export]
  macro_rules! assert_search_reset {
    ($radarr_data:expr) => {
      assert!(!$radarr_data.is_searching);
      assert!($radarr_data.search.is_none());
      assert!($radarr_data.filter.is_none());
      assert!($radarr_data.filtered_movies.items.is_empty());
      assert!($radarr_data.filtered_collections.items.is_empty());
      assert!($radarr_data.add_searched_movies.items.is_empty());
    };
  }

  #[macro_export]
  macro_rules! assert_filter_reset {
    ($radarr_data:expr) => {
      assert!(!$radarr_data.is_filtering);
      assert!($radarr_data.filter.is_none());
      assert!($radarr_data.filtered_movies.items.is_empty());
      assert!($radarr_data.filtered_collections.items.is_empty());
    };
  }

  #[macro_export]
  macro_rules! assert_movie_info_tabs_reset {
    ($radarr_data:expr) => {
      assert!($radarr_data.file_details.is_empty());
      assert!($radarr_data.audio_details.is_empty());
      assert!($radarr_data.video_details.is_empty());
      assert!($radarr_data.movie_details.get_text().is_empty());
      assert!($radarr_data.movie_history.items.is_empty());
      assert!($radarr_data.movie_cast.items.is_empty());
      assert!($radarr_data.movie_crew.items.is_empty());
      assert!($radarr_data.movie_releases.items.is_empty());
      assert!($radarr_data.movie_releases_sort.items.is_empty());
      assert!($radarr_data.sort_ascending.is_none());
      assert_eq!($radarr_data.movie_info_tabs.index, 0);
    };
  }
}
