#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::Route;
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::ReadarrUi;

  #[test]
  fn test_readarr_ui_accepts() {
    for readarr_block in ActiveReadarrBlock::iter() {
      assert!(ReadarrUi::accepts(Route::Readarr(readarr_block, None)));
    }
  }

  mod snapshot_tests {
    use super::*;
    use crate::app::App;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};
    use rstest::rstest;

    #[rstest]
    #[case(ActiveReadarrBlock::Authors, 0)]
    #[case(ActiveReadarrBlock::Downloads, 1)]
    #[case(ActiveReadarrBlock::Blocklist, 2)]
    #[case(ActiveReadarrBlock::History, 3)]
    #[case(ActiveReadarrBlock::RootFolders, 4)]
    #[case(ActiveReadarrBlock::Indexers, 5)]
    #[case(ActiveReadarrBlock::System, 6)]
    fn test_readarr_ui_renders_readarr_tabs(
      #[case] active_readarr_block: ActiveReadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      app.data.readarr_data.main_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ReadarrUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("readarr_tabs_{active_readarr_block}"), output);
    }
  }
}
