use std::sync::atomic::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Layout;
use ratatui::widgets::ListItem;

use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::lidarr::lidarr_data::{ADD_ROOT_FOLDER_BLOCKS, ActiveLidarrBlock};
use crate::models::servarr_data::lidarr::modals::AddRootFolderModal;
use crate::render_selectable_input_box;

use crate::ui::utils::title_block_centered;
use crate::ui::widgets::button::Button;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{DrawUi, draw_popup};

#[cfg(test)]
#[path = "add_root_folder_ui_tests.rs"]
mod add_root_folder_ui_tests;

pub(super) struct AddRootFolderUi;

impl DrawUi for AddRootFolderUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    ADD_ROOT_FOLDER_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
      let draw_add_root_folder_prompt =
        |f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect| {
          draw_add_root_folder_confirmation_prompt(f, app, prompt_area);

          match active_lidarr_block {
            ActiveLidarrBlock::AddRootFolderSelectMonitor => {
              draw_add_root_folder_select_monitor_popup(f, app);
            }
            ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems => {
              draw_add_root_folder_select_monitor_new_items_popup(f, app);
            }
            ActiveLidarrBlock::AddRootFolderSelectQualityProfile => {
              draw_add_root_folder_select_quality_profile_popup(f, app);
            }
            ActiveLidarrBlock::AddRootFolderSelectMetadataProfile => {
              draw_add_root_folder_select_metadata_profile_popup(f, app);
            }
            _ => (),
          }
        };

      draw_popup(f, app, draw_add_root_folder_prompt, Size::Long);
    }
  }
}

fn draw_add_root_folder_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let title = "Add Root Folder";
  f.render_widget(title_block_centered(title), area);

  let yes_no_value = app.data.lidarr_data.prompt_confirm;
  let selected_block = app.data.lidarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveLidarrBlock::AddRootFolderConfirmPrompt;
  let AddRootFolderModal {
    name,
    path,
    monitor_list,
    monitor_new_items_list,
    quality_profile_list,
    metadata_profile_list,
    tags,
  } = app
    .data
    .lidarr_data
    .add_root_folder_modal
    .as_ref()
    .expect("add_root_folder_modal must exist in this context");
  let selected_monitor = monitor_list.current_selection();
  let selected_monitor_new_items = monitor_new_items_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();
  let selected_metadata_profile = metadata_profile_list.current_selection();

  let [
    _,
    name_area,
    path_area,
    monitor_area,
    monitor_new_items_area,
    quality_profile_area,
    metadata_profile_area,
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
    Constraint::Length(3),
    Constraint::Fill(1),
    Constraint::Length(3),
  ])
  .margin(1)
  .areas(area);
  let [save_area, cancel_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  let monitor_drop_down_button = Button::default()
    .title(selected_monitor.to_display_str())
    .label("Monitor")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::AddRootFolderSelectMonitor);
  let monitor_new_items_drop_down_button = Button::default()
    .title(selected_monitor_new_items.to_display_str())
    .label("Monitor New Albums")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems);
  let quality_profile_drop_down_button = Button::default()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::AddRootFolderSelectQualityProfile);
  let metadata_profile_drop_down_button = Button::default()
    .title(selected_metadata_profile)
    .label("Metadata Profile")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::AddRootFolderSelectMetadataProfile);

  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let name_input_box = InputBox::new(&name.text)
      .offset(name.offset.load(Ordering::SeqCst))
      .label("Name")
      .highlighted(selected_block == ActiveLidarrBlock::AddRootFolderNameInput)
      .selected(active_lidarr_block == ActiveLidarrBlock::AddRootFolderNameInput);
    let path_input_box = InputBox::new(&path.text)
      .offset(path.offset.load(Ordering::SeqCst))
      .label("Path")
      .highlighted(selected_block == ActiveLidarrBlock::AddRootFolderPathInput)
      .selected(active_lidarr_block == ActiveLidarrBlock::AddRootFolderPathInput);
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(selected_block == ActiveLidarrBlock::AddRootFolderTagsInput)
      .selected(active_lidarr_block == ActiveLidarrBlock::AddRootFolderTagsInput);

    match active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderNameInput => name_input_box.show_cursor(f, name_area),
      ActiveLidarrBlock::AddRootFolderPathInput => path_input_box.show_cursor(f, path_area),
      ActiveLidarrBlock::AddRootFolderTagsInput => tags_input_box.show_cursor(f, tags_area),
      _ => (),
    }

    render_selectable_input_box!(name_input_box, f, name_area);
    render_selectable_input_box!(path_input_box, f, path_area);
    render_selectable_input_box!(tags_input_box, f, tags_area);
  }

  let save_button = Button::default()
    .title("Save")
    .selected(yes_no_value && highlight_yes_no);
  let cancel_button = Button::default()
    .title("Cancel")
    .selected(!yes_no_value && highlight_yes_no);

  f.render_widget(monitor_drop_down_button, monitor_area);
  f.render_widget(monitor_new_items_drop_down_button, monitor_new_items_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(metadata_profile_drop_down_button, metadata_profile_area);
  f.render_widget(save_button, save_area);
  f.render_widget(cancel_button, cancel_area);
}

fn draw_add_root_folder_select_monitor_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .add_root_folder_modal
      .as_mut()
      .expect("add_root_folder_modal must exist in this context")
      .monitor_list,
    |monitor_type| ListItem::new(monitor_type.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_root_folder_select_monitor_new_items_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_new_items_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .add_root_folder_modal
      .as_mut()
      .expect("add_root_folder_modal must exist in this context")
      .monitor_new_items_list,
    |monitor_type| ListItem::new(monitor_type.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_new_items_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_root_folder_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .add_root_folder_modal
      .as_mut()
      .expect("add_root_folder_modal must exist in this context")
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_root_folder_select_metadata_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let metadata_profile_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .add_root_folder_modal
      .as_mut()
      .expect("add_root_folder_modal must exist in this context")
      .metadata_profile_list,
    |metadata_profile| ListItem::new(metadata_profile.clone()),
  );
  let popup = Popup::new(metadata_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
