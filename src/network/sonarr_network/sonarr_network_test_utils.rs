#[cfg(test)]
pub(in crate::network::sonarr_network) mod test_utils {
  use crate::models::servarr_models::{
    Indexer, IndexerField, Language, Quality, QualityWrapper, RootFolder,
  };
  use crate::models::sonarr_models::{
    AddSeriesSearchResult, AddSeriesSearchResultStatistics, BlocklistItem, DownloadRecord,
    DownloadStatus, DownloadsResponse, Episode, EpisodeFile, IndexerSettings, MediaInfo, Rating,
    Season, SeasonStatistics, Series, SeriesStatistics, SeriesStatus, SeriesType,
    SonarrHistoryData, SonarrHistoryEventType, SonarrHistoryItem, SonarrRelease,
  };
  use crate::models::HorizontallyScrollableText;
  use chrono::DateTime;
  use serde_json::{json, Number};

  pub const SERIES_JSON: &str = r#"{
        "title": "Test",
        "status": "continuing",
        "ended": false,
        "overview": "Blah blah blah",
        "network": "HBO",
        "seasons": [
            {
                "seasonNumber": 1,
                "monitored": true,
                "statistics": {
                    "previousAiring": "2022-10-24T01:00:00Z",
                    "episodeFileCount": 10,
                    "episodeCount": 10,
                    "totalEpisodeCount": 10,
                    "sizeOnDisk": 36708563419,
                    "percentOfEpisodes": 100.0
                }
            }
        ],
        "year": 2022,
        "path": "/nfs/tv/Test",
        "qualityProfileId": 6,
        "languageProfileId": 1,
        "seasonFolder": true,
        "monitored": true,
        "runtime": 63,
        "tvdbId": 371572,
        "seriesType": "standard",
        "certification": "TV-MA",
        "genres": ["cool", "family", "fun"],
        "tags": [3],
        "ratings": {"votes": 406744, "value": 8.4},
        "statistics": {
            "seasonCount": 2,
            "episodeFileCount": 18,
            "episodeCount": 18,
            "totalEpisodeCount": 50,
            "sizeOnDisk": 63894022699,
            "percentOfEpisodes": 100.0
        },
        "id": 1
    }
"#;

  pub const EPISODE_JSON: &str = r#"{
    "seriesId": 1,
    "tvdbId": 1234,
    "episodeFileId": 1,
    "seasonNumber": 1,
    "episodeNumber": 1,
    "title": "Something cool",
    "airDateUtc": "2024-02-10T07:28:45Z",
    "overview": "Okay so this one time at band camp...",
    "episodeFile": {
        "id": 1,
        "relativePath": "/season 1/episode 1.mkv",
        "path": "/nfs/tv/series/season 1/episode 1.mkv",
        "size": 3543348019,
        "dateAdded": "2024-02-10T07:28:45Z",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" } },
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
            "resolution": "1920x1080",
            "runTime": "23:51",
            "scanType": "Progressive",
            "subtitles": "English"
        }
    },
    "hasFile": true,
    "monitored": true,
    "id": 1
  }"#;

  pub fn add_series_search_result() -> AddSeriesSearchResult {
    AddSeriesSearchResult {
      tvdb_id: 1234,
      title: HorizontallyScrollableText::from("Test"),
      status: Some("continuing".to_owned()),
      ended: false,
      overview: Some("New series blah blah blah".to_owned()),
      genres: genres(),
      year: 2023,
      network: Some("Prime Video".to_owned()),
      runtime: 60,
      ratings: Some(rating()),
      statistics: Some(add_series_search_result_statistics()),
    }
  }

  pub fn add_series_search_result_statistics() -> AddSeriesSearchResultStatistics {
    AddSeriesSearchResultStatistics { season_count: 3 }
  }

  pub fn blocklist_item() -> BlocklistItem {
    BlocklistItem {
      id: 1,
      series_id: 1,
      series_title: None,
      episode_ids: vec![Number::from(1)],
      source_title: "Test Source Title".to_owned(),
      languages: vec![language()],
      quality: quality_wrapper(),
      date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      protocol: "usenet".to_owned(),
      indexer: "NZBgeek (Prowlarr)".to_owned(),
      message: "test message".to_owned(),
    }
  }

  pub fn download_record() -> DownloadRecord {
    DownloadRecord {
      title: "Test Download Title".to_owned(),
      status: DownloadStatus::Downloading,
      id: 1,
      episode_id: Some(Number::from(1i64)),
      size: 3543348019f64,
      sizeleft: 1771674009f64,
      output_path: Some(HorizontallyScrollableText::from(
        "/nfs/tv/Test show/season 1/",
      )),
      indexer: "kickass torrents".to_owned(),
      download_client: Some("transmission".to_owned()),
    }
  }

  pub fn downloads_response() -> DownloadsResponse {
    DownloadsResponse {
      records: vec![download_record()],
    }
  }

  pub fn episode() -> Episode {
    Episode {
      id: 1,
      series_id: 1,
      tvdb_id: 1234,
      episode_file_id: 1,
      season_number: 1,
      episode_number: 1,
      title: "Something cool".to_owned(),
      air_date_utc: Some(DateTime::from(
        DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap(),
      )),
      overview: Some("Okay so this one time at band camp...".to_owned()),
      has_file: true,
      monitored: true,
      episode_file: Some(episode_file()),
    }
  }

  pub fn episode_file() -> EpisodeFile {
    EpisodeFile {
      id: 1,
      relative_path: "/season 1/episode 1.mkv".to_owned(),
      path: "/nfs/tv/series/season 1/episode 1.mkv".to_owned(),
      size: 3543348019,
      quality: quality_wrapper(),
      languages: vec![Some(language())],
      date_added: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      media_info: Some(media_info()),
    }
  }

  pub fn genres() -> Vec<String> {
    vec!["cool".to_owned(), "family".to_owned(), "fun".to_owned()]
  }

  pub fn history_data() -> SonarrHistoryData {
    SonarrHistoryData {
      dropped_path: Some("/nfs/nzbget/completed/series/Coolness/something.cool.mkv".to_owned()),
      imported_path: Some(
        "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv".to_owned(),
      ),
      ..SonarrHistoryData::default()
    }
  }

  pub fn history_item() -> SonarrHistoryItem {
    SonarrHistoryItem {
      id: 1,
      source_title: "Test source".into(),
      episode_id: 1,
      quality: quality_wrapper(),
      languages: vec![language()],
      date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      event_type: SonarrHistoryEventType::Grabbed,
      data: history_data(),
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
      id: 1,
      minimum_age: 1,
      retention: 1,
      maximum_size: 12345,
      rss_sync_interval: 60,
    }
  }

  pub fn language() -> Language {
    Language {
      id: 1,
      name: "English".to_owned(),
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
      resolution: "1920x1080".to_owned(),
      run_time: "23:51".to_owned(),
      scan_type: "Progressive".to_owned(),
      subtitles: Some("English".to_owned()),
    }
  }
  pub fn quality() -> Quality {
    Quality {
      name: "Bluray-1080p".to_owned(),
    }
  }

  pub fn quality_wrapper() -> QualityWrapper {
    QualityWrapper { quality: quality() }
  }

  pub fn rating() -> Rating {
    Rating {
      votes: 406744,
      value: 8.4,
    }
  }

  pub fn season() -> Season {
    Season {
      title: None,
      season_number: 1,
      monitored: true,
      statistics: Some(season_statistics()),
    }
  }

  pub fn season_statistics() -> SeasonStatistics {
    SeasonStatistics {
      previous_airing: Some(DateTime::from(
        DateTime::parse_from_rfc3339("2022-10-24T01:00:00Z").unwrap(),
      )),
      next_airing: None,
      episode_file_count: 10,
      episode_count: 10,
      total_episode_count: 10,
      size_on_disk: 36708563419,
      percent_of_episodes: 100.0,
    }
  }

  pub fn series() -> Series {
    Series {
      title: "Test".to_owned().into(),
      status: SeriesStatus::Continuing,
      ended: false,
      overview: Some("Blah blah blah".to_owned()),
      network: Some("HBO".to_owned()),
      seasons: Some(vec![season()]),
      year: 2022,
      path: "/nfs/tv/Test".to_owned(),
      quality_profile_id: 6,
      language_profile_id: 1,
      season_folder: true,
      monitored: true,
      runtime: 63,
      tvdb_id: 371572,
      series_type: SeriesType::Standard,
      certification: Some("TV-MA".to_owned()),
      genres: vec!["cool".to_owned(), "family".to_owned(), "fun".to_owned()],
      tags: vec![Number::from(3)],
      ratings: rating(),
      statistics: Some(series_statistics()),
      id: 1,
    }
  }

  pub fn series_statistics() -> SeriesStatistics {
    SeriesStatistics {
      season_count: 2,
      episode_file_count: 18,
      episode_count: 18,
      total_episode_count: 50,
      size_on_disk: 63894022699,
      percent_of_episodes: 100.0,
    }
  }

  pub fn rejections() -> Vec<String> {
    vec![
      "Unknown quality profile".to_owned(),
      "Release is already mapped".to_owned(),
    ]
  }

  pub fn release() -> SonarrRelease {
    SonarrRelease {
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
      full_season: false,
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
}
