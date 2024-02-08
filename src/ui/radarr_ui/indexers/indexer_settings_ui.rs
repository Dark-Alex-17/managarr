use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::Frame;

use crate::app::App;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS,
};
use crate::models::Route;
use crate::ui::radarr_ui::indexers::draw_indexers;
use crate::ui::utils::title_block_centered;
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

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    draw_popup_over(
      f,
      app,
      area,
      draw_indexers,
      draw_edit_indexer_settings_prompt,
      70,
      45,
    );
  }
}

fn draw_edit_indexer_settings_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let block = title_block_centered("Configure All Indexer Settings");
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == &ActiveRadarrBlock::IndexerSettingsConfirmPrompt;
  let indexer_settings_option = &app.data.radarr_data.indexer_settings;

  if indexer_settings_option.is_some() {
    let indexer_settings = indexer_settings_option.as_ref().unwrap();
    f.render_widget(block, area);

    let [settings_area, buttons_area] =
      Layout::vertical([Constraint::Fill(0), Constraint::Length(3)])
        .margin(1)
        .areas(area);

    let [left_side_area, right_side_area] =
      Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .margin(1)
        .areas(settings_area);

    let [min_age_area, retention_area, max_size_area, prefer_flags_area, _] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(0),
    ])
    .areas(left_side_area);
    let [availability_delay_area, rss_sync_interval_area, whitelisted_sub_tags_area, allow_hardcoded_subs_area, _] =
      Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(0),
      ])
      .areas(right_side_area);

    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      draw_text_box_with_label(
        f,
        LabeledTextBoxProps {
          area: min_age_area,
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
          area: retention_area,
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
          area: max_size_area,
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
          area: availability_delay_area,
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
          area: rss_sync_interval_area,
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
          area: whitelisted_sub_tags_area,
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
      prefer_flags_area,
      "Prefer Indexer Flags",
      indexer_settings.prefer_indexer_flags,
      selected_block == &ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
    );

    draw_checkbox_with_label(
      f,
      allow_hardcoded_subs_area,
      "Allow Hardcoded Subs",
      indexer_settings.allow_hardcoded_subs,
      selected_block == &ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
    );

    let [save_area, cancel_area] =
      Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(25)])
        .flex(Flex::Center)
        .areas(buttons_area);

    draw_button(f, save_area, "Save", yes_no_value && highlight_yes_no);
    draw_button(f, cancel_area, "Cancel", !yes_no_value && highlight_yes_no);
  } else {
    loading(f, block, area, app.is_loading);
  }
}
