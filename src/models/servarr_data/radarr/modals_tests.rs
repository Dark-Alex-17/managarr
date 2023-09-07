#[cfg(test)]
mod test {
  use crate::models::radarr_models::{Collection, MinimumAvailability, Monitor, Movie, RootFolder};
  use crate::models::servarr_data::radarr::modals::{
    AddMovieModal, EditCollectionModal, EditMovieModal,
  };
  use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
  use crate::models::servarr_data::radarr::radarr_data::RadarrData;
  use crate::models::StatefulTable;
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::Number;
  use strum::IntoEnumIterator;

  #[rstest]
  fn test_edit_movie_modal_from_radarr_data(#[values(true, false)] test_filtered_movies: bool) {
    let mut radarr_data = RadarrData {
      quality_profile_map: BiMap::from_iter([
        (2222, "HD - 1080p".to_owned()),
        (1111, "Any".to_owned()),
      ]),
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      filtered_movies: None,
      ..create_test_radarr_data()
    };
    let movie = Movie {
      path: "/nfs/movies/Test".to_owned(),
      monitored: true,
      quality_profile_id: 2222,
      minimum_availability: MinimumAvailability::Released,
      tags: vec![Number::from(1), Number::from(2)],
      ..Movie::default()
    };

    if test_filtered_movies {
      let mut filtered_movies = StatefulTable::default();
      filtered_movies.set_items(vec![movie]);
      radarr_data.filtered_movies = Some(filtered_movies);
    } else {
      radarr_data.movies.set_items(vec![movie]);
    }

    let edit_movie_modal = EditMovieModal::from(&radarr_data);

    assert_eq!(
      edit_movie_modal.minimum_availability_list.items,
      Vec::from_iter(MinimumAvailability::iter())
    );
    assert_eq!(
      edit_movie_modal
        .minimum_availability_list
        .current_selection(),
      &MinimumAvailability::Released
    );
    assert_eq!(
      edit_movie_modal.quality_profile_list.items,
      vec!["Any".to_owned(), "HD - 1080p".to_owned()]
    );
    assert_str_eq!(
      edit_movie_modal.quality_profile_list.current_selection(),
      "HD - 1080p"
    );
    assert_str_eq!(edit_movie_modal.path.text, "/nfs/movies/Test");
    assert_str_eq!(edit_movie_modal.tags.text, "usenet, test");
    assert_eq!(edit_movie_modal.monitored, Some(true));
  }

  #[test]
  fn test_add_movie_modal_from_radarr_data() {
    let root_folder = RootFolder {
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
      unmapped_folders: None,
    };
    let mut radarr_data = RadarrData {
      quality_profile_map: BiMap::from_iter([
        (2222, "HD - 1080p".to_owned()),
        (1111, "Any".to_owned()),
      ]),
      ..RadarrData::default()
    };
    radarr_data
      .root_folders
      .set_items(vec![root_folder.clone()]);

    let add_movie_modal = AddMovieModal::from(&radarr_data);

    assert_eq!(
      add_movie_modal.monitor_list.items,
      Vec::from_iter(Monitor::iter())
    );
    assert_eq!(
      add_movie_modal.minimum_availability_list.items,
      Vec::from_iter(MinimumAvailability::iter())
    );
    assert_eq!(
      add_movie_modal.quality_profile_list.items,
      vec!["Any".to_owned(), "HD - 1080p".to_owned()]
    );
    assert_eq!(add_movie_modal.root_folder_list.items, vec![root_folder]);
    assert!(add_movie_modal.tags.text.is_empty());
  }

  #[rstest]
  fn test_edit_collection_modal_from_radarr_data(
    #[values(true, false)] test_filtered_collections: bool,
  ) {
    let mut radarr_data = RadarrData {
      quality_profile_map: BiMap::from_iter([
        (2222, "HD - 1080p".to_owned()),
        (1111, "Any".to_owned()),
      ]),
      filtered_collections: None,
      ..create_test_radarr_data()
    };
    let collection = Collection {
      root_folder_path: Some("/nfs/movies/Test".to_owned()),
      monitored: true,
      search_on_add: true,
      quality_profile_id: 2222,
      minimum_availability: MinimumAvailability::Released,
      ..Collection::default()
    };

    if test_filtered_collections {
      let mut filtered_collections = StatefulTable::default();
      filtered_collections.set_items(vec![collection]);
      radarr_data.filtered_collections = Some(filtered_collections);
    } else {
      radarr_data.collections.set_items(vec![collection]);
    }

    let edit_collection_modal = EditCollectionModal::from(&radarr_data);

    assert_eq!(
      edit_collection_modal.minimum_availability_list.items,
      Vec::from_iter(MinimumAvailability::iter())
    );
    assert_eq!(
      edit_collection_modal
        .minimum_availability_list
        .current_selection(),
      &MinimumAvailability::Released
    );
    assert_eq!(
      edit_collection_modal.quality_profile_list.items,
      vec!["Any".to_owned(), "HD - 1080p".to_owned()]
    );
    assert_str_eq!(
      edit_collection_modal
        .quality_profile_list
        .current_selection(),
      "HD - 1080p"
    );
    assert_str_eq!(edit_collection_modal.path.text, "/nfs/movies/Test");
    assert_eq!(edit_collection_modal.monitored, Some(true));
    assert_eq!(edit_collection_modal.search_on_add, Some(true));
  }
}
