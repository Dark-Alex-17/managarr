use strum::IntoEnumIterator;

use super::lidarr_data::LidarrData;
use crate::models::{
  HorizontallyScrollableText,
  lidarr_models::{MonitorType, NewItemMonitorType},
  servarr_models::RootFolder,
  stateful_list::StatefulList,
};

#[cfg(test)]
#[path = "modals_tests.rs"]
mod modals_tests;

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct AddArtistModal {
  pub root_folder_list: StatefulList<RootFolder>,
  pub monitor_list: StatefulList<MonitorType>,
  pub monitor_new_items_list: StatefulList<NewItemMonitorType>,
  pub quality_profile_list: StatefulList<String>,
  pub metadata_profile_list: StatefulList<String>,
  pub tags: HorizontallyScrollableText,
}

impl From<&LidarrData<'_>> for AddArtistModal {
  fn from(lidarr_data: &LidarrData<'_>) -> AddArtistModal {
    let mut add_artist_modal = AddArtistModal::default();
    add_artist_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_artist_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_artist_modal
      .quality_profile_list
      .set_items(lidarr_data.sorted_quality_profile_names());
    add_artist_modal
      .metadata_profile_list
      .set_items(lidarr_data.sorted_metadata_profile_names());
    add_artist_modal
      .root_folder_list
      .set_items(lidarr_data.root_folders.items.to_vec());

    add_artist_modal
  }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct EditArtistModal {
  pub monitor_list: StatefulList<NewItemMonitorType>,
  pub quality_profile_list: StatefulList<String>,
  pub metadata_profile_list: StatefulList<String>,
  pub monitored: Option<bool>,
  pub path: HorizontallyScrollableText,
  pub tags: HorizontallyScrollableText,
}

impl From<&LidarrData<'_>> for EditArtistModal {
  fn from(lidarr_data: &LidarrData<'_>) -> EditArtistModal {
    let mut edit_artist_modal = EditArtistModal::default();
    let artist = lidarr_data.artists.current_selection();

    edit_artist_modal
      .monitor_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    edit_artist_modal.path = artist.path.clone().into();
    edit_artist_modal.tags = lidarr_data.tag_ids_to_display(&artist.tags).into();
    edit_artist_modal.monitored = Some(artist.monitored);

    let monitor_index = edit_artist_modal
      .monitor_list
      .items
      .iter()
      .position(|m| *m == artist.monitor_new_items);
    edit_artist_modal.monitor_list.state.select(monitor_index);

    edit_artist_modal
      .quality_profile_list
      .set_items(lidarr_data.sorted_quality_profile_names());
    let quality_profile_name = lidarr_data
      .quality_profile_map
      .get_by_left(&artist.quality_profile_id)
      .unwrap();
    let quality_profile_index = edit_artist_modal
      .quality_profile_list
      .items
      .iter()
      .position(|profile| profile == quality_profile_name);
    edit_artist_modal
      .quality_profile_list
      .state
      .select(quality_profile_index);

    edit_artist_modal
      .metadata_profile_list
      .set_items(lidarr_data.sorted_metadata_profile_names());
    let metadata_profile_name = lidarr_data
      .metadata_profile_map
      .get_by_left(&artist.metadata_profile_id)
      .unwrap();
    let metadata_profile_index = edit_artist_modal
      .metadata_profile_list
      .items
      .iter()
      .position(|profile| profile == metadata_profile_name);
    edit_artist_modal
      .metadata_profile_list
      .state
      .select(metadata_profile_index);

    edit_artist_modal
  }
}
