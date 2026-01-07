#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::models::lidarr_models::{Artist, NewItemMonitorType};
  use crate::models::servarr_data::lidarr::lidarr_data::LidarrData;
  use crate::models::servarr_data::lidarr::modals::EditArtistModal;

  #[test]
  fn test_edit_artist_modal_from_lidarr_data() {
    let mut lidarr_data = LidarrData {
      quality_profile_map: BiMap::from_iter([(1i64, "HD - 1080p".to_owned()), (2i64, "Any".to_owned())]),
      metadata_profile_map: BiMap::from_iter([(1i64, "Standard".to_owned()), (2i64, "None".to_owned())]),
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
}
