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
      ActiveSonarrBlock, DELETE_SERIES_BLOCKS, DELETE_SERIES_SELECTION_BLOCKS, DOWNLOADS_BLOCKS,
      EDIT_SERIES_BLOCKS, EDIT_SERIES_SELECTION_BLOCKS, LIBRARY_BLOCKS,
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
      assert!(DELETE_SERIES_BLOCKS.contains(&ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion));
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
  }
}
