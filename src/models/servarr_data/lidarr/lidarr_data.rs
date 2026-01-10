use serde_json::Number;

use super::modals::{AddArtistModal, EditArtistModal};
use crate::app::lidarr::lidarr_context_clues::{
  ARTIST_DETAILS_CONTEXT_CLUES, ARTISTS_CONTEXT_CLUES,
};
use crate::models::{
  BlockSelectionState, HorizontallyScrollableText, Route, TabRoute, TabState,
  lidarr_models::{AddArtistSearchResult, Album, Artist, DownloadRecord},
  servarr_models::{DiskSpace, RootFolder},
  stateful_table::StatefulTable,
};
use crate::network::lidarr_network::LidarrEvent;
use bimap::BiMap;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use strum::EnumIter;
#[cfg(test)]
use {
  crate::models::lidarr_models::{MonitorType, NewItemMonitorType},
  crate::models::stateful_table::SortOption,
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::quality_profile_map,
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{
    add_artist_search_result, album, artist, download_record, metadata_profile,
    metadata_profile_map, quality_profile, root_folder, tags_map,
  },
  crate::network::servarr_test_utils::diskspace,
  strum::{Display, EnumString, IntoEnumIterator},
};

#[cfg(test)]
#[path = "lidarr_data_tests.rs"]
mod lidarr_data_tests;

pub struct LidarrData<'a> {
  pub add_artist_modal: Option<AddArtistModal>,
  pub add_artist_search: Option<HorizontallyScrollableText>,
  pub add_import_list_exclusion: bool,
  pub add_searched_artists: Option<StatefulTable<AddArtistSearchResult>>,
  pub albums: StatefulTable<Album>,
  pub artist_info_tabs: TabState,
  pub artists: StatefulTable<Artist>,
  pub delete_files: bool,
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
  pub fn reset_delete_preferences(&mut self) {
    self.delete_files = false;
    self.add_import_list_exclusion = false;
  }

  pub fn reset_artist_info_tabs(&mut self) {
    self.albums = StatefulTable::default();
    self.artist_info_tabs.index = 0;
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
    self
      .quality_profile_map
      .iter()
      .sorted_by_key(|(id, _)| *id)
      .map(|(_, name)| name)
      .cloned()
      .collect()
  }

  pub fn sorted_metadata_profile_names(&self) -> Vec<String> {
    self
      .metadata_profile_map
      .iter()
      .sorted_by_key(|(id, _)| *id)
      .map(|(_, name)| name)
      .cloned()
      .collect()
  }
}

impl<'a> Default for LidarrData<'a> {
  fn default() -> LidarrData<'a> {
    LidarrData {
      add_artist_modal: None,
      add_artist_search: None,
      add_import_list_exclusion: false,
      add_searched_artists: None,
      albums: StatefulTable::default(),
      artists: StatefulTable::default(),
      delete_files: false,
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
      artist_info_tabs: TabState::new(vec![TabRoute {
        title: "Albums".to_string(),
        route: ActiveLidarrBlock::ArtistDetails.into(),
        contextual_help: Some(&ARTIST_DETAILS_CONTEXT_CLUES),
        config: None,
      }]),
    }
  }
}

#[cfg(test)]
impl LidarrData<'_> {
  pub fn test_default_fully_populated() -> Self {
    let mut add_artist_modal = AddArtistModal {
      tags: "usenet, testing".into(),
      ..AddArtistModal::default()
    };
    add_artist_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_artist_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_artist_modal
      .metadata_profile_list
      .set_items(vec![metadata_profile().name]);
    add_artist_modal
      .quality_profile_list
      .set_items(vec![quality_profile().name]);
    add_artist_modal
      .root_folder_list
      .set_items(vec![root_folder()]);
    let mut edit_artist_modal = EditArtistModal {
      monitored: Some(true),
      path: "/nfs/music".into(),
      tags: "alex".into(),
      ..EditArtistModal::default()
    };
    edit_artist_modal
      .monitor_list
      .set_items(NewItemMonitorType::iter().collect());
    edit_artist_modal
      .quality_profile_list
      .set_items(vec![quality_profile().name]);
    edit_artist_modal
      .metadata_profile_list
      .set_items(vec![metadata_profile().name]);

    let mut lidarr_data = LidarrData {
      delete_files: true,
      disk_space_vec: vec![diskspace()],
      quality_profile_map: quality_profile_map(),
      metadata_profile_map: metadata_profile_map(),
      edit_artist_modal: Some(edit_artist_modal),
      add_artist_modal: Some(add_artist_modal),
      tags_map: tags_map(),
      ..LidarrData::default()
    };
    lidarr_data.albums.set_items(vec![album()]);
    lidarr_data.albums.search = Some("album search".into());
    lidarr_data.artists.set_items(vec![artist()]);
    lidarr_data.artists.sorting(vec![SortOption {
      name: "Name",
      cmp_fn: Some(|a: &Artist, b: &Artist| a.artist_name.text.cmp(&b.artist_name.text)),
    }]);
    lidarr_data.artists.search = Some("artist search".into());
    lidarr_data.artists.filter = Some("artist filter".into());
    lidarr_data.downloads.set_items(vec![download_record()]);
    lidarr_data.root_folders.set_items(vec![root_folder()]);
    lidarr_data.version = "1.0.0".to_owned();
    lidarr_data.add_artist_search = Some("Test Artist".into());
    let mut add_searched_artists = StatefulTable::default();
    add_searched_artists.set_items(vec![add_artist_search_result()]);
    lidarr_data.add_searched_artists = Some(add_searched_artists);

    lidarr_data
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
#[cfg_attr(test, derive(Display, EnumString))]
pub enum ActiveLidarrBlock {
  #[default]
  Artists,
  ArtistDetails,
  ArtistsSortPrompt,
  AddArtistAlreadyInLibrary,
  AddArtistConfirmPrompt,
  AddArtistEmptySearchResults,
  AddArtistPrompt,
  AddArtistSearchInput,
  AddArtistSearchResults,
  AddArtistSelectMetadataProfile,
  AddArtistSelectMonitor,
  AddArtistSelectMonitorNewItems,
  AddArtistSelectQualityProfile,
  AddArtistSelectRootFolder,
  AddArtistTagsInput,
  AutomaticallySearchArtistPrompt,
  DeleteAlbumPrompt,
  DeleteAlbumConfirmPrompt,
  DeleteAlbumToggleDeleteFile,
  DeleteAlbumToggleAddListExclusion,
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
  SearchAlbums,
  SearchAlbumsError,
  SearchArtists,
  SearchArtistsError,
  UpdateAllArtistsPrompt,
  UpdateAndScanArtistPrompt,
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

pub static ARTIST_DETAILS_BLOCKS: [ActiveLidarrBlock; 5] = [
  ActiveLidarrBlock::ArtistDetails,
  ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
  ActiveLidarrBlock::SearchAlbums,
  ActiveLidarrBlock::SearchAlbumsError,
  ActiveLidarrBlock::UpdateAndScanArtistPrompt,
];

pub static ADD_ARTIST_BLOCKS: [ActiveLidarrBlock; 12] = [
  ActiveLidarrBlock::AddArtistAlreadyInLibrary,
  ActiveLidarrBlock::AddArtistConfirmPrompt,
  ActiveLidarrBlock::AddArtistEmptySearchResults,
  ActiveLidarrBlock::AddArtistPrompt,
  ActiveLidarrBlock::AddArtistSearchInput,
  ActiveLidarrBlock::AddArtistSearchResults,
  ActiveLidarrBlock::AddArtistSelectMetadataProfile,
  ActiveLidarrBlock::AddArtistSelectMonitor,
  ActiveLidarrBlock::AddArtistSelectMonitorNewItems,
  ActiveLidarrBlock::AddArtistSelectQualityProfile,
  ActiveLidarrBlock::AddArtistSelectRootFolder,
  ActiveLidarrBlock::AddArtistTagsInput,
];

pub const ADD_ARTIST_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::AddArtistSelectRootFolder],
  &[ActiveLidarrBlock::AddArtistSelectMonitor],
  &[ActiveLidarrBlock::AddArtistSelectMonitorNewItems],
  &[ActiveLidarrBlock::AddArtistSelectQualityProfile],
  &[ActiveLidarrBlock::AddArtistSelectMetadataProfile],
  &[ActiveLidarrBlock::AddArtistTagsInput],
  &[ActiveLidarrBlock::AddArtistConfirmPrompt],
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

pub static DELETE_ALBUM_BLOCKS: [ActiveLidarrBlock; 4] = [
  ActiveLidarrBlock::DeleteAlbumPrompt,
  ActiveLidarrBlock::DeleteAlbumConfirmPrompt,
  ActiveLidarrBlock::DeleteAlbumToggleDeleteFile,
  ActiveLidarrBlock::DeleteAlbumToggleAddListExclusion,
];

pub const DELETE_ALBUM_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::DeleteAlbumToggleDeleteFile],
  &[ActiveLidarrBlock::DeleteAlbumToggleAddListExclusion],
  &[ActiveLidarrBlock::DeleteAlbumConfirmPrompt],
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
