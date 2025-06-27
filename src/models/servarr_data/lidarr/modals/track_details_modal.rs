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

pub struct TrackDetailsModal {
  pub track_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  pub track_history: StatefulTable<LidarrHistoryItem>,
  pub track_releases: StatefulTable<LidarrRelease>,
  pub track_details_tabs: TabState,
}

impl Default for TrackDetailsModal {
  fn default() -> TrackDetailsModal {
    TrackDetailsModal {
      track_details: ScrollableText::default(),
      file_details: String::new(),
      audio_details: String::new(),
      video_details: String::new(),
      track_history: StatefulTable::default(),
      track_releases: StatefulTable::default(),
      track_details_tabs: TabState::new(vec![
        TabRoute {
          title: "Details".to_string(),
          route: ActiveLidarrBlock::TrackDetails.into(),
          help: build_context_clue_string(&TRACK_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveLidarrBlock::TrackHistory.into(),
          help: build_context_clue_string(&TRACK_DETAILS_CONTEXT_CLUES),
          contextual_help: Some(build_context_clue_string(&DETAILS_CONTEXTUAL_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "File".to_string(),
          route: ActiveLidarrBlock::TrackFile.into(),
          help: build_context_clue_string(&TRACK_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
          config: None,
        },
        TabRoute {
          title: "Manual Search".to_string(),
          route: ActiveLidarrBlock::ManualTrackSearch.into(),
          help: build_context_clue_string(&MANUAL_TRACK_SEARCH_CONTEXT_CLUES),
          contextual_help: Some(build_context_clue_string(&DETAILS_CONTEXTUAL_CONTEXT_CLUES)),
          config: None,
        },
      ]),
    }
  }
}
