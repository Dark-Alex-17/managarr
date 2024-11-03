use std::sync::atomic::Ordering;

use crate::app::context_clues::build_context_clue_string;
use crate::app::radarr::radarr_context_clues::CONFIRMATION_PROMPT_CONTEXT_CLUES;
use crate::app::App;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_INDEXER_BLOCKS};
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
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[cfg(test)]
#[path = "edit_indexer_ui_tests.rs"]
mod edit_indexer_ui_tests;

pub(super) struct EditIndexerUi;

impl DrawUi for EditIndexerUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return EDIT_INDEXER_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    draw_popup_over(
      f,
      app,
      area,
      draw_indexers,
      draw_edit_indexer_prompt,
      Size::LargePrompt,
    );
  }
}

fn draw_edit_indexer_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let block = title_block_centered("Edit Indexer");
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == &ActiveRadarrBlock::EditIndexerConfirmPrompt;
  let edit_indexer_modal_option = &app.data.radarr_data.edit_indexer_modal;
  let protocol = &app.data.radarr_data.indexers.current_selection().protocol;
  let help_text = Text::from(build_context_clue_string(&CONFIRMATION_PROMPT_CONTEXT_CLUES).help());
  let help_paragraph = Paragraph::new(help_text).centered();

  if edit_indexer_modal_option.is_some() {
    let edit_indexer_modal = edit_indexer_modal_option.as_ref().unwrap();

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
    let [name_area, rss_area, auto_search_area, interactive_search_area] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
    ])
    .areas(left_side_area);
    let [url_area, api_key_area, seed_ratio_area, tags_area] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
    ])
    .areas(right_side_area);

    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      let name_input_box = InputBox::new(&edit_indexer_modal.name.text)
        .offset(edit_indexer_modal.name.offset.load(Ordering::SeqCst))
        .label("Name")
        .highlighted(selected_block == &ActiveRadarrBlock::EditIndexerNameInput)
        .selected(active_radarr_block == ActiveRadarrBlock::EditIndexerNameInput);
      let url_input_box = InputBox::new(&edit_indexer_modal.url.text)
        .offset(edit_indexer_modal.url.offset.load(Ordering::SeqCst))
        .label("URL")
        .highlighted(selected_block == &ActiveRadarrBlock::EditIndexerUrlInput)
        .selected(active_radarr_block == ActiveRadarrBlock::EditIndexerUrlInput);
      let api_key_input_box = InputBox::new(&edit_indexer_modal.api_key.text)
        .offset(edit_indexer_modal.api_key.offset.load(Ordering::SeqCst))
        .label("API Key")
        .highlighted(selected_block == &ActiveRadarrBlock::EditIndexerApiKeyInput)
        .selected(active_radarr_block == ActiveRadarrBlock::EditIndexerApiKeyInput);
      let tags_input_box = InputBox::new(&edit_indexer_modal.tags.text)
        .offset(edit_indexer_modal.tags.offset.load(Ordering::SeqCst))
        .label("Tags")
        .highlighted(selected_block == &ActiveRadarrBlock::EditIndexerTagsInput)
        .selected(active_radarr_block == ActiveRadarrBlock::EditIndexerTagsInput);

      render_selectable_input_box!(name_input_box, f, name_area);
      render_selectable_input_box!(url_input_box, f, url_area);
      render_selectable_input_box!(api_key_input_box, f, api_key_area);

      if protocol == "torrent" {
        let seed_ratio_input_box = InputBox::new(&edit_indexer_modal.seed_ratio.text)
          .offset(edit_indexer_modal.seed_ratio.offset.load(Ordering::SeqCst))
          .label("Seed Ratio")
          .highlighted(selected_block == &ActiveRadarrBlock::EditIndexerSeedRatioInput)
          .selected(active_radarr_block == ActiveRadarrBlock::EditIndexerSeedRatioInput);
        let tags_input_box = InputBox::new(&edit_indexer_modal.tags.text)
          .offset(edit_indexer_modal.tags.offset.load(Ordering::SeqCst))
          .label("Tags")
          .highlighted(selected_block == &ActiveRadarrBlock::EditIndexerTagsInput)
          .selected(active_radarr_block == ActiveRadarrBlock::EditIndexerTagsInput);

        render_selectable_input_box!(seed_ratio_input_box, f, seed_ratio_area);
        render_selectable_input_box!(tags_input_box, f, tags_area);
      } else {
        render_selectable_input_box!(tags_input_box, f, seed_ratio_area);
      }

      let rss_checkbox = Checkbox::new("Enable RSS")
        .checked(edit_indexer_modal.enable_rss.unwrap_or_default())
        .highlighted(selected_block == &ActiveRadarrBlock::EditIndexerToggleEnableRss);
      let auto_search_checkbox = Checkbox::new("Enable Automatic Search")
        .checked(
          edit_indexer_modal
            .enable_automatic_search
            .unwrap_or_default(),
        )
        .highlighted(selected_block == &ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch);
      let interactive_search_checkbox = Checkbox::new("Enable Interactive Search")
        .checked(
          edit_indexer_modal
            .enable_interactive_search
            .unwrap_or_default(),
        )
        .highlighted(
          selected_block == &ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch,
        );

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
      f.render_widget(rss_checkbox, rss_area);
      f.render_widget(auto_search_checkbox, auto_search_area);
      f.render_widget(interactive_search_checkbox, interactive_search_area);
      f.render_widget(save_button, save_area);
      f.render_widget(cancel_button, cancel_area);
      f.render_widget(help_paragraph, help_area);
    }
  } else {
    f.render_widget(LoadingBlock::new(app.is_loading, block), area);
  }
}
