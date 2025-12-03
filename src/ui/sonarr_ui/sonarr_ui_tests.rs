#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::{
    models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock,
    ui::{DrawUi, sonarr_ui::SonarrUi},
  };

  #[test]
  fn test_sonarr_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      assert!(SonarrUi::accepts(active_sonarr_block.into()));
    });
  }
}
