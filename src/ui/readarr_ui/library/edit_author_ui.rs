use std::sync::atomic::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Layout;
use ratatui::widgets::ListItem;

use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::readarr::modals::EditAuthorModal;
use crate::models::servarr_data::readarr::readarr_data::{
  AUTHOR_DETAILS_BLOCKS, ActiveReadarrBlock, EDIT_AUTHOR_BLOCKS,
};
use crate::render_selectable_input_box;

use crate::ui::readarr_ui::library::author_details_ui::AuthorDetailsUi;
use crate::ui::utils::title_block_centered;
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{DrawUi, draw_popup};

#[cfg(test)]
#[path = "edit_author_ui_tests.rs"]
mod edit_author_ui_tests;

pub(super) struct EditAuthorUi;

impl DrawUi for EditAuthorUi {
  fn accepts(route: Route) -> bool {
    let Route::Readarr(active_readarr_block, _) = route else {
      return false;
    };
    EDIT_AUTHOR_BLOCKS.contains(&active_readarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Readarr(active_readarr_block, context_option) = app.get_current_route() {
      if let Some(context) = context_option
        && AUTHOR_DETAILS_BLOCKS.contains(&context)
      {
        draw_popup(f, app, AuthorDetailsUi::draw, Size::Large);
      }

      let draw_edit_author_prompt = |f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect| {
        draw_edit_author_confirmation_prompt(f, app, prompt_area);

        match active_readarr_block {
          ActiveReadarrBlock::EditAuthorSelectMonitorNewItems => {
            draw_edit_author_select_monitor_new_items_popup(f, app);
          }
          ActiveReadarrBlock::EditAuthorSelectQualityProfile => {
            draw_edit_author_select_quality_profile_popup(f, app);
          }
          ActiveReadarrBlock::EditAuthorSelectMetadataProfile => {
            draw_edit_author_select_metadata_profile_popup(f, app);
          }
          _ => (),
        }
      };

      draw_popup(f, app, draw_edit_author_prompt, Size::Long);
    }
  }
}

fn build_edit_author_prompt_title(author_name: &str, disambiguation: &str) -> String {
  if disambiguation.is_empty() {
    format!("Edit - {author_name}")
  } else {
    format!("Edit - {author_name} ({disambiguation})")
  }
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
struct EditAuthorPromptHighlights {
  monitored: bool,
  monitor_new_items: bool,
  quality_profile: bool,
  metadata_profile: bool,
  path: bool,
  tags: bool,
  confirm: bool,
}

fn edit_author_prompt_highlights(selected_block: ActiveReadarrBlock) -> EditAuthorPromptHighlights {
  EditAuthorPromptHighlights {
    monitored: selected_block == ActiveReadarrBlock::EditAuthorToggleMonitored,
    monitor_new_items: selected_block == ActiveReadarrBlock::EditAuthorSelectMonitorNewItems,
    quality_profile: selected_block == ActiveReadarrBlock::EditAuthorSelectQualityProfile,
    metadata_profile: selected_block == ActiveReadarrBlock::EditAuthorSelectMetadataProfile,
    path: selected_block == ActiveReadarrBlock::EditAuthorPathInput,
    tags: selected_block == ActiveReadarrBlock::EditAuthorTagsInput,
    confirm: selected_block == ActiveReadarrBlock::EditAuthorConfirmPrompt,
  }
}

fn draw_edit_author_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let author_name = app
    .data
    .readarr_data
    .authors
    .current_selection()
    .author_name
    .text
    .clone();
  let author_disambiguation = app
    .data
    .readarr_data
    .authors
    .current_selection()
    .disambiguation
    .clone()
    .unwrap_or_default();
  let title = build_edit_author_prompt_title(&author_name, &author_disambiguation);
  f.render_widget(title_block_centered(&title), area);

  let yes_no_value = app.data.readarr_data.prompt_confirm;
  let selected_block = app.data.readarr_data.selected_block.get_active_block();
  let highlights = edit_author_prompt_highlights(selected_block);
  let EditAuthorModal {
    monitor_list,
    quality_profile_list,
    metadata_profile_list,
    monitored,
    path,
    tags,
  } = app
    .data
    .readarr_data
    .edit_author_modal
    .as_ref()
    .expect("edit_author_modal must exist in this context");
  let selected_monitor_new_items = monitor_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();
  let selected_metadata_profile = metadata_profile_list.current_selection();

  let [
    _,
    monitored_area,
    monitor_new_items_area,
    quality_profile_area,
    metadata_profile_area,
    path_area,
    tags_area,
    _,
    buttons_area,
  ] = Layout::vertical([
    Constraint::Fill(1),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Fill(1),
    Constraint::Length(3),
  ])
  .margin(1)
  .areas(area);
  let [save_area, cancel_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  let monitored_checkbox = Checkbox::new("Monitored")
    .checked(monitored.unwrap_or_default())
    .highlighted(highlights.monitored);
  let monitor_new_items_drop_down_button = Button::default()
    .title(selected_monitor_new_items.to_display_str())
    .label("Monitor New Books")
    .icon("▼")
    .selected(highlights.monitor_new_items);
  let quality_profile_drop_down_button = Button::default()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(highlights.quality_profile);
  let metadata_profile_drop_down_button = Button::default()
    .title(selected_metadata_profile)
    .label("Metadata Profile")
    .icon("▼")
    .selected(highlights.metadata_profile);

  if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
    let path_input_box = InputBox::new(&path.text)
      .offset(path.offset.load(Ordering::SeqCst))
      .label("Path")
      .highlighted(highlights.path)
      .selected(active_readarr_block == ActiveReadarrBlock::EditAuthorPathInput);
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(highlights.tags)
      .selected(active_readarr_block == ActiveReadarrBlock::EditAuthorTagsInput);

    match active_readarr_block {
      ActiveReadarrBlock::EditAuthorPathInput => path_input_box.show_cursor(f, path_area),
      ActiveReadarrBlock::EditAuthorTagsInput => tags_input_box.show_cursor(f, tags_area),
      _ => (),
    }

    render_selectable_input_box!(path_input_box, f, path_area);
    render_selectable_input_box!(tags_input_box, f, tags_area);
  }

  let save_button = Button::default()
    .title("Save")
    .selected(yes_no_value && highlights.confirm);
  let cancel_button = Button::default()
    .title("Cancel")
    .selected(!yes_no_value && highlights.confirm);

  f.render_widget(monitored_checkbox, monitored_area);
  f.render_widget(monitor_new_items_drop_down_button, monitor_new_items_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(metadata_profile_drop_down_button, metadata_profile_area);
  f.render_widget(save_button, save_area);
  f.render_widget(cancel_button, cancel_area);
}

fn draw_edit_author_select_monitor_new_items_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_list = SelectableList::new(
    &mut app
      .data
      .readarr_data
      .edit_author_modal
      .as_mut()
      .expect("edit_author_modal must exist in this context")
      .monitor_list,
    |monitor_type| ListItem::new(monitor_type.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_edit_author_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .readarr_data
      .edit_author_modal
      .as_mut()
      .expect("edit_author_modal must exist in this context")
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_edit_author_select_metadata_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let metadata_profile_list = SelectableList::new(
    &mut app
      .data
      .readarr_data
      .edit_author_modal
      .as_mut()
      .expect("edit_author_modal must exist in this context")
      .metadata_profile_list,
    |metadata_profile| ListItem::new(metadata_profile.clone()),
  );
  let popup = Popup::new(metadata_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
