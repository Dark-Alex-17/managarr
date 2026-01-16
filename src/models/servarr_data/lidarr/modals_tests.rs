#[cfg(test)]
mod tests {
  use crate::app::lidarr::lidarr_context_clues::{
    ALBUM_DETAILS_CONTEXT_CLUES, ALBUM_HISTORY_CONTEXT_CLUES, MANUAL_ALBUM_SEARCH_CONTEXT_CLUES,
  };
  use crate::models::lidarr_models::{Artist, MonitorType, NewItemMonitorType};
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, LidarrData};
  use crate::models::servarr_data::lidarr::modals::{
    AddArtistModal, AlbumDetailsModal, EditArtistModal,
  };
  use crate::models::servarr_data::modals::EditIndexerModal;
  use crate::models::servarr_models::{Indexer, IndexerField, RootFolder};
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::{Number, Value};

  #[test]
  fn test_add_artist_modal_from_lidarr_data() {
    let mut lidarr_data = LidarrData {
      quality_profile_map: BiMap::from_iter([
        (2i64, "Lossless".to_owned()),
        (1i64, "Standard".to_owned()),
      ]),
      metadata_profile_map: BiMap::from_iter([
        (2i64, "None".to_owned()),
        (1i64, "Standard".to_owned()),
      ]),
      ..LidarrData::default()
    };
    let root_folder_1 = RootFolder {
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
      unmapped_folders: None,
    };
    lidarr_data.root_folders.set_items(vec![
      root_folder_1.clone(),
      RootFolder {
        id: 2,
        path: "/nfs2".to_owned(),
        accessible: true,
        free_space: 21990232555520,
        unmapped_folders: None,
      },
    ]);

    let add_artist_modal = AddArtistModal::from(&lidarr_data);

    assert_eq!(
      *add_artist_modal.monitor_list.current_selection(),
      MonitorType::default()
    );
    assert_eq!(
      *add_artist_modal.monitor_new_items_list.current_selection(),
      NewItemMonitorType::default()
    );
    assert_str_eq!(
      add_artist_modal.quality_profile_list.current_selection(),
      "Standard"
    );
    assert_str_eq!(
      add_artist_modal.metadata_profile_list.current_selection(),
      "Standard"
    );
    assert_eq!(
      add_artist_modal.root_folder_list.current_selection(),
      &root_folder_1
    );
    assert_is_empty!(add_artist_modal.tags.text);
  }

  #[test]
  fn test_edit_artist_modal_from_lidarr_data() {
    let mut lidarr_data = LidarrData {
      quality_profile_map: BiMap::from_iter([
        (1i64, "HD - 1080p".to_owned()),
        (2i64, "Any".to_owned()),
      ]),
      metadata_profile_map: BiMap::from_iter([
        (1i64, "Standard".to_owned()),
        (2i64, "None".to_owned()),
      ]),
      tags_map: BiMap::from_iter([(1i64, "usenet".to_owned())]),
      ..LidarrData::default()
    };
    let artist = Artist {
      id: 1,
      monitored: true,
      monitor_new_items: NewItemMonitorType::All,
      quality_profile_id: 1,
      metadata_profile_id: 1,
      path: "/nfs/music/test_artist".to_owned(),
      tags: vec![serde_json::Number::from(1)],
      ..Artist::default()
    };
    lidarr_data.artists.set_items(vec![artist]);

    let edit_artist_modal = EditArtistModal::from(&lidarr_data);

    assert_eq!(edit_artist_modal.monitored, Some(true));
    assert_eq!(
      *edit_artist_modal.monitor_list.current_selection(),
      NewItemMonitorType::All
    );
    assert_str_eq!(
      edit_artist_modal.quality_profile_list.current_selection(),
      "HD - 1080p"
    );
    assert_str_eq!(
      edit_artist_modal.metadata_profile_list.current_selection(),
      "Standard"
    );
    assert_str_eq!(edit_artist_modal.path.text, "/nfs/music/test_artist");
    assert_str_eq!(edit_artist_modal.tags.text, "usenet");
  }

  #[rstest]
  fn test_edit_indexer_modal_from_lidarr_data(#[values(true, false)] seed_ratio_present: bool) {
    let mut lidarr_data = LidarrData {
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      ..LidarrData::default()
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
    lidarr_data.indexers.set_items(vec![indexer]);

    let edit_indexer_modal = EditIndexerModal::from(&lidarr_data);

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
  fn test_edit_indexer_modal_from_lidarr_data_seed_ratio_value_is_none() {
    let mut lidarr_data = LidarrData {
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      ..LidarrData::default()
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
    lidarr_data.indexers.set_items(vec![indexer]);

    let edit_indexer_modal = EditIndexerModal::from(&lidarr_data);

    assert_str_eq!(edit_indexer_modal.name.text, "Test");
    assert_eq!(edit_indexer_modal.enable_rss, Some(true));
    assert_eq!(edit_indexer_modal.enable_automatic_search, Some(true));
    assert_eq!(edit_indexer_modal.enable_interactive_search, Some(true));
    assert_eq!(edit_indexer_modal.priority, 1);
    assert_str_eq!(edit_indexer_modal.url.text, "https://test.com");
    assert_str_eq!(edit_indexer_modal.api_key.text, "1234");
    assert!(edit_indexer_modal.seed_ratio.text.is_empty());
  }

  #[test]
  fn test_album_details_modal_default() {
    let album_details_modal = AlbumDetailsModal::default();

    assert!(album_details_modal.tracks.is_empty());
    // assert!(album_details_modal.track_details_modal.is_none());
    assert!(album_details_modal.track_files.is_empty());
    assert!(album_details_modal.album_releases.is_empty());
    assert!(album_details_modal.album_history.is_empty());

    assert_eq!(album_details_modal.album_details_tabs.tabs.len(), 3);

    assert_str_eq!(
      album_details_modal.album_details_tabs.tabs[0].title,
      "Tracks"
    );
    assert_eq!(
      album_details_modal.album_details_tabs.tabs[0].route,
      ActiveLidarrBlock::AlbumDetails.into()
    );
    assert!(
      album_details_modal.album_details_tabs.tabs[0]
        .contextual_help
        .is_some()
    );
    assert_eq!(
      album_details_modal.album_details_tabs.tabs[0]
        .contextual_help
        .unwrap(),
      &ALBUM_DETAILS_CONTEXT_CLUES
    );
    assert_eq!(album_details_modal.album_details_tabs.tabs[0].config, None);

    assert_str_eq!(
      album_details_modal.album_details_tabs.tabs[1].title,
      "History"
    );
    assert_eq!(
      album_details_modal.album_details_tabs.tabs[1].route,
      ActiveLidarrBlock::AlbumHistory.into()
    );
    assert!(
      album_details_modal.album_details_tabs.tabs[1]
        .contextual_help
        .is_some()
    );
    assert_eq!(
      album_details_modal.album_details_tabs.tabs[1]
        .contextual_help
        .unwrap(),
      &ALBUM_HISTORY_CONTEXT_CLUES
    );
    assert_eq!(album_details_modal.album_details_tabs.tabs[1].config, None);

    assert_str_eq!(
      album_details_modal.album_details_tabs.tabs[2].title,
      "Manual Search"
    );
    assert_eq!(
      album_details_modal.album_details_tabs.tabs[2].route,
      ActiveLidarrBlock::ManualAlbumSearch.into()
    );
    assert!(
      album_details_modal.album_details_tabs.tabs[2]
        .contextual_help
        .is_some()
    );
    assert_eq!(
      album_details_modal.album_details_tabs.tabs[2]
        .contextual_help
        .unwrap(),
      &MANUAL_ALBUM_SEARCH_CONTEXT_CLUES
    );
    assert_eq!(album_details_modal.album_details_tabs.tabs[2].config, None);
  }
}
