use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};

use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::lidarr::lidarr_data::{
  ActiveLidarrBlock, INDEXER_SETTINGS_BLOCKS,
};
use crate::render_selectable_input_box;
use crate::ui::utils::title_block_centered;
use crate::ui::widgets::button::Button;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::popup::Size;
use crate::ui::{DrawUi, draw_popup};

#[cfg(test)]
#[path = "indexer_settings_ui_tests.rs"]
mod indexer_settings_ui_tests;

pub(super) struct IndexerSettingsUi;

impl DrawUi for IndexerSettingsUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    INDEXER_SETTINGS_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    draw_popup(f, app, draw_edit_indexer_settings_prompt, Size::LargePrompt);
  }
}

fn draw_edit_indexer_settings_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let block = title_block_centered("Configure All Indexer Settings");
  let yes_no_value = app.data.lidarr_data.prompt_confirm;
  let selected_block = app.data.lidarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveLidarrBlock::IndexerSettingsConfirmPrompt;
  let indexer_settings_option = &app.data.lidarr_data.indexer_settings;

  if indexer_settings_option.is_some() {
    f.render_widget(block, area);
    let indexer_settings = indexer_settings_option.as_ref().unwrap();

    let [
      _,
      min_age_area,
      retention_area,
      max_size_area,
      rss_sync_area,
      _,
      buttons_area,
    ] = Layout::vertical([
      Constraint::Fill(1),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(1),
      Constraint::Length(3),
    ])
    .margin(1)
    .areas(area);

    if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
      let min_age = indexer_settings.minimum_age.to_string();
      let retention = indexer_settings.retention.to_string();
      let max_size = indexer_settings.maximum_size.to_string();
      let rss_sync_interval = indexer_settings.rss_sync_interval.to_string();

      let min_age_text_box = InputBox::new(&min_age)
        .cursor_after_string(false)
        .label("Minimum Age (minutes) ▴▾")
        .highlighted(selected_block == ActiveLidarrBlock::IndexerSettingsMinimumAgeInput)
        .selected(active_lidarr_block == ActiveLidarrBlock::IndexerSettingsMinimumAgeInput);
      let retention_input_box = InputBox::new(&retention)
        .cursor_after_string(false)
        .label("Retention (days) ▴▾")
        .highlighted(selected_block == ActiveLidarrBlock::IndexerSettingsRetentionInput)
        .selected(active_lidarr_block == ActiveLidarrBlock::IndexerSettingsRetentionInput);
      let max_size_input_box = InputBox::new(&max_size)
        .cursor_after_string(false)
        .label("Maximum Size (MB) ▴▾")
        .highlighted(selected_block == ActiveLidarrBlock::IndexerSettingsMaximumSizeInput)
        .selected(active_lidarr_block == ActiveLidarrBlock::IndexerSettingsMaximumSizeInput);
      let rss_sync_interval_input_box = InputBox::new(&rss_sync_interval)
        .cursor_after_string(false)
        .label("RSS Sync Interval (minutes) ▴▾")
        .highlighted(selected_block == ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput)
        .selected(active_lidarr_block == ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput);

      render_selectable_input_box!(min_age_text_box, f, min_age_area);
      render_selectable_input_box!(retention_input_box, f, retention_area);
      render_selectable_input_box!(max_size_input_box, f, max_size_area);
      render_selectable_input_box!(rss_sync_interval_input_box, f, rss_sync_area);
    }

    let [save_area, cancel_area] =
      Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(25)])
        .flex(Flex::Center)
        .areas(buttons_area);

    let save_button = Button::default()
      .title("Save")
      .selected(yes_no_value && highlight_yes_no);
    let cancel_button = Button::default()
      .title("Cancel")
      .selected(!yes_no_value && highlight_yes_no);

    f.render_widget(save_button, save_area);
    f.render_widget(cancel_button, cancel_area);
  } else {
    f.render_widget(LoadingBlock::new(app.is_loading, block), area);
  }
}
