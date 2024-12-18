#[cfg(test)]
#[macro_use]
pub(in crate::handlers::sonarr_handlers) mod utils {
  use crate::models::servarr_models::{Indexer, IndexerField, Language, Quality, QualityWrapper, RootFolder};
  use crate::models::sonarr_models::{AddSeriesSearchResult, AddSeriesSearchResultStatistics, BlocklistItem, DownloadRecord, DownloadStatus, DownloadsResponse, Episode, EpisodeFile, IndexerSettings, MediaInfo, Rating, Season, SeasonStatistics, Series, SeriesStatistics, SeriesStatus, SeriesType, SonarrHistoryData, SonarrHistoryEventType, SonarrHistoryItem, SonarrRelease};
  use crate::models::HorizontallyScrollableText;
  use chrono::DateTime;
  use serde_json::{json, Number};

  #[macro_export]
  macro_rules! test_edit_series_key {
    ($handler:ident, $block:expr, $context:expr) => {
      let mut app = App::default();
      let mut sonarr_data = SonarrData {
        quality_profile_map: bimap::BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        language_profiles_map: bimap::BiMap::from_iter([
          (2222, "English".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        tags_map: bimap::BiMap::from_iter([(1, "test".to_owned())]),
        ..create_test_sonarr_data()
      };
      sonarr_data.series.set_items(vec![Series {
        path: "/nfs/series/Test".to_owned().into(),
        monitored: true,
        season_folder: true,
        quality_profile_id: 2222,
        language_profile_id: 2222,
        series_type: SeriesType::Anime,
        tags: vec![Number::from(1)],
        ..Series::default()
      }]);
      app.data.sonarr_data = sonarr_data;

      $handler::with(DEFAULT_KEYBINDINGS.edit.key, &mut app, $block, None).handle();

      assert_eq!(
        app.get_current_route(),
        (ActiveSonarrBlock::EditSeriesPrompt, Some($context)).into()
      );
      assert_eq!(
        app.data.sonarr_data.selected_block.get_active_block(),
        ActiveSonarrBlock::EditSeriesToggleMonitored
      );
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .series_type_list
          .items,
        Vec::from_iter(SeriesType::iter())
      );
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .series_type_list
          .current_selection(),
        &SeriesType::Anime
      );
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "HD - 1080p"
      );
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .items,
        vec!["Any".to_owned(), "English".to_owned()]
      );
      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .current_selection(),
        "English"
      );
      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "/nfs/series/Test"
      );
      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "test"
      );
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(true)
      );
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .use_season_folders,
        Some(true)
      );
      assert_eq!(
        app.data.sonarr_data.selected_block.blocks,
        $crate::models::servarr_data::sonarr::sonarr_data::EDIT_SERIES_SELECTION_BLOCKS
      );
    };
  }

  fn add_series_search_result() -> AddSeriesSearchResult {
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

  fn add_series_search_result_statistics() -> AddSeriesSearchResultStatistics {
    AddSeriesSearchResultStatistics { season_count: 3 }
  }

  fn blocklist_item() -> BlocklistItem {
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

  fn download_record() -> DownloadRecord {
    DownloadRecord {
      title: "Test Download Title".to_owned(),
      status: DownloadStatus::Downloading,
      id: 1,
      episode_id: 1,
      size: 3543348019f64,
      sizeleft: 1771674009f64,
      output_path: Some(HorizontallyScrollableText::from(
        "/nfs/tv/Test show/season 1/",
      )),
      indexer: "kickass torrents".to_owned(),
      download_client: Some("transmission".to_owned()),
    }
  }

  fn downloads_response() -> DownloadsResponse {
    DownloadsResponse {
      records: vec![download_record()],
    }
  }

  fn episode() -> Episode {
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

  fn episode_file() -> EpisodeFile {
    EpisodeFile {
      id: 1,
      relative_path: "/season 1/episode 1.mkv".to_owned(),
      path: "/nfs/tv/series/season 1/episode 1.mkv".to_owned(),
      size: 3543348019,
      quality: quality_wrapper(),
      languages: vec![language()],
      date_added: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      media_info: Some(media_info()),
    }
  }

  fn genres() -> Vec<String> {
    vec!["cool".to_owned(), "family".to_owned(), "fun".to_owned()]
  }

  fn history_data() -> SonarrHistoryData {
    SonarrHistoryData {
      dropped_path: Some("/nfs/nzbget/completed/series/Coolness/something.cool.mkv".to_owned()),
      imported_path: Some(
        "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv".to_owned(),
      ),
      ..SonarrHistoryData::default()
    }
  }

  fn history_item() -> SonarrHistoryItem {
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

  fn indexer() -> Indexer {
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

  fn indexer_settings() -> IndexerSettings {
    IndexerSettings {
      id: 1,
      minimum_age: 1,
      retention: 1,
      maximum_size: 12345,
      rss_sync_interval: 60,
    }
  }

  fn language() -> Language {
    Language {
      id: 1,
      name: "English".to_owned(),
    }
  }

  fn media_info() -> MediaInfo {
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
      resolution: "1920x1080".to_owned(),
      run_time: "23:51".to_owned(),
      scan_type: "Progressive".to_owned(),
      subtitles: Some("English".to_owned()),
    }
  }
  fn quality() -> Quality {
    Quality {
      name: "Bluray-1080p".to_owned(),
    }
  }

  fn quality_wrapper() -> QualityWrapper {
    QualityWrapper { quality: quality() }
  }

  fn rating() -> Rating {
    Rating {
      votes: 406744,
      value: 8.4,
    }
  }

  fn season() -> Season {
    Season {
      title: None,
      season_number: 1,
      monitored: true,
      statistics: season_statistics(),
    }
  }

  fn season_statistics() -> SeasonStatistics {
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

  fn series() -> Series {
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

  fn series_statistics() -> SeriesStatistics {
    SeriesStatistics {
      season_count: 2,
      episode_file_count: 18,
      episode_count: 18,
      total_episode_count: 50,
      size_on_disk: 63894022699,
      percent_of_episodes: 100.0,
    }
  }

  fn rejections() -> Vec<String> {
    vec![
      "Unknown quality profile".to_owned(),
      "Release is already mapped".to_owned(),
    ]
  }

  fn release() -> SonarrRelease {
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

  fn root_folder() -> RootFolder {
    RootFolder {
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
      unmapped_folders: None,
    }
  }
}
