use std::sync::atomic::Ordering;

use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::context_clues::{build_context_clue_string, CONFIRMATION_PROMPT_CONTEXT_CLUES};
use crate::app::App;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS,
};
use crate::models::Route;
use crate::render_selectable_input_box;
use crate::ui::radarr_ui::indexers::draw_indexers;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::title_block_centered;
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::popup::Size;
use crate::ui::{draw_popup_over, DrawUi};

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
      Size::LargePrompt,
    );
  }
}

fn draw_edit_indexer_settings_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let block = title_block_centered("Configure All Indexer Settings");
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveRadarrBlock::IndexerSettingsConfirmPrompt;
  let indexer_settings_option = &app.data.radarr_data.indexer_settings;
  let help_text = Text::from(build_context_clue_string(&CONFIRMATION_PROMPT_CONTEXT_CLUES).help());
  let help_paragraph = Paragraph::new(help_text).centered();

  if indexer_settings_option.is_some() {
    let indexer_settings = indexer_settings_option.as_ref().unwrap();

    let [settings_area, _, buttons_area, help_area] = Layout::vertical([
      Constraint::Length(15),
      Constraint::Fill(1),
      Constraint::Length(3),
      Constraint::Length(1),
    ])
    .margin(1)
    .areas(area);
    let [left_side_area, right_side_area] =
      Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .margin(1)
        .areas(settings_area);
    let [min_age_area, retention_area, max_size_area, prefer_flags_area] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
    ])
    .areas(left_side_area);
    let [availability_delay_area, rss_sync_interval_area, whitelisted_sub_tags_area, allow_hardcoded_subs_area] =
      Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
      ])
      .areas(right_side_area);

    if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
      let min_age = indexer_settings.minimum_age.to_string();
      let retention = indexer_settings.retention.to_string();
      let max_size = indexer_settings.maximum_size.to_string();
      let availability_delay = indexer_settings.availability_delay.to_string();
      let rss_sync_interval = indexer_settings.rss_sync_interval.to_string();

      let min_age_text_box = InputBox::new(&min_age)
        .cursor_after_string(false)
        .label("Minimum Age (minutes) ▴▾")
        .highlighted(selected_block == ActiveRadarrBlock::IndexerSettingsMinimumAgeInput)
        .selected(active_radarr_block == ActiveRadarrBlock::IndexerSettingsMinimumAgeInput);
      let retention_input_box = InputBox::new(&retention)
        .cursor_after_string(false)
        .label("Retention (days) ▴▾")
        .highlighted(selected_block == ActiveRadarrBlock::IndexerSettingsRetentionInput)
        .selected(active_radarr_block == ActiveRadarrBlock::IndexerSettingsRetentionInput);
      let max_size_input_box = InputBox::new(&max_size)
        .cursor_after_string(false)
        .label("Maximum Size (MB) ▴▾")
        .highlighted(selected_block == ActiveRadarrBlock::IndexerSettingsMaximumSizeInput)
        .selected(active_radarr_block == ActiveRadarrBlock::IndexerSettingsMaximumSizeInput);
      let availability_delay_input_box = InputBox::new(&availability_delay)
        .cursor_after_string(false)
        .label("Availability Delay (days) ▴▾")
        .highlighted(selected_block == ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput)
        .selected(active_radarr_block == ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput);
      let rss_sync_interval_input_box = InputBox::new(&rss_sync_interval)
        .cursor_after_string(false)
        .label("RSS Sync Interval (minutes) ▴▾")
        .highlighted(selected_block == ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput)
        .selected(active_radarr_block == ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput);
      let whitelisted_subs_input_box =
        InputBox::new(&indexer_settings.whitelisted_hardcoded_subs.text)
          .offset(
            indexer_settings
              .whitelisted_hardcoded_subs
              .offset
              .load(Ordering::SeqCst),
          )
          .label("Whitelisted Subtitle Tags")
          .highlighted(
            selected_block == ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
          )
          .selected(
            active_radarr_block == ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
          );

      render_selectable_input_box!(min_age_text_box, f, min_age_area);
      render_selectable_input_box!(retention_input_box, f, retention_area);
      render_selectable_input_box!(max_size_input_box, f, max_size_area);
      render_selectable_input_box!(availability_delay_input_box, f, availability_delay_area);
      render_selectable_input_box!(rss_sync_interval_input_box, f, rss_sync_interval_area);
      render_selectable_input_box!(whitelisted_subs_input_box, f, whitelisted_sub_tags_area);
    }

    let prefer_indexer_flags_checkbox = Checkbox::new("Prefer Indexer Flags")
      .highlighted(selected_block == ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags)
      .checked(indexer_settings.prefer_indexer_flags);
    let allow_hardcoded_subs_checkbox = Checkbox::new("Allow Hardcoded Subs")
      .highlighted(selected_block == ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs)
      .checked(indexer_settings.allow_hardcoded_subs);

    let [save_area, cancel_area] =
      Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(25)])
        .flex(Flex::Center)
        .areas(buttons_area);

    let save_button = Button::new()
      .title("Save")
      .selected(yes_no_value && highlight_yes_no);
    let cancel_button = Button::new()
      .title("Cancel")
      .selected(!yes_no_value && highlight_yes_no);

    f.render_widget(block, area);
    f.render_widget(prefer_indexer_flags_checkbox, prefer_flags_area);
    f.render_widget(allow_hardcoded_subs_checkbox, allow_hardcoded_subs_area);
    f.render_widget(save_button, save_area);
    f.render_widget(cancel_button, cancel_area);
    f.render_widget(help_paragraph, help_area);
  } else {
    f.render_widget(LoadingBlock::new(app.is_loading, block), area);
  }
}
