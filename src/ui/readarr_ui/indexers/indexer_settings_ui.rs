use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};

use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::readarr::readarr_data::{
  ActiveReadarrBlock, INDEXER_SETTINGS_BLOCKS,
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
    let Route::Readarr(active_readarr_block, _) = route else {
      return false;
    };
    INDEXER_SETTINGS_BLOCKS.contains(&active_readarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    draw_popup(f, app, draw_edit_indexer_settings_prompt, Size::LargePrompt);
  }
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
struct IndexerSettingsPromptHighlights {
  minimum_age: bool,
  retention: bool,
  maximum_size: bool,
  rss_sync_interval: bool,
  confirm: bool,
}

fn indexer_settings_prompt_highlights(
  selected_block: ActiveReadarrBlock,
) -> IndexerSettingsPromptHighlights {
  IndexerSettingsPromptHighlights {
    minimum_age: selected_block == ActiveReadarrBlock::IndexerSettingsMinimumAgeInput,
    retention: selected_block == ActiveReadarrBlock::IndexerSettingsRetentionInput,
    maximum_size: selected_block == ActiveReadarrBlock::IndexerSettingsMaximumSizeInput,
    rss_sync_interval: selected_block == ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput,
    confirm: selected_block == ActiveReadarrBlock::IndexerSettingsConfirmPrompt,
  }
}

fn draw_edit_indexer_settings_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let block = title_block_centered("Configure All Indexer Settings");
  let yes_no_value = app.data.readarr_data.prompt_confirm;
  let selected_block = app.data.readarr_data.selected_block.get_active_block();
  let highlights = indexer_settings_prompt_highlights(selected_block);
  let indexer_settings_option = &app.data.readarr_data.indexer_settings;

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

    if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
      let min_age = indexer_settings.minimum_age.to_string();
      let retention = indexer_settings.retention.to_string();
      let max_size = indexer_settings.maximum_size.to_string();
      let rss_sync_interval = indexer_settings.rss_sync_interval.to_string();

      let min_age_text_box = InputBox::new(&min_age)
        .cursor_after_string(false)
        .label("Minimum Age (minutes) ▴▾")
        .highlighted(highlights.minimum_age)
        .selected(active_readarr_block == ActiveReadarrBlock::IndexerSettingsMinimumAgeInput);
      let retention_input_box = InputBox::new(&retention)
        .cursor_after_string(false)
        .label("Retention (days) ▴▾")
        .highlighted(highlights.retention)
        .selected(active_readarr_block == ActiveReadarrBlock::IndexerSettingsRetentionInput);
      let max_size_input_box = InputBox::new(&max_size)
        .cursor_after_string(false)
        .label("Maximum Size (MB) ▴▾")
        .highlighted(highlights.maximum_size)
        .selected(active_readarr_block == ActiveReadarrBlock::IndexerSettingsMaximumSizeInput);
      let rss_sync_interval_input_box = InputBox::new(&rss_sync_interval)
        .cursor_after_string(false)
        .label("RSS Sync Interval (minutes) ▴▾")
        .highlighted(highlights.rss_sync_interval)
        .selected(active_readarr_block == ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput);

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
      .selected(yes_no_value && highlights.confirm);
    let cancel_button = Button::default()
      .title("Cancel")
      .selected(!yes_no_value && highlights.confirm);

    f.render_widget(save_button, save_area);
    f.render_widget(cancel_button, cancel_area);
  } else {
    f.render_widget(LoadingBlock::new(app.is_loading, block), area);
  }
}
