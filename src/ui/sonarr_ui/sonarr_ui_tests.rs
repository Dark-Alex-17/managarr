#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::{
    app::App,
    models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock,
    ui::{DrawUi, sonarr_ui::SonarrUi, ui_test_utils::test_utils::render_to_string_with_app},
  };

  #[test]
  fn test_sonarr_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      assert!(SonarrUi::accepts(active_sonarr_block.into()));
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveSonarrBlock::Series, 0)]
    #[case(ActiveSonarrBlock::Downloads, 1)]
    #[case(ActiveSonarrBlock::Blocklist, 2)]
    #[case(ActiveSonarrBlock::History, 3)]
    #[case(ActiveSonarrBlock::RootFolders, 4)]
    #[case(ActiveSonarrBlock::Indexers, 5)]
    #[case(ActiveSonarrBlock::System, 6)]
    fn test_sonarr_ui_renders_sonarr_tabs(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data.main_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SonarrUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("sonarr_tabs_{active_sonarr_block}"), output);
    }
  }
}
