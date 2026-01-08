#[cfg(test)]
mod tests {
  mod sonarr_data_tests {
    use crate::app::sonarr::sonarr_context_clues::SERIES_HISTORY_CONTEXT_CLUES;
    use crate::models::sonarr_models::{Season, SonarrHistoryItem};
    use crate::models::stateful_table::StatefulTable;
    use crate::{
      app::{
        context_clues::{
          BLOCKLIST_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
          ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
        },
        sonarr::sonarr_context_clues::{
          HISTORY_CONTEXT_CLUES, SERIES_CONTEXT_CLUES, SERIES_DETAILS_CONTEXT_CLUES,
        },
      },
      models::{
        BlockSelectionState, Route,
        servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SonarrData},
      },
    };
    use bimap::BiMap;
    use chrono::{DateTime, Utc};
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::Number;

    #[test]
    fn test_from_active_sonarr_block_to_route() {
      assert_eq!(
        Route::from(ActiveSonarrBlock::SeriesSortPrompt),
        Route::Sonarr(ActiveSonarrBlock::SeriesSortPrompt, None)
      );
    }

    #[test]
    fn test_from_tuple_to_route_with_context() {
      assert_eq!(
        Route::from((
          ActiveSonarrBlock::SeriesSortPrompt,
          Some(ActiveSonarrBlock::Series)
        )),
        Route::Sonarr(
          ActiveSonarrBlock::SeriesSortPrompt,
          Some(ActiveSonarrBlock::Series),
        )
      );
    }

    #[test]
    fn test_reset_delete_series_preferences() {
      let mut sonarr_data = SonarrData {
        add_list_exclusion: true,
        delete_series_files: true,
        ..SonarrData::default()
      };

      sonarr_data.reset_delete_series_preferences();

      assert!(!sonarr_data.delete_series_files);
      assert!(!sonarr_data.add_list_exclusion);
    }

    #[test]
    fn test_reset_series_info_tabs() {
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      let mut sonarr_data = SonarrData {
        series_history: Some(series_history),
        ..SonarrData::default()
      };
      sonarr_data.seasons.set_items(vec![Season::default()]);
      sonarr_data.series_info_tabs.index = 1;

      sonarr_data.reset_series_info_tabs();

      assert!(sonarr_data.series_history.is_none());
      assert!(sonarr_data.seasons.is_empty());
      assert_eq!(sonarr_data.series_info_tabs.index, 0);
    }

    #[test]
    fn test_tag_ids_to_display() {
      let mut tags_map = BiMap::new();
      tags_map.insert(3, "test 3".to_owned());
      tags_map.insert(2, "test 2".to_owned());
      tags_map.insert(1, "test 1".to_owned());
      let sonarr_data = SonarrData {
        tags_map,
        ..SonarrData::default()
      };

      assert_str_eq!(
        sonarr_data.tag_ids_to_display(&[Number::from(1), Number::from(2)]),
        "test 1, test 2"
      );
    }

    #[test]
    fn test_sorted_quality_profile_names() {
      let mut quality_profile_map = BiMap::new();
      quality_profile_map.insert(3, "test 1".to_owned());
      quality_profile_map.insert(2, "test 2".to_owned());
      quality_profile_map.insert(1, "test 3".to_owned());
      let sonarr_data = SonarrData {
        quality_profile_map,
        ..SonarrData::default()
      };
      let expected_quality_profile_vec = vec![
        "test 3".to_owned(),
        "test 2".to_owned(),
        "test 1".to_owned(),
      ];

      assert_iter_eq!(
        sonarr_data.sorted_quality_profile_names(),
        expected_quality_profile_vec
      );
    }

    #[test]
    fn test_sorted_language_profile_names() {
      let mut language_profiles_map = BiMap::new();
      language_profiles_map.insert(3, "test 1".to_owned());
      language_profiles_map.insert(2, "test 2".to_owned());
      language_profiles_map.insert(1, "test 3".to_owned());
      let sonarr_data = SonarrData {
        language_profiles_map,
        ..SonarrData::default()
      };
      let expected_language_profiles_vec = vec![
        "test 3".to_owned(),
        "test 2".to_owned(),
        "test 1".to_owned(),
      ];

      assert_iter_eq!(
        sonarr_data.sorted_language_profile_names(),
        expected_language_profiles_vec
      );
    }

    #[test]
    fn test_sonarr_data_defaults() {
      let sonarr_data = SonarrData::default();

      assert!(!sonarr_data.add_list_exclusion);
      assert_none!(sonarr_data.add_searched_series);
      assert_none!(sonarr_data.add_series_search);
      assert_none!(sonarr_data.add_series_modal);
      assert_is_empty!(sonarr_data.blocklist);
      assert!(!sonarr_data.delete_series_files);
      assert_is_empty!(sonarr_data.downloads);
      assert_is_empty!(sonarr_data.disk_space_vec);
      assert_none!(sonarr_data.edit_indexer_modal);
      assert_none!(sonarr_data.edit_root_folder);
      assert_none!(sonarr_data.edit_series_modal);
      assert_is_empty!(sonarr_data.history);
      assert_is_empty!(sonarr_data.indexers);
      assert_none!(sonarr_data.indexer_settings);
      assert_none!(sonarr_data.indexer_test_errors);
      assert_none!(sonarr_data.indexer_test_all_results);
      assert_is_empty!(sonarr_data.language_profiles_map);
      assert_is_empty!(sonarr_data.logs);
      assert_is_empty!(sonarr_data.log_details);
      assert!(!sonarr_data.prompt_confirm);
      assert_none!(sonarr_data.prompt_confirm_action);
      assert_is_empty!(sonarr_data.quality_profile_map);
      assert_is_empty!(sonarr_data.queued_events);
      assert_is_empty!(sonarr_data.root_folders);
      assert_is_empty!(sonarr_data.seasons);
      assert_none!(sonarr_data.season_details_modal);
      assert_eq!(sonarr_data.selected_block, BlockSelectionState::default());
      assert_is_empty!(sonarr_data.series);
      assert_none!(sonarr_data.series_history);
      assert_eq!(sonarr_data.start_time, <DateTime<Utc>>::default());
      assert_is_empty!(sonarr_data.tags_map);
      assert_is_empty!(sonarr_data.tasks);
      assert_is_empty!(sonarr_data.updates);
      assert_is_empty!(sonarr_data.version);

      assert_eq!(sonarr_data.main_tabs.tabs.len(), 7);

      assert_str_eq!(sonarr_data.main_tabs.tabs[0].title, "Library");
      assert_eq!(
        sonarr_data.main_tabs.tabs[0].route,
        ActiveSonarrBlock::Series.into()
      );
      assert_some_eq_x!(
        &sonarr_data.main_tabs.tabs[0].contextual_help,
        &SERIES_CONTEXT_CLUES
      );
      assert_none!(sonarr_data.main_tabs.tabs[0].config);

      assert_str_eq!(sonarr_data.main_tabs.tabs[1].title, "Downloads");
      assert_eq!(
        sonarr_data.main_tabs.tabs[1].route,
        ActiveSonarrBlock::Downloads.into()
      );
      assert_some_eq_x!(
        &sonarr_data.main_tabs.tabs[1].contextual_help,
        &DOWNLOADS_CONTEXT_CLUES
      );
      assert_none!(sonarr_data.main_tabs.tabs[1].config);

      assert_str_eq!(sonarr_data.main_tabs.tabs[2].title, "Blocklist");
      assert_eq!(
        sonarr_data.main_tabs.tabs[2].route,
        ActiveSonarrBlock::Blocklist.into()
      );
      assert_some_eq_x!(
        &sonarr_data.main_tabs.tabs[2].contextual_help,
        &BLOCKLIST_CONTEXT_CLUES
      );
      assert_none!(sonarr_data.main_tabs.tabs[2].config);

      assert_str_eq!(sonarr_data.main_tabs.tabs[3].title, "History");
      assert_eq!(
        sonarr_data.main_tabs.tabs[3].route,
        ActiveSonarrBlock::History.into()
      );
      assert_some_eq_x!(
        &sonarr_data.main_tabs.tabs[3].contextual_help,
        &HISTORY_CONTEXT_CLUES
      );
      assert_none!(sonarr_data.main_tabs.tabs[3].config);

      assert_str_eq!(sonarr_data.main_tabs.tabs[4].title, "Root Folders");
      assert_eq!(
        sonarr_data.main_tabs.tabs[4].route,
        ActiveSonarrBlock::RootFolders.into()
      );
      assert_some_eq_x!(
        &sonarr_data.main_tabs.tabs[4].contextual_help,
        &ROOT_FOLDERS_CONTEXT_CLUES
      );
      assert_none!(sonarr_data.main_tabs.tabs[4].config);

      assert_str_eq!(sonarr_data.main_tabs.tabs[5].title, "Indexers");
      assert_eq!(
        sonarr_data.main_tabs.tabs[5].route,
        ActiveSonarrBlock::Indexers.into()
      );
      assert_some_eq_x!(
        &sonarr_data.main_tabs.tabs[5].contextual_help,
        &INDEXERS_CONTEXT_CLUES
      );
      assert_none!(sonarr_data.main_tabs.tabs[5].config);

      assert_str_eq!(sonarr_data.main_tabs.tabs[6].title, "System");
      assert_eq!(
        sonarr_data.main_tabs.tabs[6].route,
        ActiveSonarrBlock::System.into()
      );
      assert_some_eq_x!(
        &sonarr_data.main_tabs.tabs[6].contextual_help,
        &SYSTEM_CONTEXT_CLUES
      );
      assert_none!(sonarr_data.main_tabs.tabs[6].config);

      assert_eq!(sonarr_data.series_info_tabs.tabs.len(), 2);

      assert_str_eq!(sonarr_data.series_info_tabs.tabs[0].title, "Seasons");
      assert_eq!(
        sonarr_data.series_info_tabs.tabs[0].route,
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert_some_eq_x!(
        &sonarr_data.series_info_tabs.tabs[0].contextual_help,
        &SERIES_DETAILS_CONTEXT_CLUES
      );
      assert_none!(sonarr_data.series_info_tabs.tabs[0].config);

      assert_str_eq!(sonarr_data.series_info_tabs.tabs[1].title, "History");
      assert_eq!(
        sonarr_data.series_info_tabs.tabs[1].route,
        ActiveSonarrBlock::SeriesHistory.into()
      );
      assert_some_eq_x!(
        &sonarr_data.series_info_tabs.tabs[1].contextual_help,
        &SERIES_HISTORY_CONTEXT_CLUES
      );
      assert_none!(sonarr_data.series_info_tabs.tabs[1].config);
    }
  }

  mod active_sonarr_block_tests {
    use crate::models::servarr_data::sonarr::sonarr_data::{
      ADD_SERIES_BLOCKS, ADD_SERIES_SELECTION_BLOCKS, ActiveSonarrBlock, BLOCKLIST_BLOCKS,
      DELETE_SERIES_BLOCKS, DELETE_SERIES_SELECTION_BLOCKS, DOWNLOADS_BLOCKS, EDIT_INDEXER_BLOCKS,
      EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS, EDIT_SERIES_BLOCKS,
      EDIT_SERIES_SELECTION_BLOCKS, EPISODE_DETAILS_BLOCKS, HISTORY_BLOCKS,
      INDEXER_SETTINGS_BLOCKS, INDEXER_SETTINGS_SELECTION_BLOCKS, INDEXERS_BLOCKS, LIBRARY_BLOCKS,
      ROOT_FOLDERS_BLOCKS, SEASON_DETAILS_BLOCKS, SERIES_DETAILS_BLOCKS, SYSTEM_DETAILS_BLOCKS,
    };

    #[test]
    fn test_library_blocks_contents() {
      assert_eq!(LIBRARY_BLOCKS.len(), 7);
      assert!(LIBRARY_BLOCKS.contains(&ActiveSonarrBlock::Series));
      assert!(LIBRARY_BLOCKS.contains(&ActiveSonarrBlock::SeriesSortPrompt));
      assert!(LIBRARY_BLOCKS.contains(&ActiveSonarrBlock::SearchSeries));
      assert!(LIBRARY_BLOCKS.contains(&ActiveSonarrBlock::SearchSeriesError));
      assert!(LIBRARY_BLOCKS.contains(&ActiveSonarrBlock::FilterSeries));
      assert!(LIBRARY_BLOCKS.contains(&ActiveSonarrBlock::FilterSeriesError));
      assert!(LIBRARY_BLOCKS.contains(&ActiveSonarrBlock::UpdateAllSeriesPrompt));
    }

    #[test]
    fn test_add_series_blocks_contents() {
      assert_eq!(ADD_SERIES_BLOCKS.len(), 13);
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesAlreadyInLibrary));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesConfirmPrompt));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesEmptySearchResults));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesPrompt));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesSearchInput));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesSearchResults));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesSelectLanguageProfile));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesSelectMonitor));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesSelectQualityProfile));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesSelectRootFolder));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesSelectSeriesType));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesTagsInput));
      assert!(ADD_SERIES_BLOCKS.contains(&ActiveSonarrBlock::AddSeriesToggleUseSeasonFolder));
    }

    #[test]
    fn test_add_series_selection_blocks_ordering() {
      let mut add_series_block_iter = ADD_SERIES_SELECTION_BLOCKS.iter();

      assert_eq!(
        add_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::AddSeriesSelectRootFolder]
      );
      assert_eq!(
        add_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::AddSeriesSelectMonitor]
      );
      assert_eq!(
        add_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::AddSeriesSelectQualityProfile]
      );
      assert_eq!(
        add_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::AddSeriesSelectLanguageProfile]
      );
      assert_eq!(
        add_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::AddSeriesSelectSeriesType]
      );
      assert_eq!(
        add_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::AddSeriesToggleUseSeasonFolder]
      );
      assert_eq!(
        add_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::AddSeriesTagsInput]
      );
      assert_eq!(
        add_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::AddSeriesConfirmPrompt]
      );
      assert_eq!(add_series_block_iter.next(), None);
    }

    #[test]
    fn test_blocklist_blocks_contents() {
      assert_eq!(BLOCKLIST_BLOCKS.len(), 5);
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveSonarrBlock::Blocklist));
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveSonarrBlock::BlocklistItemDetails));
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveSonarrBlock::DeleteBlocklistItemPrompt));
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveSonarrBlock::BlocklistClearAllItemsPrompt));
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveSonarrBlock::BlocklistSortPrompt));
    }

    #[test]
    fn test_edit_movie_blocks_contents() {
      assert_eq!(EDIT_SERIES_BLOCKS.len(), 9);
      assert!(EDIT_SERIES_BLOCKS.contains(&ActiveSonarrBlock::EditSeriesPrompt));
      assert!(EDIT_SERIES_BLOCKS.contains(&ActiveSonarrBlock::EditSeriesConfirmPrompt));
      assert!(EDIT_SERIES_BLOCKS.contains(&ActiveSonarrBlock::EditSeriesPathInput));
      assert!(EDIT_SERIES_BLOCKS.contains(&ActiveSonarrBlock::EditSeriesSelectSeriesType));
      assert!(EDIT_SERIES_BLOCKS.contains(&ActiveSonarrBlock::EditSeriesSelectQualityProfile));
      assert!(EDIT_SERIES_BLOCKS.contains(&ActiveSonarrBlock::EditSeriesSelectLanguageProfile));
      assert!(EDIT_SERIES_BLOCKS.contains(&ActiveSonarrBlock::EditSeriesTagsInput));
      assert!(EDIT_SERIES_BLOCKS.contains(&ActiveSonarrBlock::EditSeriesToggleMonitored));
      assert!(EDIT_SERIES_BLOCKS.contains(&ActiveSonarrBlock::EditSeriesToggleSeasonFolder));
    }

    #[test]
    fn test_edit_series_selection_blocks_ordering() {
      let mut edit_series_block_iter = EDIT_SERIES_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::EditSeriesToggleMonitored]
      );
      assert_eq!(
        edit_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::EditSeriesToggleSeasonFolder]
      );
      assert_eq!(
        edit_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::EditSeriesSelectQualityProfile]
      );
      assert_eq!(
        edit_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::EditSeriesSelectLanguageProfile]
      );
      assert_eq!(
        edit_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::EditSeriesSelectSeriesType]
      );
      assert_eq!(
        edit_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::EditSeriesPathInput]
      );
      assert_eq!(
        edit_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::EditSeriesTagsInput]
      );
      assert_eq!(
        edit_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::EditSeriesConfirmPrompt]
      );
      assert_eq!(edit_series_block_iter.next(), None);
    }

    #[test]
    fn test_downloads_blocks_contents() {
      assert_eq!(DOWNLOADS_BLOCKS.len(), 3);
      assert!(DOWNLOADS_BLOCKS.contains(&ActiveSonarrBlock::Downloads));
      assert!(DOWNLOADS_BLOCKS.contains(&ActiveSonarrBlock::DeleteDownloadPrompt));
      assert!(DOWNLOADS_BLOCKS.contains(&ActiveSonarrBlock::UpdateDownloadsPrompt));
    }

    #[test]
    fn test_delete_series_blocks_contents() {
      assert_eq!(DELETE_SERIES_BLOCKS.len(), 4);
      assert!(DELETE_SERIES_BLOCKS.contains(&ActiveSonarrBlock::DeleteSeriesPrompt));
      assert!(DELETE_SERIES_BLOCKS.contains(&ActiveSonarrBlock::DeleteSeriesConfirmPrompt));
      assert!(DELETE_SERIES_BLOCKS.contains(&ActiveSonarrBlock::DeleteSeriesToggleDeleteFile));
      assert!(
        DELETE_SERIES_BLOCKS.contains(&ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion)
      );
    }

    #[test]
    fn test_delete_series_selection_blocks_ordering() {
      let mut delete_series_block_iter = DELETE_SERIES_SELECTION_BLOCKS.iter();

      assert_eq!(
        delete_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::DeleteSeriesToggleDeleteFile]
      );
      assert_eq!(
        delete_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion]
      );
      assert_eq!(
        delete_series_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::DeleteSeriesConfirmPrompt]
      );
      assert_eq!(delete_series_block_iter.next(), None);
    }

    #[test]
    fn test_edit_indexer_blocks_contents() {
      assert_eq!(EDIT_INDEXER_BLOCKS.len(), 11);
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerPrompt));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerConfirmPrompt));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerApiKeyInput));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerNameInput));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerSeedRatioInput));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerToggleEnableRss));
      assert!(
        EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch)
      );
      assert!(
        EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch)
      );
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerUrlInput));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerTagsInput));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveSonarrBlock::EditIndexerPriorityInput));
    }

    #[test]
    fn test_edit_indexer_nzb_selection_blocks_ordering() {
      let mut edit_indexer_nzb_selection_block_iter = EDIT_INDEXER_NZB_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerNameInput,
          ActiveSonarrBlock::EditIndexerUrlInput,
        ]
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerToggleEnableRss,
          ActiveSonarrBlock::EditIndexerApiKeyInput,
        ]
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
          ActiveSonarrBlock::EditIndexerTagsInput,
        ]
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
          ActiveSonarrBlock::EditIndexerPriorityInput,
        ]
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerConfirmPrompt,
          ActiveSonarrBlock::EditIndexerConfirmPrompt,
        ]
      );
      assert_eq!(edit_indexer_nzb_selection_block_iter.next(), None);
    }

    #[test]
    fn test_edit_indexer_torrent_selection_blocks_ordering() {
      let mut edit_indexer_torrent_selection_block_iter =
        EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerNameInput,
          ActiveSonarrBlock::EditIndexerUrlInput,
        ]
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerToggleEnableRss,
          ActiveSonarrBlock::EditIndexerApiKeyInput,
        ]
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
          ActiveSonarrBlock::EditIndexerSeedRatioInput,
        ]
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
          ActiveSonarrBlock::EditIndexerTagsInput,
        ]
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerPriorityInput,
          ActiveSonarrBlock::EditIndexerConfirmPrompt,
        ]
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &[
          ActiveSonarrBlock::EditIndexerConfirmPrompt,
          ActiveSonarrBlock::EditIndexerConfirmPrompt,
        ]
      );
      assert_eq!(edit_indexer_torrent_selection_block_iter.next(), None);
    }

    #[test]
    fn test_indexer_settings_blocks_contents() {
      assert_eq!(INDEXER_SETTINGS_BLOCKS.len(), 6);
      assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveSonarrBlock::AllIndexerSettingsPrompt));
      assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveSonarrBlock::IndexerSettingsConfirmPrompt));
      assert!(
        INDEXER_SETTINGS_BLOCKS.contains(&ActiveSonarrBlock::IndexerSettingsMaximumSizeInput)
      );
      assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveSonarrBlock::IndexerSettingsMinimumAgeInput));
      assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveSonarrBlock::IndexerSettingsRetentionInput));
      assert!(
        INDEXER_SETTINGS_BLOCKS.contains(&ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput)
      );
    }

    #[test]
    fn test_indexer_settings_selection_blocks_ordering() {
      let mut indexer_settings_block_iter = INDEXER_SETTINGS_SELECTION_BLOCKS.iter();

      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::IndexerSettingsMinimumAgeInput,]
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::IndexerSettingsRetentionInput,]
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::IndexerSettingsMaximumSizeInput,]
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput,]
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &[ActiveSonarrBlock::IndexerSettingsConfirmPrompt,]
      );
      assert_eq!(indexer_settings_block_iter.next(), None);
    }

    #[test]
    fn test_history_blocks_contents() {
      assert_eq!(HISTORY_BLOCKS.len(), 7);
      assert!(HISTORY_BLOCKS.contains(&ActiveSonarrBlock::History));
      assert!(HISTORY_BLOCKS.contains(&ActiveSonarrBlock::HistoryItemDetails));
      assert!(HISTORY_BLOCKS.contains(&ActiveSonarrBlock::HistorySortPrompt));
      assert!(HISTORY_BLOCKS.contains(&ActiveSonarrBlock::FilterHistory));
      assert!(HISTORY_BLOCKS.contains(&ActiveSonarrBlock::FilterHistoryError));
      assert!(HISTORY_BLOCKS.contains(&ActiveSonarrBlock::SearchHistory));
      assert!(HISTORY_BLOCKS.contains(&ActiveSonarrBlock::SearchHistoryError));
    }

    #[test]
    fn test_root_folders_blocks_contents() {
      assert_eq!(ROOT_FOLDERS_BLOCKS.len(), 3);
      assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveSonarrBlock::RootFolders));
      assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveSonarrBlock::AddRootFolderPrompt));
      assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveSonarrBlock::DeleteRootFolderPrompt));
    }

    #[test]
    fn test_indexers_blocks_contents() {
      assert_eq!(INDEXERS_BLOCKS.len(), 3);
      assert!(INDEXERS_BLOCKS.contains(&ActiveSonarrBlock::DeleteIndexerPrompt));
      assert!(INDEXERS_BLOCKS.contains(&ActiveSonarrBlock::Indexers));
      assert!(INDEXERS_BLOCKS.contains(&ActiveSonarrBlock::TestIndexer));
    }

    #[test]
    fn test_system_details_blocks_contents() {
      assert_eq!(SYSTEM_DETAILS_BLOCKS.len(), 5);
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SystemLogs));
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SystemQueuedEvents));
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SystemTasks));
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SystemTaskStartConfirmPrompt));
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SystemUpdates));
    }

    #[test]
    fn test_series_details_blocks_contents() {
      assert_eq!(SERIES_DETAILS_BLOCKS.len(), 12);
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SeriesDetails));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SeriesHistory));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SearchSeason));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SearchSeasonError));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::UpdateAndScanSeriesPrompt));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::AutomaticallySearchSeriesPrompt));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SearchSeriesHistory));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SearchSeriesHistoryError));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::FilterSeriesHistory));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::FilterSeriesHistoryError));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SeriesHistorySortPrompt));
      assert!(SERIES_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SeriesHistoryDetails));
    }

    #[test]
    fn test_season_details_blocks_contents() {
      assert_eq!(SEASON_DETAILS_BLOCKS.len(), 15);
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SeasonDetails));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SeasonHistory));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SearchEpisodes));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SearchEpisodesError));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::AutomaticallySearchSeasonPrompt));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SearchSeasonHistory));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SearchSeasonHistoryError));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::FilterSeasonHistory));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::FilterSeasonHistoryError));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SeasonHistorySortPrompt));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::SeasonHistoryDetails));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::ManualSeasonSearch));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::ManualSeasonSearchSortPrompt));
      assert!(SEASON_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::DeleteEpisodeFilePrompt));
    }

    #[test]
    fn test_episode_details_blocks_contents() {
      assert_eq!(EPISODE_DETAILS_BLOCKS.len(), 8);
      assert!(EPISODE_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::EpisodeDetails));
      assert!(EPISODE_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::EpisodeHistory));
      assert!(EPISODE_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::EpisodeHistoryDetails));
      assert!(EPISODE_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::EpisodeFile));
      assert!(EPISODE_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::ManualEpisodeSearch));
      assert!(EPISODE_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::ManualEpisodeSearchSortPrompt));
      assert!(
        EPISODE_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt)
      );
      assert!(
        EPISODE_DETAILS_BLOCKS.contains(&ActiveSonarrBlock::AutomaticallySearchEpisodePrompt)
      );
    }
  }
}
