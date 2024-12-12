#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::context_clues::build_context_clue_string;
  use crate::app::sonarr::sonarr_context_clues::{
    DETAILS_CONTEXTUAL_CONTEXT_CLUES, EPISODE_DETAILS_CONTEXT_CLUES, EPISODE_HISTORY_CONTEXT_CLUES,
    MANUAL_EPISODE_SEARCH_CONTEXT_CLUES, MANUAL_SEASON_SEARCH_CONTEXT_CLUES,
    SEASON_DETAILS_CONTEXTUAL_CONTEXT_CLUES, SEASON_DETAILS_CONTEXT_CLUES,
    SEASON_HISTORY_CONTEXT_CLUES,
  };
  use crate::models::servarr_data::sonarr::modals::{
    EditSeriesModal, EpisodeDetailsModal, SeasonDetailsModal,
  };
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::servarr_models::{Indexer, IndexerField};
  use crate::models::{
    servarr_data::sonarr::{modals::AddSeriesModal, sonarr_data::SonarrData},
    servarr_models::RootFolder,
    sonarr_models::{SeriesMonitor, SeriesType},
  };
  use crate::models::{sonarr_models::Series, stateful_table::StatefulTable};
  use serde_json::{Number, Value};

  use crate::models::servarr_data::modals::EditIndexerModal;

  #[test]
  fn test_add_series_modal_from_sonarr_data() {
    let root_folder = RootFolder {
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
      unmapped_folders: None,
    };
    let mut sonarr_data = SonarrData {
      quality_profile_map: BiMap::from_iter([
        (2222, "HD - 1080p".to_owned()),
        (1111, "Any".to_owned()),
      ]),
      language_profiles_map: BiMap::from_iter([
        (2222, "English".to_owned()),
        (1111, "Any".to_owned()),
      ]),
      ..SonarrData::default()
    };
    sonarr_data
      .root_folders
      .set_items(vec![root_folder.clone()]);

    let add_series_modal = AddSeriesModal::from(&sonarr_data);

    assert_eq!(
      add_series_modal.monitor_list.items,
      Vec::from_iter(SeriesMonitor::iter())
    );
    assert_eq!(
      add_series_modal.series_type_list.items,
      Vec::from_iter(SeriesType::iter())
    );
    assert_eq!(
      add_series_modal.quality_profile_list.items,
      vec!["Any".to_owned(), "HD - 1080p".to_owned()]
    );
    assert_eq!(
      add_series_modal.language_profile_list.items,
      vec!["Any".to_owned(), "English".to_owned()]
    );
    assert_eq!(add_series_modal.root_folder_list.items, vec![root_folder]);
    assert!(add_series_modal.tags.text.is_empty());
    assert!(add_series_modal.use_season_folder);
  }

  #[rstest]
  fn test_edit_indexer_modal_from_sonarr_data(#[values(true, false)] seed_ratio_present: bool) {
    let mut sonarr_data = SonarrData {
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      ..SonarrData::default()
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
      priority: 1,
      ..Indexer::default()
    };
    sonarr_data.indexers.set_items(vec![indexer]);

    let edit_indexer_modal = EditIndexerModal::from(&sonarr_data);

    assert_str_eq!(edit_indexer_modal.name.text, "Test");
    assert_eq!(edit_indexer_modal.enable_rss, Some(true));
    assert_eq!(edit_indexer_modal.enable_automatic_search, Some(true));
    assert_eq!(edit_indexer_modal.enable_interactive_search, Some(true));
    assert_eq!(edit_indexer_modal.priority, 1);
    assert_str_eq!(edit_indexer_modal.url.text, "https://test.com");
    assert_str_eq!(edit_indexer_modal.api_key.text, "1234");

    if seed_ratio_present {
      assert_str_eq!(edit_indexer_modal.seed_ratio.text, "1.2");
    } else {
      assert!(edit_indexer_modal.seed_ratio.text.is_empty());
    }
  }

  #[test]
  fn test_edit_indexer_modal_from_sonarr_data_seed_ratio_value_is_none() {
    let mut sonarr_data = SonarrData {
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      ..SonarrData::default()
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
      priority: 1,
      ..Indexer::default()
    };
    sonarr_data.indexers.set_items(vec![indexer]);

    let edit_indexer_modal = EditIndexerModal::from(&sonarr_data);

    assert_str_eq!(edit_indexer_modal.name.text, "Test");
    assert_eq!(edit_indexer_modal.enable_rss, Some(true));
    assert_eq!(edit_indexer_modal.enable_automatic_search, Some(true));
    assert_eq!(edit_indexer_modal.enable_interactive_search, Some(true));
    assert_eq!(edit_indexer_modal.priority, 1);
    assert_str_eq!(edit_indexer_modal.url.text, "https://test.com");
    assert_str_eq!(edit_indexer_modal.api_key.text, "1234");
    assert!(edit_indexer_modal.seed_ratio.text.is_empty());
  }

  #[rstest]
  fn test_edit_series_modal_from_sonarr_data(#[values(true, false)] test_filtered_series: bool) {
    let mut sonarr_data = SonarrData {
      quality_profile_map: BiMap::from_iter([
        (2222, "HD - 1080p".to_owned()),
        (1111, "Any".to_owned()),
      ]),
      language_profiles_map: BiMap::from_iter([
        (2222, "English".to_owned()),
        (1111, "Any".to_owned()),
      ]),
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      series: StatefulTable::default(),
      ..SonarrData::default()
    };
    let series = Series {
      path: "/nfs/seriess/Test".to_owned(),
      monitored: true,
      season_folder: true,
      quality_profile_id: 2222,
      language_profile_id: 2222,
      series_type: SeriesType::Anime,
      tags: vec![Number::from(1), Number::from(2)],
      ..Series::default()
    };

    if test_filtered_series {
      sonarr_data.series.set_filtered_items(vec![series]);
    } else {
      sonarr_data.series.set_items(vec![series]);
    }

    let edit_series_modal = EditSeriesModal::from(&sonarr_data);

    assert_eq!(
      edit_series_modal.series_type_list.items,
      Vec::from_iter(SeriesType::iter())
    );
    assert_eq!(
      edit_series_modal.series_type_list.current_selection(),
      &SeriesType::Anime,
    );
    assert_eq!(
      edit_series_modal.quality_profile_list.items,
      vec!["Any".to_owned(), "HD - 1080p".to_owned()]
    );
    assert_str_eq!(
      edit_series_modal.quality_profile_list.current_selection(),
      "HD - 1080p"
    );
    assert_eq!(
      edit_series_modal.language_profile_list.items,
      vec!["Any".to_owned(), "English".to_owned()]
    );
    assert_str_eq!(
      edit_series_modal.language_profile_list.current_selection(),
      "English"
    );
    assert_str_eq!(edit_series_modal.path.text, "/nfs/seriess/Test");
    assert_str_eq!(edit_series_modal.tags.text, "usenet, test");
    assert_eq!(edit_series_modal.monitored, Some(true));
    assert_eq!(edit_series_modal.use_season_folders, Some(true));
  }

  #[test]
  fn test_episode_details_modal_default() {
    let episode_details_modal = EpisodeDetailsModal::default();

    assert!(episode_details_modal.episode_details.is_empty());
    assert!(episode_details_modal.file_details.is_empty());
    assert!(episode_details_modal.audio_details.is_empty());
    assert!(episode_details_modal.video_details.is_empty());
    assert!(episode_details_modal.episode_history.is_empty());
    assert!(episode_details_modal.episode_releases.is_empty());

    assert_eq!(episode_details_modal.episode_details_tabs.tabs.len(), 4);

    assert_str_eq!(
      episode_details_modal.episode_details_tabs.tabs[0].title,
      "Details"
    );
    assert_eq!(
      episode_details_modal.episode_details_tabs.tabs[0].route,
      ActiveSonarrBlock::EpisodeDetails.into()
    );
    assert_str_eq!(
      episode_details_modal.episode_details_tabs.tabs[0].help,
      build_context_clue_string(&EPISODE_DETAILS_CONTEXT_CLUES)
    );
    assert!(episode_details_modal.episode_details_tabs.tabs[0]
      .contextual_help
      .is_none());

    assert_str_eq!(
      episode_details_modal.episode_details_tabs.tabs[1].title,
      "History"
    );
    assert_eq!(
      episode_details_modal.episode_details_tabs.tabs[1].route,
      ActiveSonarrBlock::EpisodeHistory.into()
    );
    assert_str_eq!(
      episode_details_modal.episode_details_tabs.tabs[1].help,
      build_context_clue_string(&EPISODE_HISTORY_CONTEXT_CLUES)
    );
    assert_eq!(
      episode_details_modal.episode_details_tabs.tabs[1].contextual_help,
      Some(build_context_clue_string(&DETAILS_CONTEXTUAL_CONTEXT_CLUES))
    );

    assert_str_eq!(
      episode_details_modal.episode_details_tabs.tabs[2].title,
      "File"
    );
    assert_eq!(
      episode_details_modal.episode_details_tabs.tabs[2].route,
      ActiveSonarrBlock::EpisodeFile.into()
    );
    assert_str_eq!(
      episode_details_modal.episode_details_tabs.tabs[2].help,
      build_context_clue_string(&EPISODE_DETAILS_CONTEXT_CLUES)
    );
    assert!(episode_details_modal.episode_details_tabs.tabs[2]
      .contextual_help
      .is_none());

    assert_str_eq!(
      episode_details_modal.episode_details_tabs.tabs[3].title,
      "Manual Search"
    );
    assert_eq!(
      episode_details_modal.episode_details_tabs.tabs[3].route,
      ActiveSonarrBlock::ManualEpisodeSearch.into()
    );
    assert_str_eq!(
      episode_details_modal.episode_details_tabs.tabs[3].help,
      build_context_clue_string(&MANUAL_EPISODE_SEARCH_CONTEXT_CLUES)
    );
    assert_eq!(
      episode_details_modal.episode_details_tabs.tabs[3].contextual_help,
      Some(build_context_clue_string(&DETAILS_CONTEXTUAL_CONTEXT_CLUES))
    );
  }

  #[test]
  fn test_season_details_modal_default() {
    let season_details_modal = SeasonDetailsModal::default();

    assert!(season_details_modal.episodes.is_empty());
    assert!(season_details_modal.episode_details_modal.is_none());
    assert!(season_details_modal.episode_files.is_empty());
    assert!(season_details_modal.season_releases.is_empty());
    assert!(season_details_modal.season_history.is_empty());

    assert_eq!(season_details_modal.season_details_tabs.tabs.len(), 3);

    assert_str_eq!(
      season_details_modal.season_details_tabs.tabs[0].title,
      "Episodes"
    );
    assert_eq!(
      season_details_modal.season_details_tabs.tabs[0].route,
      ActiveSonarrBlock::SeasonDetails.into()
    );
    assert_str_eq!(
      season_details_modal.season_details_tabs.tabs[0].help,
      build_context_clue_string(&SEASON_DETAILS_CONTEXT_CLUES)
    );
    assert_eq!(
      season_details_modal.season_details_tabs.tabs[0].contextual_help,
      Some(build_context_clue_string(&SEASON_DETAILS_CONTEXTUAL_CONTEXT_CLUES))
    );

    assert_str_eq!(
      season_details_modal.season_details_tabs.tabs[1].title,
      "History"
    );
    assert_eq!(
      season_details_modal.season_details_tabs.tabs[1].route,
      ActiveSonarrBlock::SeasonHistory.into()
    );
    assert_str_eq!(
      season_details_modal.season_details_tabs.tabs[1].help,
      build_context_clue_string(&SEASON_HISTORY_CONTEXT_CLUES)
    );
    assert_eq!(
      season_details_modal.season_details_tabs.tabs[1].contextual_help,
      Some(build_context_clue_string(&DETAILS_CONTEXTUAL_CONTEXT_CLUES))
    );

    assert_str_eq!(
      season_details_modal.season_details_tabs.tabs[2].title,
      "Manual Search"
    );
    assert_eq!(
      season_details_modal.season_details_tabs.tabs[2].route,
      ActiveSonarrBlock::ManualSeasonSearch.into()
    );
    assert_str_eq!(
      season_details_modal.season_details_tabs.tabs[2].help,
      build_context_clue_string(&MANUAL_SEASON_SEARCH_CONTEXT_CLUES)
    );
    assert_eq!(
      season_details_modal.season_details_tabs.tabs[2].contextual_help,
      Some(build_context_clue_string(
        &DETAILS_CONTEXTUAL_CONTEXT_CLUES
      ))
    );
  }
}
