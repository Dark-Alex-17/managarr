use crate::app::App;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_INDEXER_BLOCKS};
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
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;
use std::iter;

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

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, content_rect: Rect) {
    draw_popup_over(
      f,
      app,
      content_rect,
      draw_indexers,
      draw_edit_indexer_prompt,
      70,
      45,
    );
  }
}

fn draw_edit_indexer_prompt(f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect) {
  let block = title_block_centered("Edit Indexer");
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == &ActiveRadarrBlock::EditIndexerConfirmPrompt;
  let edit_indexer_modal_option = &app.data.radarr_data.edit_indexer_modal;
  let protocol = &app.data.radarr_data.indexers.current_selection().protocol;

  if edit_indexer_modal_option.is_some() {
    let edit_indexer_modal = edit_indexer_modal_option.as_ref().unwrap();
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

    let [name, rss, auto_search, interactive_search, _] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Min(0),
    ])
    .areas(split_chunks[0]);
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
          area: right_chunks[0],
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
          area: right_chunks[1],
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
            area: right_chunks[2],
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
            area: right_chunks[3],
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
            area: right_chunks[2],
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
    }
  } else {
    loading(f, block, prompt_area, app.is_loading);
  }
}
