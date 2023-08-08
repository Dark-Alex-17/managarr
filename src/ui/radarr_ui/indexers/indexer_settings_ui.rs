use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

use crate::app::radarr::INDEXER_SETTINGS_BLOCKS;
use crate::app::App;
use crate::models::Route;
use crate::ui::DrawUi;

#[cfg(test)]
#[path = "indexer_settings_ui_tests.rs"]
mod indexer_settings_ui_tests;

pub(super) struct IndexerSettingsUi {}

impl DrawUi for IndexerSettingsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return INDEXER_SETTINGS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw<B: Backend>(_f: &mut Frame<'_, B>, _app: &mut App<'_>, _content_rect: Rect) {}
}
