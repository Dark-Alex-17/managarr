use strum::IntoEnumIterator;

use super::lidarr_data::LidarrData;
use crate::models::servarr_data::modals::EditIndexerModal;
use crate::models::servarr_models::Indexer;
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

impl From<&LidarrData<'_>> for EditIndexerModal {
  fn from(lidarr_data: &LidarrData<'_>) -> EditIndexerModal {
    let mut edit_indexer_modal = EditIndexerModal::default();
    let Indexer {
      name,
      enable_rss,
      enable_automatic_search,
      enable_interactive_search,
      tags,
      fields,
      priority,
      ..
    } = lidarr_data.indexers.current_selection();
    let seed_ratio_field_option = fields
      .as_ref()
      .expect("indexer fields must exist")
      .iter()
      .find(|field| {
        field.name.as_ref().expect("indexer field name must exist") == "seedCriteria.seedRatio"
      });
    let seed_ratio_value_option = if let Some(seed_ratio_field) = seed_ratio_field_option {
      seed_ratio_field.value.clone()
    } else {
      None
    };

    edit_indexer_modal.name = name.clone().expect("indexer name must exist").into();
    edit_indexer_modal.enable_rss = Some(*enable_rss);
    edit_indexer_modal.enable_automatic_search = Some(*enable_automatic_search);
    edit_indexer_modal.enable_interactive_search = Some(*enable_interactive_search);
    edit_indexer_modal.priority = *priority;
    edit_indexer_modal.url = fields
      .as_ref()
      .expect("indexer fields must exist")
      .iter()
      .find(|field| field.name.as_ref().expect("indexer field name must exist") == "baseUrl")
      .expect("baseUrl field must exist")
      .value
      .clone()
      .expect("baseUrl field value must exist")
      .as_str()
      .expect("baseUrl field value must be a string")
      .into();
    edit_indexer_modal.api_key = fields
      .as_ref()
      .expect("indexer fields must exist")
      .iter()
      .find(|field| field.name.as_ref().expect("indexer field name must exist") == "apiKey")
      .expect("apiKey field must exist")
      .value
      .clone()
      .expect("apiKey field value must exist")
      .as_str()
      .expect("apiKey field value must be a string")
      .into();

    if let Some(seed_ratio_value) = seed_ratio_value_option {
      edit_indexer_modal.seed_ratio = seed_ratio_value
        .as_f64()
        .expect("Seed ratio value must be a valid f64")
        .to_string()
        .into();
    }

    edit_indexer_modal.tags = lidarr_data.tag_ids_to_display(tags).into();

    edit_indexer_modal
  }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct AddRootFolderModal {
  pub name: HorizontallyScrollableText,
  pub path: HorizontallyScrollableText,
  pub monitor_list: StatefulList<MonitorType>,
  pub monitor_new_items_list: StatefulList<NewItemMonitorType>,
  pub quality_profile_list: StatefulList<String>,
  pub metadata_profile_list: StatefulList<String>,
  pub tags: HorizontallyScrollableText,
}

impl From<&LidarrData<'_>> for AddRootFolderModal {
  fn from(lidarr_data: &LidarrData<'_>) -> AddRootFolderModal {
    let mut add_root_folder_modal = AddRootFolderModal::default();
    add_root_folder_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_root_folder_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_root_folder_modal
      .quality_profile_list
      .set_items(lidarr_data.sorted_quality_profile_names());
    add_root_folder_modal
      .metadata_profile_list
      .set_items(lidarr_data.sorted_metadata_profile_names());

    add_root_folder_modal
  }
}
