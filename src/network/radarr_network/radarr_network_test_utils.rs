#[cfg(test)]
pub(in crate::network::radarr_network) mod test_utils {
  use crate::models::HorizontallyScrollableText;
  use crate::models::radarr_models::{
    AddMovieSearchResult, BlocklistItem, BlocklistItemMovie, Collection, CollectionMovie, Credit,
    CreditType, DownloadRecord, DownloadsResponse, IndexerSettings, MediaInfo, MinimumAvailability,
    Movie, MovieCollection, MovieFile, MovieHistoryItem, RadarrRelease, Rating, RatingsList,
  };
  use crate::models::servarr_models::{
    Indexer, IndexerField, Language, Quality, QualityWrapper, RootFolder,
  };
  use chrono::DateTime;
  use serde_json::{Number, Value, json};

  pub const MOVIE_JSON: &str = r#"{
        "id": 1,
        "title": "Test",
        "tmdbId": 1234,
        "originalLanguage": {
          "id": 1,
          "name": "English"
        },
        "sizeOnDisk": 3543348019,
        "status": "Downloaded",
        "overview": "Blah blah blah",
        "path": "/nfs/movies",
        "studio": "21st Century Alex",
        "genres": ["cool", "family", "fun"],
        "year": 2023,
        "monitored": true,
        "hasFile": true,
        "runtime": 120,
        "qualityProfileId": 2222,
        "minimumAvailability": "announced",
        "certification": "R",
        "tags": [1],
        "ratings": {
          "imdb": {
            "value": 9.9
          },
          "tmdb": {
            "value": 9.9
          },
          "rottenTomatoes": {
            "value": 9.9
          }
        },
        "movieFile": {
          "relativePath": "Test.mkv",
          "path": "/nfs/movies/Test.mkv",
          "dateAdded": "2022-12-30T07:37:56Z",
          "mediaInfo": {
            "audioBitrate": 0,
            "audioChannels": 7.1,
            "audioCodec": "AAC",
            "audioLanguages": "eng",
            "audioStreamCount": 1,
            "videoBitDepth": 10,
            "videoBitrate": 0,
            "videoCodec": "x265",
            "videoFps": 23.976,
            "resolution": "1920x804",
            "runTime": "2:00:00",
            "scanType": "Progressive"
          }
        },
        "collection": {
          "id": 123,
          "title": "Test Collection",
          "rootFolderPath": "/nfs/movies",
          "searchOnAdd": true,
          "monitored": true,
          "minimumAvailability": "released",
          "overview": "Collection blah blah blah",
          "qualityProfileId": 2222,
          "movies": [
            {
              "title": "Test",
              "overview": "Collection blah blah blah",
              "year": 2023,
              "runtime": 120,
              "tmdbId": 1234,
              "genres": ["cool", "family", "fun"],
              "ratings": {
                "imdb": {
                  "value": 9.9
                },
                "tmdb": {
                  "value": 9.9
                },
                "rottenTomatoes": {
                  "value": 9.9
                }
              }
            }
          ]
        }
      }"#;

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
      video_codec: Some("x265".to_owned()),
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

  pub fn blocklist_item() -> BlocklistItem {
    BlocklistItem {
      id: 1,
      movie_id: 1,
      source_title: "z movie".to_owned(),
      languages: vec![language()],
      quality: quality_wrapper(),
      custom_formats: Some(vec![language()]),
      date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      protocol: "usenet".to_owned(),
      indexer: "DrunkenSlug (Prowlarr)".to_owned(),
      message: "test message".to_owned(),
      movie: blocklist_item_movie(),
    }
  }

  pub fn blocklist_item_movie() -> BlocklistItemMovie {
    BlocklistItemMovie {
      title: "Test".into(),
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
      studio: Some("21st Century Alex".to_owned()),
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

  pub fn movie_history_item() -> MovieHistoryItem {
    MovieHistoryItem {
      source_title: HorizontallyScrollableText::from("Test"),
      quality: quality_wrapper(),
      languages: vec![language()],
      date: DateTime::from(DateTime::parse_from_rfc3339("2022-12-30T07:37:56Z").unwrap()),
      event_type: "grabbed".to_owned(),
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

  pub fn downloads_response() -> DownloadsResponse {
    DownloadsResponse {
      records: vec![download_record()],
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

  pub fn cast_credit() -> Credit {
    Credit {
      person_name: "Madison Clarke".to_owned(),
      character: Some("Johnny Blaze".to_owned()),
      department: None,
      job: None,
      credit_type: CreditType::Cast,
    }
  }

  pub fn crew_credit() -> Credit {
    Credit {
      person_name: "Alex Clarke".to_owned(),
      character: None,
      department: Some("Music".to_owned()),
      job: Some("Composition".to_owned()),
      credit_type: CreditType::Crew,
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

  pub fn tag() -> Value {
    json!({
      "id": 3,
      "label": "testing"
    })
  }

  pub fn quality_profile() -> Value {
    json!({
      "id": 2222,
      "name": "HD - 1080p"
    })
  }
}
