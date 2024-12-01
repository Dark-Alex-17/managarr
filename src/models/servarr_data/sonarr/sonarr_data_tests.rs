#[cfg(test)]
mod tests {
  mod sonarr_data_tests {
    use chrono::{DateTime, Utc};
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::{
      app::{
        context_clues::{
          build_context_clue_string, BLOCKLIST_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES,
          INDEXERS_CONTEXT_CLUES, ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
        },
        sonarr::sonarr_context_clues::{
          HISTORY_CONTEXT_CLUES, SERIES_CONTEXT_CLUES, SERIES_DETAILS_CONTEXT_CLUES,
        },
      },
      models::{
        servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SonarrData},
        BlockSelectionState, Route,
      },
    };

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
    fn test_sonarr_data_defaults() {
      let sonarr_data = SonarrData::default();

      assert!(!sonarr_data.add_list_exclusion);
      assert!(sonarr_data.add_searched_series.is_none());
      assert!(sonarr_data.add_series_search.is_none());
      assert!(sonarr_data.add_series_modal.is_none());
      assert!(sonarr_data.blocklist.is_empty());
      assert!(!sonarr_data.delete_series_files);
      assert!(sonarr_data.downloads.is_empty());
      assert!(sonarr_data.disk_space_vec.is_empty());
      assert!(sonarr_data.edit_indexer_modal.is_none());
      assert!(sonarr_data.edit_root_folder.is_none());
      assert!(sonarr_data.edit_series_modal.is_none());
      assert!(sonarr_data.history.is_empty());
      assert!(sonarr_data.indexers.is_empty());
      assert!(sonarr_data.indexer_settings.is_none());
      assert!(sonarr_data.indexer_test_error.is_none());
      assert!(sonarr_data.indexer_test_all_results.is_none());
      assert!(sonarr_data.language_profiles_map.is_empty());
      assert!(sonarr_data.logs.is_empty());
      assert!(sonarr_data.log_details.is_empty());
      assert!(!sonarr_data.prompt_confirm);
      assert!(sonarr_data.prompt_confirm_action.is_none());
      assert!(sonarr_data.quality_profile_map.is_empty());
      assert!(sonarr_data.queued_events.is_empty());
      assert!(sonarr_data.root_folders.is_empty());
      assert!(sonarr_data.seasons.is_empty());
      assert!(sonarr_data.season_details_modal.is_none());
      assert_eq!(sonarr_data.selected_block, BlockSelectionState::default());
      assert!(sonarr_data.series.is_empty());
      assert!(sonarr_data.series_history.is_none());
      assert_eq!(sonarr_data.start_time, <DateTime<Utc>>::default());
      assert!(sonarr_data.tags_map.is_empty());
      assert!(sonarr_data.tasks.is_empty());
      assert!(sonarr_data.updates.is_empty());
      assert!(sonarr_data.version.is_empty());

      assert_eq!(sonarr_data.main_tabs.tabs.len(), 7);

      assert_str_eq!(sonarr_data.main_tabs.tabs[0].title, "Library");
      assert_eq!(
        sonarr_data.main_tabs.tabs[0].route,
        ActiveSonarrBlock::Series.into()
      );
      assert!(sonarr_data.main_tabs.tabs[0].help.is_empty());
      assert_eq!(
        sonarr_data.main_tabs.tabs[0].contextual_help,
        Some(build_context_clue_string(&SERIES_CONTEXT_CLUES))
      );

      assert_str_eq!(sonarr_data.main_tabs.tabs[1].title, "Downloads");
      assert_eq!(
        sonarr_data.main_tabs.tabs[1].route,
        ActiveSonarrBlock::Downloads.into()
      );
      assert!(sonarr_data.main_tabs.tabs[1].help.is_empty());
      assert_eq!(
        sonarr_data.main_tabs.tabs[1].contextual_help,
        Some(build_context_clue_string(&DOWNLOADS_CONTEXT_CLUES))
      );

      assert_str_eq!(sonarr_data.main_tabs.tabs[2].title, "Blocklist");
      assert_eq!(
        sonarr_data.main_tabs.tabs[2].route,
        ActiveSonarrBlock::Blocklist.into()
      );
      assert!(sonarr_data.main_tabs.tabs[2].help.is_empty());
      assert_eq!(
        sonarr_data.main_tabs.tabs[2].contextual_help,
        Some(build_context_clue_string(&BLOCKLIST_CONTEXT_CLUES))
      );

      assert_str_eq!(sonarr_data.main_tabs.tabs[3].title, "History");
      assert_eq!(
        sonarr_data.main_tabs.tabs[3].route,
        ActiveSonarrBlock::History.into()
      );
      assert!(sonarr_data.main_tabs.tabs[3].help.is_empty());
      assert_eq!(
        sonarr_data.main_tabs.tabs[3].contextual_help,
        Some(build_context_clue_string(&HISTORY_CONTEXT_CLUES))
      );

      assert_str_eq!(sonarr_data.main_tabs.tabs[4].title, "Root Folders");
      assert_eq!(
        sonarr_data.main_tabs.tabs[4].route,
        ActiveSonarrBlock::RootFolders.into()
      );
      assert!(sonarr_data.main_tabs.tabs[4].help.is_empty());
      assert_eq!(
        sonarr_data.main_tabs.tabs[4].contextual_help,
        Some(build_context_clue_string(&ROOT_FOLDERS_CONTEXT_CLUES))
      );

      assert_str_eq!(sonarr_data.main_tabs.tabs[5].title, "Indexers");
      assert_eq!(
        sonarr_data.main_tabs.tabs[5].route,
        ActiveSonarrBlock::Indexers.into()
      );
      assert!(sonarr_data.main_tabs.tabs[5].help.is_empty());
      assert_eq!(
        sonarr_data.main_tabs.tabs[5].contextual_help,
        Some(build_context_clue_string(&INDEXERS_CONTEXT_CLUES))
      );

      assert_str_eq!(sonarr_data.main_tabs.tabs[6].title, "System");
      assert_eq!(
        sonarr_data.main_tabs.tabs[6].route,
        ActiveSonarrBlock::System.into()
      );
      assert!(sonarr_data.main_tabs.tabs[6].help.is_empty());
      assert_eq!(
        sonarr_data.main_tabs.tabs[6].contextual_help,
        Some(build_context_clue_string(&SYSTEM_CONTEXT_CLUES))
      );

      assert_eq!(sonarr_data.series_info_tabs.tabs.len(), 2);

      assert_str_eq!(sonarr_data.series_info_tabs.tabs[0].title, "Seasons");
      assert_eq!(
        sonarr_data.series_info_tabs.tabs[0].route,
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert!(sonarr_data.series_info_tabs.tabs[0].help.is_empty());
      assert_eq!(
        sonarr_data.series_info_tabs.tabs[0].contextual_help,
        Some(build_context_clue_string(&SERIES_DETAILS_CONTEXT_CLUES))
      );

      assert_str_eq!(sonarr_data.series_info_tabs.tabs[1].title, "History");
      assert_eq!(
        sonarr_data.series_info_tabs.tabs[1].route,
        ActiveSonarrBlock::SeriesHistory.into()
      );
      assert!(sonarr_data.series_info_tabs.tabs[1].help.is_empty());
      assert_eq!(
        sonarr_data.series_info_tabs.tabs[1].contextual_help,
        Some(build_context_clue_string(&HISTORY_CONTEXT_CLUES))
      );
    }
  }

  mod active_sonarr_block_tests {
    use crate::models::servarr_data::sonarr::sonarr_data::{
      ActiveSonarrBlock, DOWNLOADS_BLOCKS, SERIES_BLOCKS,
    };

    #[test]
    fn test_series_blocks_contents() {
      assert_eq!(SERIES_BLOCKS.len(), 7);
      assert!(SERIES_BLOCKS.contains(&ActiveSonarrBlock::Series));
      assert!(SERIES_BLOCKS.contains(&ActiveSonarrBlock::SeriesSortPrompt));
      assert!(SERIES_BLOCKS.contains(&ActiveSonarrBlock::SearchSeries));
      assert!(SERIES_BLOCKS.contains(&ActiveSonarrBlock::SearchSeriesError));
      assert!(SERIES_BLOCKS.contains(&ActiveSonarrBlock::FilterSeries));
      assert!(SERIES_BLOCKS.contains(&ActiveSonarrBlock::FilterSeriesError));
      assert!(SERIES_BLOCKS.contains(&ActiveSonarrBlock::UpdateAllSeriesPrompt));
    }

    #[test]
    fn test_downloads_blocks_contents() {
      assert_eq!(DOWNLOADS_BLOCKS.len(), 3);
      assert!(DOWNLOADS_BLOCKS.contains(&ActiveSonarrBlock::Downloads));
      assert!(DOWNLOADS_BLOCKS.contains(&ActiveSonarrBlock::DeleteDownloadPrompt));
      assert!(DOWNLOADS_BLOCKS.contains(&ActiveSonarrBlock::UpdateDownloadsPrompt));
    }
  }
}
