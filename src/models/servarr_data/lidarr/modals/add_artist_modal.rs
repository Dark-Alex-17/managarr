use strum::IntoEnumIterator;

use super::super::lidarr_data::{ActiveLidarrBlock, LidarrData};
use crate::models::lidarr_models::ArtistStatus;
use crate::{
  app::{
    context_clues::build_context_clue_string,
    lidarr::lidarr_context_clues::{
      DETAILS_CONTEXTUAL_CONTEXT_CLUES, TRACK_DETAILS_CONTEXT_CLUES,
      MANUAL_TRACK_SEARCH_CONTEXT_CLUES, MANUAL_ALBUM_SEARCH_CONTEXT_CLUES,
      ALBUM_DETAILS_CONTEXTUAL_CONTEXT_CLUES, ALBUM_DETAILS_CONTEXT_CLUES,
      ALBUM_HISTORY_CONTEXT_CLUES,
    },
  },
  models::{
    servarr_data::modals::EditIndexerModal,
    servarr_models::{Indexer, RootFolder},
    lidarr_models::{Album, Artist, LidarrHistoryItem, LidarrRelease, Track},
    stateful_list::StatefulList,
    stateful_table::StatefulTable,
    HorizontallyScrollableText, ScrollableText, TabRoute, TabState,
  },
};

// #[cfg(test)]
// #[path = "modals_tests.rs"]
// mod modals_tests;

#[derive(Default)]
pub struct AddArtistModal {
  pub root_folder_list: StatefulList<RootFolder>,
  pub monitor_list: StatefulList<ArtistStatus>,
  pub quality_profile_list: StatefulList<String>,
  pub metadata_profile_list: StatefulList<String>,
  pub tags: HorizontallyScrollableText,
}

impl From<&LidarrData<'_>> for AddArtistModal {
  fn from(lidarr_data: &LidarrData<'_>) -> AddArtistModal {
    let mut add_artist_modal = AddArtistModal {
      ..AddArtistModal::default()
    };
    add_artist_modal
      .monitor_list
      .set_items(Vec::from_iter(ArtistStatus::iter()));
    let mut quality_profile_names: Vec<String> = lidarr_data
      .quality_profile_map
      .right_values()
      .cloned()
      .collect();
    quality_profile_names.sort();
    add_artist_modal
      .quality_profile_list
      .set_items(quality_profile_names);
    let mut metadata_profile_names: Vec<String> = lidarr_data
      .metadata_profiles_map
      .right_values()
      .cloned()
      .collect();
    metadata_profile_names.sort();
    add_artist_modal
      .metadata_profile_list
      .set_items(metadata_profile_names);
    add_artist_modal
      .root_folder_list
      .set_items(lidarr_data.root_folders.items.to_vec());

    add_artist_modal
  }
}
