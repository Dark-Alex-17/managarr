use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;
#[cfg(test)]
use strum::{Display, EnumString};

use crate::models::{
  Route, TabRoute, TabState,
  lidarr_models::{Artist, DownloadRecord},
  servarr_models::{DiskSpace, RootFolder},
  stateful_table::StatefulTable,
};
use crate::network::lidarr_network::LidarrEvent;

#[cfg(test)]
#[path = "lidarr_data_tests.rs"]
mod lidarr_data_tests;

pub struct LidarrData<'a> {
  pub artists: StatefulTable<Artist>,
  pub disk_space_vec: Vec<DiskSpace>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub main_tabs: TabState,
  pub metadata_profile_map: BiMap<i64, String>,
  pub prompt_confirm: bool,
  pub prompt_confirm_action: Option<LidarrEvent>,
  pub quality_profile_map: BiMap<i64, String>,
  pub root_folders: StatefulTable<RootFolder>,
  pub selected_block: crate::models::BlockSelectionState<'a, ActiveLidarrBlock>,
  pub start_time: DateTime<Utc>,
  pub tags_map: BiMap<i64, String>,
  pub version: String,
}

impl LidarrData<'_> {
  pub fn reset_sorting(&mut self) {
    self.artists.sorting(vec![]);
  }
}

impl<'a> Default for LidarrData<'a> {
  fn default() -> LidarrData<'a> {
    LidarrData {
      artists: StatefulTable::default(),
      disk_space_vec: Vec::new(),
      downloads: StatefulTable::default(),
      metadata_profile_map: BiMap::new(),
      prompt_confirm: false,
      prompt_confirm_action: None,
      quality_profile_map: BiMap::new(),
      root_folders: StatefulTable::default(),
      selected_block: crate::models::BlockSelectionState::default(),
      start_time: Utc::now(),
      tags_map: BiMap::new(),
      version: String::new(),
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library".to_string(),
          route: ActiveLidarrBlock::Artists.into(),
          contextual_help: Some(&ARTISTS_CONTEXT_CLUES),
          config: None,
        },
      ]),
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
    lidarr_data.quality_profile_map = BiMap::from_iter([(1i64, "Lossless".to_owned())]);
    lidarr_data.metadata_profile_map = BiMap::from_iter([(1i64, "Standard".to_owned())]);
    lidarr_data.tags_map = BiMap::from_iter([(1i64, "usenet".to_owned())]);
    lidarr_data.disk_space_vec = vec![DiskSpace {
      free_space: 50000000000,
      total_space: 100000000000,
    }];
    lidarr_data.downloads.set_items(vec![DownloadRecord::default()]);
    lidarr_data.root_folders.set_items(vec![RootFolder::default()]);
    lidarr_data.version = "1.0.0".to_owned();
    
    lidarr_data
  }
}

use crate::app::context_clues::ContextClue;
use crate::app::key_binding::DEFAULT_KEYBINDINGS;

pub static ARTISTS_CONTEXT_CLUES: [ContextClue; 5] = [
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter"),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
#[cfg_attr(test, derive(Display, EnumString))]
pub enum ActiveLidarrBlock {
  #[default]
  Artists,
  ArtistsSortPrompt,
  SearchArtists,
  SearchArtistsError,
  FilterArtists,
  FilterArtistsError,
}

pub static LIBRARY_BLOCKS: [ActiveLidarrBlock; 6] = [
  ActiveLidarrBlock::Artists,
  ActiveLidarrBlock::ArtistsSortPrompt,
  ActiveLidarrBlock::SearchArtists,
  ActiveLidarrBlock::SearchArtistsError,
  ActiveLidarrBlock::FilterArtists,
  ActiveLidarrBlock::FilterArtistsError,
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
