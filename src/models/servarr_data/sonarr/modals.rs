use crate::models::{
  servarr_models::Release, sonarr_models::Episode, stateful_table::StatefulTable, ScrollableText,
};

#[derive(Default)]
pub struct EpisodeDetailsModal {
  pub episode_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  // pub episode_history: StatefulTable<MovieHistoryItem>,
  pub episode_releases: StatefulTable<Release>,
}

#[derive(Default)]
pub struct SeasonDetailsModal {
  pub episodes: StatefulTable<Episode>,
  pub episode_details_modal: Option<EpisodeDetailsModal>,
  pub season_releases: StatefulTable<Release>,
}
