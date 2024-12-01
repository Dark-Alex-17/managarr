#[cfg(test)]
#[macro_use]
mod utils {

  #[macro_export]
  macro_rules! test_edit_series_key {
    ($handler:ident, $block:expr, $context:expr) => {
      let mut app = App::default();
      let mut sonarr_data = SonarrData {
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        language_profiles_map: BiMap::from_iter([
          (2222, "English".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        tags_map: BiMap::from_iter([(1, "test".to_owned())]),
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
}
