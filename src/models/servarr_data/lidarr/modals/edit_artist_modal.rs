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
pub struct EditArtistModal {
  pub quality_profile_list: StatefulList<String>,
  pub metadata_profile_list: StatefulList<String>,
  pub monitored: Option<bool>,
  pub path: HorizontallyScrollableText,
  pub tags: HorizontallyScrollableText,
}

impl From<&LidarrData<'_>> for EditArtistModal {
  fn from(lidarr_data: &LidarrData<'_>) -> EditArtistModal {
    let mut edit_artist_modal = EditArtistModal::default();
    let Artist {
      path,
      tags,
      monitored,
      quality_profile_id,
      metadata_profile_id,
      ..
    } = lidarr_data.artists.current_selection();

    edit_artist_modal.path = path.clone().into();
    edit_artist_modal.tags = tags
      .iter()
      .map(|tag_id| {
        lidarr_data
          .tags_map
          .get_by_left(&tag_id.as_i64().unwrap())
          .unwrap()
          .clone()
      })
      .collect::<Vec<String>>()
      .join(", ")
      .into();

    edit_artist_modal.monitored = Some(*monitored);

    let mut quality_profile_names: Vec<String> = lidarr_data
      .quality_profile_map
      .right_values()
      .cloned()
      .collect();
    quality_profile_names.sort();
    edit_artist_modal
      .quality_profile_list
      .set_items(quality_profile_names);
    let quality_profile_name = lidarr_data
      .quality_profile_map
      .get_by_left(quality_profile_id)
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

    let mut metadata_profile_names: Vec<String> = lidarr_data
      .metadata_profiles_map
      .right_values()
      .cloned()
      .collect();
    metadata_profile_names.sort();
    edit_artist_modal
      .metadata_profile_list
      .set_items(metadata_profile_names);
    let metadata_profile_name = lidarr_data
      .metadata_profiles_map
      .get_by_left(metadata_profile_id)
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
