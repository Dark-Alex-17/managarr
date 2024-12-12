#[cfg(test)]
mod tests {
	use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS};
	use crate::ui::sonarr_ui::library::episode_details_ui::EpisodeDetailsUi;
	use crate::ui::DrawUi;
	use strum::IntoEnumIterator;

	#[test]
	fn test_episode_details_ui_accepts() {
		ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
			if EPISODE_DETAILS_BLOCKS.contains(&active_sonarr_block) {
				assert!(EpisodeDetailsUi::accepts(active_sonarr_block.into()));
			} else {
				assert!(!EpisodeDetailsUi::accepts(active_sonarr_block.into()));
			}
		});
	}
}