#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ActiveLidarrBlock, DELETE_ALBUM_BLOCKS, DELETE_ALBUM_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::delete_album_ui::DeleteAlbumUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_delete_album_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if DELETE_ALBUM_BLOCKS.contains(&active_lidarr_block) {
        assert!(DeleteAlbumUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!DeleteAlbumUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_delete_album_ui_renders_delete_album() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::DeleteAlbumPrompt.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(DELETE_ALBUM_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DeleteAlbumUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
