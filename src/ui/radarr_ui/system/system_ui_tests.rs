#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::system::SystemUi;

  #[test]
  fn test_system_ui_accepts() {
    let mut system_ui_blocks = Vec::new();
    system_ui_blocks.push(ActiveRadarrBlock::System);
    system_ui_blocks.extend(SYSTEM_DETAILS_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if system_ui_blocks.contains(&active_radarr_block) {
        assert!(SystemUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!SystemUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
