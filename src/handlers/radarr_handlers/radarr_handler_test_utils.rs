#[cfg(test)]
#[macro_use]
pub(in crate::handlers::radarr_handlers) mod utils {
  use crate::models::radarr_models::{
    AddMovieBody, AddMovieOptions, AddMovieSearchResult, Collection, CollectionMovie,
    DownloadRecord, IndexerSettings, MediaInfo, MinimumAvailability, Movie, MovieCollection,
    MovieFile, RadarrRelease, Rating, RatingsList,
  };
  use crate::models::servarr_models::{
    Indexer, IndexerField, Language, Quality, QualityWrapper, RootFolder,
  };
  use crate::models::HorizontallyScrollableText;
  use chrono::DateTime;
  use serde_json::{json, Number};

  #[macro_export]
  macro_rules! test_edit_movie_key {
    ($handler:ident, $block:expr, $context:expr) => {
      let mut app = App::test_default();
      let mut radarr_data = RadarrData {
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        tags_map: BiMap::from_iter([(1, "test".to_owned())]),
        ..create_test_radarr_data()
      };
      radarr_data.movies.set_items(vec![Movie {
        path: "/nfs/movies/Test".to_owned().into(),
        monitored: true,
        quality_profile_id: 2222,
        minimum_availability: MinimumAvailability::Released,
        tags: vec![Number::from(1)],
        ..Movie::default()
      }]);
      app.data.radarr_data = radarr_data;

      $handler::new(DEFAULT_KEYBINDINGS.edit.key, &mut app, $block, None).handle();

      assert_eq!(
        app.get_current_route(),
        (ActiveRadarrBlock::EditMoviePrompt, Some($context)).into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::EditMovieToggleMonitored
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "/nfs/movies/Test"
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "test"
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(true)
      );
      assert_eq!(
        app.data.radarr_data.selected_block.blocks,
        EDIT_MOVIE_SELECTION_BLOCKS
      );
    };
  }

  #[macro_export]
  macro_rules! test_edit_collection_key {
    ($handler:ident, $block:expr, $context:expr) => {
      let mut app = App::test_default();
      let mut radarr_data = RadarrData {
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        ..create_test_radarr_data()
      };
      radarr_data.collections.set_items(vec![Collection {
        root_folder_path: "/nfs/movies/Test".to_owned().into(),
        monitored: true,
        search_on_add: true,
        quality_profile_id: 2222,
        minimum_availability: MinimumAvailability::Released,
        ..Collection::default()
      }]);
      app.data.radarr_data = radarr_data;

      $handler::new(DEFAULT_KEYBINDINGS.edit.key, &mut app, $block, None).handle();

      assert_eq!(
        app.get_current_route(),
        (ActiveRadarrBlock::EditCollectionPrompt, $context).into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::EditCollectionToggleMonitored
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "/nfs/movies/Test"
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(true)
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .search_on_add,
        Some(true)
      );
      assert_eq!(
        app.data.radarr_data.selected_block.blocks,
        EDIT_COLLECTION_SELECTION_BLOCKS
      );
    };
  }

  pub fn language() -> Language {
    Language {
      id: 1,
      name: "English".to_owned(),
    }
  }

  pub fn genres() -> Vec<String> {
    vec!["cool".to_owned(), "family".to_owned(), "fun".to_owned()]
  }

  pub fn rating() -> Rating {
    Rating {
      value: Number::from_f64(9.9).unwrap(),
    }
  }

  pub fn ratings_list() -> RatingsList {
    RatingsList {
      imdb: Some(rating()),
      tmdb: Some(rating()),
      rotten_tomatoes: Some(rating()),
    }
  }

  pub fn media_info() -> MediaInfo {
    MediaInfo {
      audio_bitrate: 0,
      audio_channels: Number::from_f64(7.1).unwrap(),
      audio_codec: Some("AAC".to_owned()),
      audio_languages: Some("eng".to_owned()),
      audio_stream_count: 1,
      video_bit_depth: 10,
      video_bitrate: 0,
      video_codec: "x265".to_owned(),
      video_fps: Number::from_f64(23.976).unwrap(),
      resolution: "1920x804".to_owned(),
      run_time: "2:00:00".to_owned(),
      scan_type: "Progressive".to_owned(),
    }
  }

  pub fn movie_file() -> MovieFile {
    MovieFile {
      relative_path: "Test.mkv".to_owned(),
      path: "/nfs/movies/Test.mkv".to_owned(),
      date_added: DateTime::from(DateTime::parse_from_rfc3339("2022-12-30T07:37:56Z").unwrap()),
      media_info: Some(media_info()),
    }
  }

  pub fn collection_movie() -> CollectionMovie {
    CollectionMovie {
      title: "Test".to_owned().into(),
      overview: "Collection blah blah blah".to_owned(),
      year: 2023,
      runtime: 120,
      tmdb_id: 1234,
      genres: genres(),
      ratings: ratings_list(),
    }
  }

  pub fn collection() -> Collection {
    Collection {
      id: 123,
      title: "Test Collection".to_owned().into(),
      root_folder_path: Some("/nfs/movies".to_owned()),
      search_on_add: true,
      monitored: true,
      minimum_availability: MinimumAvailability::Released,
      overview: Some("Collection blah blah blah".to_owned()),
      quality_profile_id: 2222,
      movies: Some(vec![collection_movie()]),
    }
  }

  pub fn movie() -> Movie {
    Movie {
      id: 1,
      title: "Test".to_owned().into(),
      original_language: language(),
      size_on_disk: 3543348019,
      status: "Downloaded".to_owned(),
      overview: "Blah blah blah".to_owned(),
      path: "/nfs/movies".to_owned(),
      studio: "21st Century Alex".to_owned(),
      genres: genres(),
      year: 2023,
      monitored: true,
      has_file: true,
      runtime: 120,
      tmdb_id: 1234,
      quality_profile_id: 2222,
      minimum_availability: MinimumAvailability::Announced,
      certification: Some("R".to_owned()),
      tags: vec![Number::from(1)],
      ratings: ratings_list(),
      movie_file: Some(movie_file()),
      collection: Some(movie_collection()),
    }
  }

  pub fn movie_collection() -> MovieCollection {
    MovieCollection {
      title: Some("Test Collection".to_owned()),
    }
  }

  pub fn rejections() -> Vec<String> {
    vec![
      "Unknown quality profile".to_owned(),
      "Release is already mapped".to_owned(),
    ]
  }

  pub fn quality() -> Quality {
    Quality {
      name: "HD - 1080p".to_owned(),
    }
  }

  pub fn quality_wrapper() -> QualityWrapper {
    QualityWrapper { quality: quality() }
  }

  pub fn release() -> RadarrRelease {
    RadarrRelease {
      guid: "1234".to_owned(),
      protocol: "torrent".to_owned(),
      age: 1,
      title: HorizontallyScrollableText::from("Test Release"),
      indexer: "kickass torrents".to_owned(),
      indexer_id: 2,
      size: 1234,
      rejected: true,
      rejections: Some(rejections()),
      seeders: Some(Number::from(2)),
      leechers: Some(Number::from(1)),
      languages: Some(vec![language()]),
      quality: quality_wrapper(),
    }
  }

  pub fn add_movie_search_result() -> AddMovieSearchResult {
    AddMovieSearchResult {
      tmdb_id: 1234,
      title: HorizontallyScrollableText::from("Test"),
      original_language: language(),
      status: "released".to_owned(),
      overview: "New movie blah blah blah".to_owned(),
      genres: genres(),
      year: 2023,
      runtime: 120,
      ratings: ratings_list(),
    }
  }

  pub fn download_record() -> DownloadRecord {
    DownloadRecord {
      title: "Test Download Title".to_owned(),
      status: "downloading".to_owned(),
      id: 1,
      movie_id: 1,
      size: 3543348019,
      sizeleft: 1771674009,
      output_path: Some(HorizontallyScrollableText::from("/nfs/movies/Test")),
      indexer: "kickass torrents".to_owned(),
      download_client: "transmission".to_owned(),
    }
  }

  pub fn root_folder() -> RootFolder {
    RootFolder {
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
      unmapped_folders: None,
    }
  }

  pub fn indexer() -> Indexer {
    Indexer {
      enable_rss: true,
      enable_automatic_search: true,
      enable_interactive_search: true,
      supports_rss: true,
      supports_search: true,
      protocol: "torrent".to_owned(),
      priority: 25,
      download_client_id: 0,
      name: Some("Test Indexer".to_owned()),
      implementation_name: Some("Torznab".to_owned()),
      implementation: Some("Torznab".to_owned()),
      config_contract: Some("TorznabSettings".to_owned()),
      tags: vec![Number::from(1)],
      id: 1,
      fields: Some(vec![
        IndexerField {
          name: Some("baseUrl".to_owned()),
          value: Some(json!("https://test.com")),
        },
        IndexerField {
          name: Some("apiKey".to_owned()),
          value: Some(json!("")),
        },
        IndexerField {
          name: Some("seedCriteria.seedRatio".to_owned()),
          value: Some(json!("1.2")),
        },
      ]),
    }
  }

  pub fn indexer_settings() -> IndexerSettings {
    IndexerSettings {
      rss_sync_interval: 60,
      allow_hardcoded_subs: true,
      id: 1,
      ..IndexerSettings::default()
    }
  }

  pub fn add_movie_body() -> AddMovieBody {
    AddMovieBody {
      tmdb_id: 1234,
      title: "Test".to_owned(),
      root_folder_path: "/nfs2".to_owned(),
      minimum_availability: "announced".to_owned(),
      monitored: true,
      quality_profile_id: 2222,
      tags: Vec::new(),
      tag_input_string: Some("usenet, testing".into()),
      add_options: AddMovieOptions {
        monitor: "movieOnly".to_owned(),
        search_for_movie: true,
      },
    }
  }
}
