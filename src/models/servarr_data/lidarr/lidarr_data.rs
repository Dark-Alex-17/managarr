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
use strum::EnumIter;
#[cfg(test)]
use strum::{Display, EnumString};

#[cfg(test)]
#[path = "lidarr_data_tests.rs"]
mod lidarr_data_tests;

pub struct LidarrData<'a> {
  pub add_import_list_exclusion: bool,
  pub artists: StatefulTable<Artist>,
  pub delete_artist_files: bool,
  pub disk_space_vec: Vec<DiskSpace>,
  pub downloads: StatefulTable<DownloadRecord>,
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
}

impl<'a> Default for LidarrData<'a> {
  fn default() -> LidarrData<'a> {
    LidarrData {
      add_import_list_exclusion: false,
      artists: StatefulTable::default(),
      delete_artist_files: false,
      disk_space_vec: Vec::new(),
      downloads: StatefulTable::default(),
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
    use crate::models::lidarr_models::{Artist, DownloadRecord};
    use crate::models::servarr_models::{DiskSpace, RootFolder};
    use crate::models::stateful_table::SortOption;

    let mut lidarr_data = LidarrData::default();
    lidarr_data.artists.set_items(vec![Artist::default()]);
    lidarr_data.artists.sorting(vec![SortOption {
      name: "Name",
      cmp_fn: Some(|a: &Artist, b: &Artist| a.artist_name.text.cmp(&b.artist_name.text)),
    }]);
    lidarr_data.artists.search = Some("artist search".into());
    lidarr_data.artists.filter = Some("artist filter".into());
    lidarr_data.quality_profile_map = BiMap::from_iter([(1i64, "Lossless".to_owned())]);
    lidarr_data.metadata_profile_map = BiMap::from_iter([(1i64, "Standard".to_owned())]);
    lidarr_data.tags_map = BiMap::from_iter([(1i64, "usenet".to_owned())]);
    lidarr_data.disk_space_vec = vec![DiskSpace {
      free_space: 50000000000,
      total_space: 100000000000,
    }];
    lidarr_data
      .downloads
      .set_items(vec![DownloadRecord::default()]);
    lidarr_data
      .root_folders
      .set_items(vec![RootFolder::default()]);
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
