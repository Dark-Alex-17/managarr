#[cfg(test)]
mod tests {
  mod sonarr_data_tests {
    use chrono::{DateTime, Utc};

    use crate::models::{
      servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SonarrData},
      Route,
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
    fn test_sonarr_data_defaults() {
      let sonarr_data = SonarrData::default();

      assert!(sonarr_data.blocklist.is_empty());
      assert!(sonarr_data.downloads.is_empty());
      assert!(sonarr_data.disk_space_vec.is_empty());
      assert!(sonarr_data.edit_root_folder.is_none());
      assert!(sonarr_data.history.is_empty());
      assert!(sonarr_data.indexers.is_empty());
      assert!(sonarr_data.indexer_settings.is_none());
      assert!(sonarr_data.logs.is_empty());
      assert!(sonarr_data.quality_profile_map.is_empty());
      assert!(sonarr_data.queued_events.is_empty());
      assert!(sonarr_data.root_folders.is_empty());
      assert!(sonarr_data.seasons.is_empty());
      assert!(sonarr_data.season_details_modal.is_none());
      assert!(sonarr_data.series.is_empty());
      assert!(sonarr_data.series_history.is_none());
      assert_eq!(sonarr_data.start_time, <DateTime<Utc>>::default());
      assert!(sonarr_data.tags_map.is_empty());
      assert!(sonarr_data.version.is_empty());
    }
  }
}
