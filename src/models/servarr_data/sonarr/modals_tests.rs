#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::models::servarr_models::{Indexer, IndexerField};
  use crate::models::{
    servarr_data::sonarr::{modals::AddSeriesModal, sonarr_data::SonarrData},
    servarr_models::RootFolder,
    sonarr_models::{SeriesMonitor, SeriesType},
  };
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
      ..Indexer::default()
    };
    sonarr_data.indexers.set_items(vec![indexer]);

    let edit_indexer_modal = EditIndexerModal::from(&sonarr_data);

    assert_str_eq!(edit_indexer_modal.name.text, "Test");
    assert_eq!(edit_indexer_modal.enable_rss, Some(true));
    assert_eq!(edit_indexer_modal.enable_automatic_search, Some(true));
    assert_eq!(edit_indexer_modal.enable_interactive_search, Some(true));
    assert_str_eq!(edit_indexer_modal.url.text, "https://test.com");
    assert_str_eq!(edit_indexer_modal.api_key.text, "1234");

    if seed_ratio_present {
      assert_str_eq!(edit_indexer_modal.seed_ratio.text, "1.2");
    } else {
      assert!(edit_indexer_modal.seed_ratio.text.is_empty());
    }
  }
}
