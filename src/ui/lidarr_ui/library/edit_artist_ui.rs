use std::sync::atomic::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Layout;
use ratatui::widgets::ListItem;

use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, EDIT_ARTIST_BLOCKS};
use crate::models::servarr_data::lidarr::modals::EditArtistModal;
use crate::render_selectable_input_box;

use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{title_block_centered};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{DrawUi, draw_popup};

#[cfg(test)]
#[path = "edit_artist_ui_tests.rs"]
mod edit_artist_ui_tests;

pub(super) struct EditArtistUi;

impl DrawUi for EditArtistUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    EDIT_ARTIST_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Lidarr(active_lidarr_block, _context_option) = app.get_current_route() {
      let draw_edit_artist_prompt = |f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect| {
        draw_edit_artist_confirmation_prompt(f, app, prompt_area);

        match active_lidarr_block {
          ActiveLidarrBlock::EditArtistSelectMonitorNewItems => {
            draw_edit_artist_select_monitor_new_items_popup(f, app);
          }
          ActiveLidarrBlock::EditArtistSelectQualityProfile => {
            draw_edit_artist_select_quality_profile_popup(f, app);
          }
          ActiveLidarrBlock::EditArtistSelectMetadataProfile => {
            draw_edit_artist_select_metadata_profile_popup(f, app);
          }
          _ => (),
        }
      };

      draw_popup(f, app, draw_edit_artist_prompt, Size::Long);
    }
  }
}

fn draw_edit_artist_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let artist_name = app
    .data
    .lidarr_data
    .artists
    .current_selection()
    .artist_name
    .text
    .clone();
  let title = format!("Edit - {artist_name}");
  f.render_widget(title_block_centered(&title), area);

  let yes_no_value = app.data.lidarr_data.prompt_confirm;
  let selected_block = app.data.lidarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveLidarrBlock::EditArtistConfirmPrompt;
  let EditArtistModal {
    monitor_list,
    quality_profile_list,
    metadata_profile_list,
    monitored,
    path,
    tags,
  } = app
    .data
    .lidarr_data
    .edit_artist_modal
    .as_ref()
    .expect("edit_artist_modal must exist in this context");
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
    .highlighted(selected_block == ActiveLidarrBlock::EditArtistToggleMonitored);
  let monitor_new_items_drop_down_button = Button::new()
    .title(selected_monitor_new_items.to_display_str())
    .label("Monitor New Albums")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::EditArtistSelectMonitorNewItems);
  let quality_profile_drop_down_button = Button::new()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::EditArtistSelectQualityProfile);
  let metadata_profile_drop_down_button = Button::new()
    .title(selected_metadata_profile)
    .label("Metadata Profile")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::EditArtistSelectMetadataProfile);

  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let path_input_box = InputBox::new(&path.text)
      .offset(path.offset.load(Ordering::SeqCst))
      .label("Path")
      .highlighted(selected_block == ActiveLidarrBlock::EditArtistPathInput)
      .selected(active_lidarr_block == ActiveLidarrBlock::EditArtistPathInput);
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(selected_block == ActiveLidarrBlock::EditArtistTagsInput)
      .selected(active_lidarr_block == ActiveLidarrBlock::EditArtistTagsInput);

    match active_lidarr_block {
      ActiveLidarrBlock::EditArtistPathInput => path_input_box.show_cursor(f, path_area),
      ActiveLidarrBlock::EditArtistTagsInput => tags_input_box.show_cursor(f, tags_area),
      _ => (),
    }

    render_selectable_input_box!(path_input_box, f, path_area);
    render_selectable_input_box!(tags_input_box, f, tags_area);
  }

  let save_button = Button::new()
    .title("Save")
    .selected(yes_no_value && highlight_yes_no);
  let cancel_button = Button::new()
    .title("Cancel")
    .selected(!yes_no_value && highlight_yes_no);

  f.render_widget(monitored_checkbox, monitored_area);
  f.render_widget(monitor_new_items_drop_down_button, monitor_new_items_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(metadata_profile_drop_down_button, metadata_profile_area);
  f.render_widget(save_button, save_area);
  f.render_widget(cancel_button, cancel_area);
}

fn draw_edit_artist_select_monitor_new_items_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .edit_artist_modal
      .as_mut()
      .expect("edit_artist_modal must exist in this context")
      .monitor_list,
    |monitor_type| ListItem::new(monitor_type.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_edit_artist_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .edit_artist_modal
      .as_mut()
      .expect("edit_artist_modal must exist in this context")
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_edit_artist_select_metadata_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let metadata_profile_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .edit_artist_modal
      .as_mut()
      .expect("edit_artist_modal must exist in this context")
      .metadata_profile_list,
    |metadata_profile| ListItem::new(metadata_profile.clone()),
  );
  let popup = Popup::new(metadata_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
