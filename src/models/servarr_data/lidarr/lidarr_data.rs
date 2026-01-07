use serde_json::Number;

use crate::app::lidarr::lidarr_context_clues::ARTISTS_CONTEXT_CLUES;
use crate::models::{
  BlockSelectionState, Route, TabRoute, TabState,
  lidarr_models::{Artist, DownloadRecord},
  servarr_models::{DiskSpace, RootFolder},
  stateful_table::StatefulTable,
};
use crate::network::lidarr_network::LidarrEvent;
use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::{EnumIter};
use super::modals::EditArtistModal;
#[cfg(test)]
use {
  strum::{Display, EnumString, IntoEnumIterator},
  crate::models::lidarr_models::NewItemMonitorType,
  crate::models::stateful_table::SortOption,
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::quality_profile_map,
  crate::network::servarr_test_utils::diskspace,
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{download_record, metadata_profile, metadata_profile_map, quality_profile, root_folder, tags_map},
};

#[cfg(test)]
#[path = "lidarr_data_tests.rs"]
mod lidarr_data_tests;

pub struct LidarrData<'a> {
  pub add_import_list_exclusion: bool,
  pub artists: StatefulTable<Artist>,
  pub delete_artist_files: bool,
  pub disk_space_vec: Vec<DiskSpace>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub edit_artist_modal: Option<EditArtistModal>,
  pub main_tabs: TabState,
  pub metadata_profile_map: BiMap<i64, String>,
  pub prompt_confirm: bool,
  pub prompt_confirm_action: Option<LidarrEvent>,
  pub quality_profile_map: BiMap<i64, String>,
  pub root_folders: StatefulTable<RootFolder>,
  pub selected_block: BlockSelectionState<'a, ActiveLidarrBlock>,
  pub start_time: DateTime<Utc>,
  pub tags_map: BiMap<i64, String>,
  pub version: String,
}

impl LidarrData<'_> {
  pub fn reset_delete_artist_preferences(&mut self) {
    self.delete_artist_files = false;
    self.add_import_list_exclusion = false;
  }

  pub fn tag_ids_to_display(&self, tag_ids: &[Number]) -> String {
    tag_ids
      .iter()
      .filter_map(|id| {
        let id = id.as_i64()?;
        self.tags_map.get_by_left(&id).cloned()
      })
      .collect::<Vec<String>>()
      .join(", ")
  }

  pub fn sorted_quality_profile_names(&self) -> Vec<String> {
    let mut quality_profile_names: Vec<String> =
      self.quality_profile_map.right_values().cloned().collect();
    quality_profile_names.sort();
    quality_profile_names
  }

  pub fn sorted_metadata_profile_names(&self) -> Vec<String> {
    let mut metadata_profile_names: Vec<String> =
      self.metadata_profile_map.right_values().cloned().collect();
    metadata_profile_names.sort();
    metadata_profile_names
  }
}

impl<'a> Default for LidarrData<'a> {
  fn default() -> LidarrData<'a> {
    LidarrData {
      add_import_list_exclusion: false,
      artists: StatefulTable::default(),
      delete_artist_files: false,
      disk_space_vec: Vec::new(),
      downloads: StatefulTable::default(),
      edit_artist_modal: None,
      metadata_profile_map: BiMap::new(),
      prompt_confirm: false,
      prompt_confirm_action: None,
      quality_profile_map: BiMap::new(),
      root_folders: StatefulTable::default(),
      selected_block: BlockSelectionState::default(),
      start_time: DateTime::default(),
      tags_map: BiMap::new(),
      version: String::new(),
      main_tabs: TabState::new(vec![TabRoute {
        title: "Library".to_string(),
        route: ActiveLidarrBlock::Artists.into(),
        contextual_help: Some(&ARTISTS_CONTEXT_CLUES),
        config: None,
      }]),
    }
  }
}

#[cfg(test)]
impl LidarrData<'_> {
  pub fn test_default_fully_populated() -> Self {
    let mut edit_artist_modal = EditArtistModal {
      monitored: Some(true),
      path: "/nfs/music".into(),
      tags: "alex".into(),
      ..EditArtistModal::default()
    };
    edit_artist_modal.monitor_list.set_items(NewItemMonitorType::iter().collect());
    edit_artist_modal.quality_profile_list.set_items(vec![quality_profile().name]);
    edit_artist_modal.metadata_profile_list.set_items(vec![metadata_profile().name]);

    let mut lidarr_data = LidarrData {
      delete_artist_files: true,
      disk_space_vec: vec![diskspace()],
      quality_profile_map: quality_profile_map(),
      metadata_profile_map: metadata_profile_map(),
      edit_artist_modal: Some(edit_artist_modal),
      tags_map: tags_map(),
      ..LidarrData::default()
    };
    lidarr_data.artists.set_items(vec![Artist::default()]);
    lidarr_data.artists.sorting(vec![SortOption {
      name: "Name",
      cmp_fn: Some(|a: &Artist, b: &Artist| a.artist_name.text.cmp(&b.artist_name.text)),
    }]);
    lidarr_data.artists.search = Some("artist search".into());
    lidarr_data.artists.filter = Some("artist filter".into());
    lidarr_data
      .downloads
      .set_items(vec![download_record()]);
    lidarr_data
      .root_folders
      .set_items(vec![root_folder()]);
    lidarr_data.version = "1.0.0".to_owned();

    lidarr_data
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
#[cfg_attr(test, derive(Display, EnumString))]
pub enum ActiveLidarrBlock {
  #[default]
  Artists,
  ArtistsSortPrompt,
  DeleteArtistPrompt,
  DeleteArtistConfirmPrompt,
  DeleteArtistToggleDeleteFile,
  DeleteArtistToggleAddListExclusion,
  EditArtistPrompt,
  EditArtistConfirmPrompt,
  EditArtistPathInput,
  EditArtistSelectMetadataProfile,
  EditArtistSelectMonitorNewItems,
  EditArtistSelectQualityProfile,
  EditArtistTagsInput,
  EditArtistToggleMonitored,
  FilterArtists,
  FilterArtistsError,
  SearchArtists,
  SearchArtistsError,
  UpdateAllArtistsPrompt,
}

pub static LIBRARY_BLOCKS: [ActiveLidarrBlock; 7] = [
  ActiveLidarrBlock::Artists,
  ActiveLidarrBlock::ArtistsSortPrompt,
  ActiveLidarrBlock::FilterArtists,
  ActiveLidarrBlock::FilterArtistsError,
  ActiveLidarrBlock::SearchArtists,
  ActiveLidarrBlock::SearchArtistsError,
  ActiveLidarrBlock::UpdateAllArtistsPrompt,
];

pub static DELETE_ARTIST_BLOCKS: [ActiveLidarrBlock; 4] = [
  ActiveLidarrBlock::DeleteArtistPrompt,
  ActiveLidarrBlock::DeleteArtistConfirmPrompt,
  ActiveLidarrBlock::DeleteArtistToggleDeleteFile,
  ActiveLidarrBlock::DeleteArtistToggleAddListExclusion,
];

pub const DELETE_ARTIST_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::DeleteArtistToggleDeleteFile],
  &[ActiveLidarrBlock::DeleteArtistToggleAddListExclusion],
  &[ActiveLidarrBlock::DeleteArtistConfirmPrompt],
];

pub static EDIT_ARTIST_BLOCKS: [ActiveLidarrBlock; 8] = [
  ActiveLidarrBlock::EditArtistPrompt,
  ActiveLidarrBlock::EditArtistConfirmPrompt,
  ActiveLidarrBlock::EditArtistPathInput,
  ActiveLidarrBlock::EditArtistSelectMetadataProfile,
  ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
  ActiveLidarrBlock::EditArtistSelectQualityProfile,
  ActiveLidarrBlock::EditArtistTagsInput,
  ActiveLidarrBlock::EditArtistToggleMonitored,
];

pub const EDIT_ARTIST_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::EditArtistToggleMonitored],
  &[ActiveLidarrBlock::EditArtistSelectMonitorNewItems],
  &[ActiveLidarrBlock::EditArtistSelectQualityProfile],
  &[ActiveLidarrBlock::EditArtistSelectMetadataProfile],
  &[ActiveLidarrBlock::EditArtistPathInput],
  &[ActiveLidarrBlock::EditArtistTagsInput],
  &[ActiveLidarrBlock::EditArtistConfirmPrompt],
];

impl From<ActiveLidarrBlock> for Route {
  fn from(active_lidarr_block: ActiveLidarrBlock) -> Route {
    Route::Lidarr(active_lidarr_block, None)
  }
}

impl From<(ActiveLidarrBlock, Option<ActiveLidarrBlock>)> for Route {
  fn from(value: (ActiveLidarrBlock, Option<ActiveLidarrBlock>)) -> Route {
    Route::Lidarr(value.0, value.1)
  }
}
