#[cfg(test)]
pub mod utils {
  use crate::models::radarr_models::{AddMovieSearchResult, BlocklistItem, Collection, CollectionMovie, Credit, DownloadRecord, Movie, MovieHistoryItem, RadarrRelease};
  use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
  use crate::models::servarr_data::radarr::radarr_data::RadarrData;
  use crate::models::stateful_table::StatefulTable;
  use crate::models::{HorizontallyScrollableText, ScrollableText};
  use crate::models::servarr_models::{Indexer, RootFolder};

  pub fn create_test_radarr_data<'a>() -> RadarrData<'a> {
    let mut movie_details_modal = MovieDetailsModal {
      movie_details: ScrollableText::with_string("test movie details".to_owned()),
      ..MovieDetailsModal::default()
    };
    movie_details_modal
      .movie_history
      .set_items(vec![MovieHistoryItem::default()]);
    movie_details_modal
      .movie_cast
      .set_items(vec![Credit::default()]);
    movie_details_modal
      .movie_crew
      .set_items(vec![Credit::default()]);
    movie_details_modal
      .movie_releases
      .set_items(vec![RadarrRelease::default()]);

    let mut radarr_data = RadarrData {
      delete_movie_files: true,
      add_list_exclusion: true,
      add_movie_search: Some("test search".into()),
      edit_root_folder: Some("test path".into()),
      movie_details_modal: Some(movie_details_modal),
      add_searched_movies: Some(StatefulTable::default()),
      ..RadarrData::default()
    };
    radarr_data.movies.set_items(vec![Movie::default()]);
    radarr_data.collection_movies.set_items(vec![CollectionMovie::default()]);
    radarr_data.collections.set_items(vec![Collection::default()]);
    radarr_data.downloads.set_items(vec![DownloadRecord::default()]);
    radarr_data.blocklist.set_items(vec![BlocklistItem::default()]);
    radarr_data.root_folders.set_items(vec![RootFolder::default()]);
    radarr_data.indexers.set_items(vec![Indexer::default()]);
    radarr_data.movie_info_tabs.index = 1;
    radarr_data
      .add_searched_movies
      .as_mut()
      .unwrap()
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
  macro_rules! assert_movie_info_tabs_reset {
    ($radarr_data:expr) => {
      assert!($radarr_data.movie_details_modal.is_none());
      assert_eq!($radarr_data.movie_info_tabs.index, 0);
    };
  }
}
