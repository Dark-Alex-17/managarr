#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr_data::ActiveRadarrBlock;
  use crate::ui::radarr_ui::RadarrUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_radarr_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      assert!(RadarrUi::accepts(active_radarr_block.into()));
    });
  }
}
