use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::context_clues::{build_context_clue_string, CONFIRMATION_PROMPT_CONTEXT_CLUES};
use crate::app::App;
use crate::models::servarr_data::sonarr::sonarr_data::{
  ActiveSonarrBlock, INDEXER_SETTINGS_BLOCKS,
};
use crate::models::Route;
use crate::render_selectable_input_box;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::title_block_centered;
use crate::ui::widgets::button::Button;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::popup::Size;
use crate::ui::{draw_popup, DrawUi};

#[cfg(test)]
#[path = "indexer_settings_ui_tests.rs"]
mod indexer_settings_ui_tests;

pub(super) struct IndexerSettingsUi;

impl DrawUi for IndexerSettingsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return INDEXER_SETTINGS_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    draw_popup(f, app, draw_edit_indexer_settings_prompt, Size::LargePrompt);
  }
}

fn draw_edit_indexer_settings_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let block = title_block_centered("Configure All Indexer Settings");
  let yes_no_value = app.data.sonarr_data.prompt_confirm;
  let selected_block = app.data.sonarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveSonarrBlock::IndexerSettingsConfirmPrompt;
  let indexer_settings_option = &app.data.sonarr_data.indexer_settings;
  let help_text = Text::from(build_context_clue_string(&CONFIRMATION_PROMPT_CONTEXT_CLUES).help());
  let help_paragraph = Paragraph::new(help_text).centered();

  if indexer_settings_option.is_some() {
    let indexer_settings = indexer_settings_option.as_ref().unwrap();

    let [_, min_age_area, retention_area, max_size_area, rss_sync_area, _, buttons_area, help_area] =
      Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(1),
      ])
      .margin(1)
      .areas(area);

    if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
      let min_age = indexer_settings.minimum_age.to_string();
      let retention = indexer_settings.retention.to_string();
      let max_size = indexer_settings.maximum_size.to_string();
      let rss_sync_interval = indexer_settings.rss_sync_interval.to_string();

      let min_age_text_box = InputBox::new(&min_age)
        .cursor_after_string(false)
        .label("Minimum Age (minutes) ▴▾")
        .highlighted(selected_block == ActiveSonarrBlock::IndexerSettingsMinimumAgeInput)
        .selected(active_sonarr_block == ActiveSonarrBlock::IndexerSettingsMinimumAgeInput);
      let retention_input_box = InputBox::new(&retention)
        .cursor_after_string(false)
        .label("Retention (days) ▴▾")
        .highlighted(selected_block == ActiveSonarrBlock::IndexerSettingsRetentionInput)
        .selected(active_sonarr_block == ActiveSonarrBlock::IndexerSettingsRetentionInput);
      let max_size_input_box = InputBox::new(&max_size)
        .cursor_after_string(false)
        .label("Maximum Size (MB) ▴▾")
        .highlighted(selected_block == ActiveSonarrBlock::IndexerSettingsMaximumSizeInput)
        .selected(active_sonarr_block == ActiveSonarrBlock::IndexerSettingsMaximumSizeInput);
      let rss_sync_interval_input_box = InputBox::new(&rss_sync_interval)
        .cursor_after_string(false)
        .label("RSS Sync Interval (minutes) ▴▾")
        .highlighted(selected_block == ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput)
        .selected(active_sonarr_block == ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput);

      render_selectable_input_box!(min_age_text_box, f, min_age_area);
      render_selectable_input_box!(retention_input_box, f, retention_area);
      render_selectable_input_box!(max_size_input_box, f, max_size_area);
      render_selectable_input_box!(rss_sync_interval_input_box, f, rss_sync_area);
    }

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
    f.render_widget(save_button, save_area);
    f.render_widget(cancel_button, cancel_area);
    f.render_widget(help_paragraph, help_area);
  } else {
    f.render_widget(LoadingBlock::new(app.is_loading, block), area);
  }
}
