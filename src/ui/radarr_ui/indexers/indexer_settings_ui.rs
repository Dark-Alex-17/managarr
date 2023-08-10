use std::iter;
use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::Frame;

use crate::app::App;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS,
};
use crate::models::Route;
use crate::ui::radarr_ui::indexers::draw_indexers;
use crate::ui::utils::{
  horizontal_chunks, horizontal_chunks_with_margin, title_block_centered, vertical_chunks,
  vertical_chunks_with_margin,
};
use crate::ui::{
  draw_button, draw_checkbox_with_label, draw_popup_over, draw_text_box_with_label, loading, DrawUi,
};

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

  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    draw_popup_over(
      f,
      app,
      content_rect,
      draw_indexers,
      draw_edit_indexer_settings_prompt,
      60,
      40,
    );
  }
}

fn draw_edit_indexer_settings_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  let block = title_block_centered("Configure All Indexer Settings");
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == &ActiveRadarrBlock::IndexerSettingsConfirmPrompt;
  let indexer_settings_option = &app.data.radarr_data.indexer_settings;

  if indexer_settings_option.is_some() {
    let indexer_settings = indexer_settings_option.as_ref().unwrap();
    f.render_widget(block, prompt_area);

    let chunks = vertical_chunks_with_margin(
      vec![Constraint::Min(0), Constraint::Length(3)],
      prompt_area,
      1,
    );

    let split_chunks = horizontal_chunks_with_margin(
      vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)],
      chunks[0],
      1,
    );

    let left_chunks = vertical_chunks(
      vec![
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
      ],
      split_chunks[0],
    );
    let right_chunks = vertical_chunks(
      vec![
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
      ],
      split_chunks[1],
    );

    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      draw_text_box_with_label(
        f,
        left_chunks[0],
        "Minimum Age",
        &indexer_settings.minimum_age.to_string(),
        0,
        selected_block == &ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
        active_radarr_block == ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
      );
      draw_text_box_with_label(
        f,
        left_chunks[1],
        "Retention",
        &indexer_settings.retention.to_string(),
        0,
        selected_block == &ActiveRadarrBlock::IndexerSettingsRetentionInput,
        active_radarr_block == ActiveRadarrBlock::IndexerSettingsRetentionInput,
      );
      draw_text_box_with_label(
        f,
        left_chunks[2],
        "Maximum Size",
        &indexer_settings.maximum_size.to_string(),
        0,
        selected_block == &ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
        active_radarr_block == ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
      );
      draw_text_box_with_label(
        f,
        right_chunks[0],
        "Availability Delay",
        &indexer_settings.availability_delay.to_string(),
        0,
        selected_block == &ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
        active_radarr_block == ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
      );
      draw_text_box_with_label(
        f,
        right_chunks[1],
        "RSS Sync Interval",
        &indexer_settings.rss_sync_interval.to_string(),
        0,
        selected_block == &ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
        active_radarr_block == ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
      );
      draw_text_box_with_label(
        f,
        right_chunks[2],
        "Whitelisted Subtitle Tags",
        &indexer_settings.whitelisted_hardcoded_subs.to_string(),
        0,
        selected_block == &ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
        active_radarr_block == ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
      );
    }

    draw_checkbox_with_label(
      f,
      left_chunks[3],
      "Prefer Indexer Flags",
      indexer_settings.prefer_indexer_flags,
      selected_block == &ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
    );

    draw_checkbox_with_label(
      f,
      right_chunks[3],
      "Allow Hardcoded Subs",
      indexer_settings.allow_hardcoded_subs,
      selected_block == &ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
    );

    let button_chunks = horizontal_chunks(
      iter::repeat(Constraint::Ratio(1, 4)).take(4).collect(),
      chunks[1],
    );

    draw_button(
      f,
      button_chunks[1],
      "Save",
      yes_no_value && highlight_yes_no,
    );
    draw_button(
      f,
      button_chunks[2],
      "Cancel",
      !yes_no_value && highlight_yes_no,
    );
  } else {
    loading(f, block, prompt_area, app.is_loading);
  }
}
