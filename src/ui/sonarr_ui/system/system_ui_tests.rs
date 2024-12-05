#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::ui::sonarr_ui::system::SystemUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_system_ui_accepts() {
    let mut system_ui_blocks = Vec::new();
    system_ui_blocks.push(ActiveSonarrBlock::System);
    system_ui_blocks.extend(SYSTEM_DETAILS_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if system_ui_blocks.contains(&active_sonarr_block) {
        assert!(SystemUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SystemUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
