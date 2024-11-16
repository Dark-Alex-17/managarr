use crate::models::ScrollableText;

#[derive(Default)]
pub struct EpisodeDetailsModal {
  pub episode_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  // pub episode_history: StatefulTable<MovieHistoryItem>,
  // pub episode_cast: StatefulTable<Credit>,
  // pub episode_crew: StatefulTable<Credit>,
  // pub episode_releases: StatefulTable<Release>,
}
