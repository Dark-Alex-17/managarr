use ratatui::layout::{Constraint, Rect};
use ratatui::Frame;
use std::iter;

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
  draw_button, draw_checkbox_with_label, draw_popup_over, draw_text_box_with_label, loading,
  DrawUi, LabeledTextBoxProps,
};

#[cfg(test)]
#[path = "indexer_settings_ui_tests.rs"]
mod indexer_settings_ui_tests;

pub(super) struct IndexerSettingsUi;

impl DrawUi for IndexerSettingsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return INDEXER_SETTINGS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, content_rect: Rect) {
    draw_popup_over(
      f,
      app,
      content_rect,
      draw_indexers,
      draw_edit_indexer_settings_prompt,
      70,
      45,
    );
  }
}

fn draw_edit_indexer_settings_prompt(f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect) {
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
        LabeledTextBoxProps {
          area: left_chunks[0],
          label: "Minimum Age (minutes) ▴▾",
          text: &indexer_settings.minimum_age.to_string(),
          offset: 0,
          is_selected: selected_block == &ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
          should_show_cursor: active_radarr_block
            == ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
          cursor_after_string: false,
        },
      );
      draw_text_box_with_label(
        f,
        LabeledTextBoxProps {
          area: left_chunks[1],
          label: "Retention (days) ▴▾",
          text: &indexer_settings.retention.to_string(),
          offset: 0,
          is_selected: selected_block == &ActiveRadarrBlock::IndexerSettingsRetentionInput,
          should_show_cursor: active_radarr_block
            == ActiveRadarrBlock::IndexerSettingsRetentionInput,
          cursor_after_string: false,
        },
      );
      draw_text_box_with_label(
        f,
        LabeledTextBoxProps {
          area: left_chunks[2],
          label: "Maximum Size (MB) ▴▾",
          text: &indexer_settings.maximum_size.to_string(),
          offset: 0,
          is_selected: selected_block == &ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
          should_show_cursor: active_radarr_block
            == ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
          cursor_after_string: false,
        },
      );
      draw_text_box_with_label(
        f,
        LabeledTextBoxProps {
          area: right_chunks[0],
          label: "Availability Delay (days) ▴▾",
          text: &indexer_settings.availability_delay.to_string(),
          offset: 0,
          is_selected: selected_block == &ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
          should_show_cursor: active_radarr_block
            == ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
          cursor_after_string: false,
        },
      );
      draw_text_box_with_label(
        f,
        LabeledTextBoxProps {
          area: right_chunks[1],
          label: "RSS Sync Interval (minutes) ▴▾",
          text: &indexer_settings.rss_sync_interval.to_string(),
          offset: 0,
          is_selected: selected_block == &ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
          should_show_cursor: active_radarr_block
            == ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
          cursor_after_string: false,
        },
      );
      draw_text_box_with_label(
        f,
        LabeledTextBoxProps {
          area: right_chunks[2],
          label: "Whitelisted Subtitle Tags",
          text: &indexer_settings.whitelisted_hardcoded_subs.text,
          offset: *indexer_settings.whitelisted_hardcoded_subs.offset.borrow(),
          is_selected: selected_block
            == &ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
          should_show_cursor: active_radarr_block
            == ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
          cursor_after_string: true,
        },
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
