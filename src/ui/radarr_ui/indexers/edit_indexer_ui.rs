use crate::app::App;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_INDEXER_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::indexers::draw_indexers;
use crate::ui::utils::title_block_centered;
use crate::ui::{
  draw_button, draw_checkbox_with_label, draw_popup_over, draw_text_box_with_label, loading,
  DrawUi, LabeledTextBoxProps,
};
use ratatui::layout::{Constraint, Flex, Layout, Rect};
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
      70,
      45,
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

  if edit_indexer_modal_option.is_some() {
    let edit_indexer_modal = edit_indexer_modal_option.as_ref().unwrap();
    f.render_widget(block, area);

    let [settings_area, buttons_area] =
      Layout::vertical([Constraint::Fill(0), Constraint::Length(3)])
        .margin(1)
        .areas(area);

    let [left_side_area, right_side_area] =
      Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .margin(1)
        .areas(settings_area);

    let [name, rss, auto_search, interactive_search, _] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(0),
    ])
    .areas(left_side_area);
    let [url_area, api_key_area, seed_ratio_area, tags_area, _] = Layout::vertical([
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
          area: name,
          label: "Name",
          text: &edit_indexer_modal.name.text,
          offset: *edit_indexer_modal.name.offset.borrow(),
          is_selected: selected_block == &ActiveRadarrBlock::EditIndexerNameInput,
          should_show_cursor: active_radarr_block == ActiveRadarrBlock::EditIndexerNameInput,
          cursor_after_string: true,
        },
      );
      draw_text_box_with_label(
        f,
        LabeledTextBoxProps {
          area: url_area,
          label: "URL",
          text: &edit_indexer_modal.url.text,
          offset: *edit_indexer_modal.url.offset.borrow(),
          is_selected: selected_block == &ActiveRadarrBlock::EditIndexerUrlInput,
          should_show_cursor: active_radarr_block == ActiveRadarrBlock::EditIndexerUrlInput,
          cursor_after_string: true,
        },
      );
      draw_text_box_with_label(
        f,
        LabeledTextBoxProps {
          area: api_key_area,
          label: "API Key",
          text: &edit_indexer_modal.api_key.text,
          offset: *edit_indexer_modal.api_key.offset.borrow(),
          is_selected: selected_block == &ActiveRadarrBlock::EditIndexerApiKeyInput,
          should_show_cursor: active_radarr_block == ActiveRadarrBlock::EditIndexerApiKeyInput,
          cursor_after_string: true,
        },
      );
      if protocol == "torrent" {
        draw_text_box_with_label(
          f,
          LabeledTextBoxProps {
            area: seed_ratio_area,
            label: "Seed Ratio",
            text: &edit_indexer_modal.seed_ratio.text,
            offset: *edit_indexer_modal.seed_ratio.offset.borrow(),
            is_selected: selected_block == &ActiveRadarrBlock::EditIndexerSeedRatioInput,
            should_show_cursor: active_radarr_block == ActiveRadarrBlock::EditIndexerSeedRatioInput,
            cursor_after_string: true,
          },
        );
        draw_text_box_with_label(
          f,
          LabeledTextBoxProps {
            area: tags_area,
            label: "Tags",
            text: &edit_indexer_modal.tags.text,
            offset: *edit_indexer_modal.tags.offset.borrow(),
            is_selected: selected_block == &ActiveRadarrBlock::EditIndexerTagsInput,
            should_show_cursor: active_radarr_block == ActiveRadarrBlock::EditIndexerTagsInput,
            cursor_after_string: true,
          },
        );
      } else {
        draw_text_box_with_label(
          f,
          LabeledTextBoxProps {
            area: seed_ratio_area,
            label: "Tags",
            text: &edit_indexer_modal.tags.text,
            offset: *edit_indexer_modal.tags.offset.borrow(),
            is_selected: selected_block == &ActiveRadarrBlock::EditIndexerTagsInput,
            should_show_cursor: active_radarr_block == ActiveRadarrBlock::EditIndexerTagsInput,
            cursor_after_string: true,
          },
        );
      }

      draw_checkbox_with_label(
        f,
        rss,
        "Enable RSS",
        edit_indexer_modal.enable_rss.unwrap_or_default(),
        selected_block == &ActiveRadarrBlock::EditIndexerToggleEnableRss,
      );
      draw_checkbox_with_label(
        f,
        auto_search,
        "Enable Automatic Search",
        edit_indexer_modal
          .enable_automatic_search
          .unwrap_or_default(),
        selected_block == &ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch,
      );
      draw_checkbox_with_label(
        f,
        interactive_search,
        "Enable Interactive Search",
        edit_indexer_modal
          .enable_interactive_search
          .unwrap_or_default(),
        selected_block == &ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch,
      );

      let [save_area, cancel_area] =
        Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(25)])
          .flex(Flex::Center)
          .areas(buttons_area);

      draw_button(f, save_area, "Save", yes_no_value && highlight_yes_no);
      draw_button(f, cancel_area, "Cancel", !yes_no_value && highlight_yes_no);
    }
  } else {
    loading(f, block, area, app.is_loading);
  }
}
