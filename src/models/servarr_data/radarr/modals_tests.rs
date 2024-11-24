#[cfg(test)]
mod test {
  use crate::models::radarr_models::{Collection, MinimumAvailability, Movie, MovieMonitor};
  use crate::models::servarr_data::radarr::modals::{
    AddMovieModal, EditCollectionModal, EditIndexerModal, EditMovieModal,
  };
  use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
  use crate::models::servarr_data::radarr::radarr_data::RadarrData;
  use crate::models::servarr_models::{Indexer, IndexerField, RootFolder};
  use crate::models::stateful_table::StatefulTable;
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::{Number, Value};
  use strum::IntoEnumIterator;

  #[rstest]
  fn test_edit_indexer_modal_from_radarr_data(#[values(true, false)] seed_ratio_present: bool) {
    use crate::models::servarr_models::{Indexer, IndexerField};

    let mut radarr_data = RadarrData {
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      ..RadarrData::default()
    };
    let mut fields = vec![
      IndexerField {
        name: Some("baseUrl".to_owned()),
        value: Some(Value::String("https://test.com".to_owned())),
      },
      IndexerField {
        name: Some("apiKey".to_owned()),
        value: Some(Value::String("1234".to_owned())),
      },
    ];

    if seed_ratio_present {
      fields.push(IndexerField {
        name: Some("seedCriteria.seedRatio".to_owned()),
        value: Some(Value::from(1.2f64)),
      });
    }

    let indexer = Indexer {
      name: Some("Test".to_owned()),
      enable_rss: true,
      enable_automatic_search: true,
      enable_interactive_search: true,
      tags: vec![Number::from(1), Number::from(2)],
      fields: Some(fields),
      ..Indexer::default()
    };
    radarr_data.indexers.set_items(vec![indexer]);

    let edit_indexer_modal = EditIndexerModal::from(&radarr_data);

    assert_str_eq!(edit_indexer_modal.name.text, "Test");
    assert_eq!(edit_indexer_modal.enable_rss, Some(true));
    assert_eq!(edit_indexer_modal.enable_automatic_search, Some(true));
    assert_eq!(edit_indexer_modal.enable_interactive_search, Some(true));
    assert_str_eq!(edit_indexer_modal.url.text, "https://test.com");
    assert_str_eq!(edit_indexer_modal.api_key.text, "1234");

    if seed_ratio_present {
      assert_str_eq!(edit_indexer_modal.seed_ratio.text, "1.2");
    } else {
      assert!(edit_indexer_modal.seed_ratio.text.is_empty());
    }
  }

  #[test]
  fn test_edit_indexer_modal_from_radarr_data_seed_ratio_value_is_none() {
    let mut radarr_data = RadarrData {
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      ..RadarrData::default()
    };
    let fields = vec![
      IndexerField {
        name: Some("baseUrl".to_owned()),
        value: Some(Value::String("https://test.com".to_owned())),
      },
      IndexerField {
        name: Some("apiKey".to_owned()),
        value: Some(Value::String("1234".to_owned())),
      },
      IndexerField {
        name: Some("seedCriteria.seedRatio".to_owned()),
        value: None,
      },
    ];

    let indexer = Indexer {
      name: Some("Test".to_owned()),
      enable_rss: true,
      enable_automatic_search: true,
      enable_interactive_search: true,
      tags: vec![Number::from(1), Number::from(2)],
      fields: Some(fields),
      ..Indexer::default()
    };
    radarr_data.indexers.set_items(vec![indexer]);

    let edit_indexer_modal = EditIndexerModal::from(&radarr_data);

    assert_str_eq!(edit_indexer_modal.name.text, "Test");
    assert_eq!(edit_indexer_modal.enable_rss, Some(true));
    assert_eq!(edit_indexer_modal.enable_automatic_search, Some(true));
    assert_eq!(edit_indexer_modal.enable_interactive_search, Some(true));
    assert_str_eq!(edit_indexer_modal.url.text, "https://test.com");
    assert_str_eq!(edit_indexer_modal.api_key.text, "1234");
    assert!(edit_indexer_modal.seed_ratio.text.is_empty());
  }

  #[rstest]
  fn test_edit_movie_modal_from_radarr_data(#[values(true, false)] test_filtered_movies: bool) {
    let mut radarr_data = RadarrData {
      quality_profile_map: BiMap::from_iter([
        (2222, "HD - 1080p".to_owned()),
        (1111, "Any".to_owned()),
      ]),
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      movies: StatefulTable::default(),
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
      radarr_data.movies.set_filtered_items(vec![movie]);
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
      Vec::from_iter(MovieMonitor::iter())
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
      collections: StatefulTable::default(),
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
      radarr_data.collections.set_filtered_items(vec![collection]);
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
