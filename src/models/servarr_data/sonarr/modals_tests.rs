#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use strum::IntoEnumIterator;

  use crate::models::{
    servarr_data::sonarr::{modals::AddSeriesModal, sonarr_data::SonarrData},
    servarr_models::RootFolder,
    sonarr_models::{SeriesMonitor, SeriesType},
  };

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
}
