use std::sync::atomic::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Cell, ListItem, Row};

use crate::App;
use crate::models::Route;
use crate::models::lidarr_models::AddArtistSearchResult;
use crate::models::servarr_data::lidarr::lidarr_data::{ADD_ARTIST_BLOCKS, ActiveLidarrBlock};
use crate::models::servarr_data::lidarr::modals::AddArtistModal;
use crate::render_selectable_input_box;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{get_width_from_percentage, layout_block, title_block_centered};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{DrawUi, draw_popup};

#[cfg(test)]
#[path = "add_artist_ui_tests.rs"]
mod add_artist_ui_tests;

pub(super) struct AddArtistUi;

impl DrawUi for AddArtistUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    ADD_ARTIST_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
      draw_popup(f, app, draw_add_artist_search, Size::Large);

      match active_lidarr_block {
        ActiveLidarrBlock::AddArtistPrompt
        | ActiveLidarrBlock::AddArtistSelectMonitor
        | ActiveLidarrBlock::AddArtistSelectMonitorNewItems
        | ActiveLidarrBlock::AddArtistSelectQualityProfile
        | ActiveLidarrBlock::AddArtistSelectMetadataProfile
        | ActiveLidarrBlock::AddArtistSelectRootFolder
        | ActiveLidarrBlock::AddArtistTagsInput => {
          draw_popup(f, app, draw_confirmation_popup, Size::Long);
        }
        ActiveLidarrBlock::AddArtistAlreadyInLibrary => {
          f.render_widget(
            Popup::new(Message::new("This artist is already in your library")).size(Size::Message),
            f.area(),
          );
        }
        _ => (),
      }
    }
  }
}

fn draw_add_artist_search(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let is_loading = app.is_loading || app.data.lidarr_data.add_searched_artists.is_none();
  let current_selection = if let Some(add_searched_artists) =
    app.data.lidarr_data.add_searched_artists.as_ref()
    && !add_searched_artists.is_empty()
  {
    add_searched_artists.current_selection().clone()
  } else {
    AddArtistSearchResult::default()
  };

  let [search_box_area, results_area] =
    Layout::vertical([Constraint::Length(3), Constraint::Fill(0)])
      .margin(1)
      .areas(area);
  let block_content = &app
    .data
    .lidarr_data
    .add_artist_search
    .as_ref()
    .expect("add_artist_search must be populated")
    .text;
  let offset = app
    .data
    .lidarr_data
    .add_artist_search
    .as_ref()
    .expect("add_artist_search must be populated")
    .offset
    .load(Ordering::SeqCst);

  let search_results_row_mapping = |artist: &AddArtistSearchResult| {
    let rating = artist
      .ratings
      .as_ref()
      .map_or(String::new(), |r| format!("{:.1}", r.value));
    let in_library = if app
      .data
      .lidarr_data
      .artists
      .items
      .iter()
      .any(|a| a.foreign_artist_id == artist.foreign_artist_id)
    {
      "✔"
    } else {
      ""
    };

    artist.artist_name.scroll_left_or_reset(
      get_width_from_percentage(area, 27),
      *artist == current_selection,
      app.ui_scroll_tick_count == 0,
    );

    Row::new(vec![
      Cell::from(in_library),
      Cell::from(artist.artist_name.to_string()),
      Cell::from(artist.artist_type.clone().unwrap_or_default()),
      Cell::from(artist.status.to_display_str()),
      Cell::from(rating),
      Cell::from(artist.genres.join(", ")),
    ])
    .primary()
  };

  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    match active_lidarr_block {
      ActiveLidarrBlock::AddArtistSearchInput => {
        let search_box = InputBox::new(block_content)
          .offset(offset)
          .block(title_block_centered("Add Artist"));

        search_box.show_cursor(f, search_box_area);
        f.render_widget(layout_block().default_color(), results_area);
        f.render_widget(search_box, search_box_area);
      }
      ActiveLidarrBlock::AddArtistEmptySearchResults => {
        let error_message = Message::new("No artists found matching your query!");
        let error_message_popup = Popup::new(error_message).size(Size::Message);

        f.render_widget(layout_block().default_color(), results_area);
        f.render_widget(error_message_popup, f.area());
      }
      ActiveLidarrBlock::AddArtistSearchResults
      | ActiveLidarrBlock::AddArtistPrompt
      | ActiveLidarrBlock::AddArtistSelectMonitor
      | ActiveLidarrBlock::AddArtistSelectMonitorNewItems
      | ActiveLidarrBlock::AddArtistSelectQualityProfile
      | ActiveLidarrBlock::AddArtistSelectMetadataProfile
      | ActiveLidarrBlock::AddArtistSelectRootFolder
      | ActiveLidarrBlock::AddArtistAlreadyInLibrary
      | ActiveLidarrBlock::AddArtistTagsInput => {
        let search_results_table = ManagarrTable::new(
          app.data.lidarr_data.add_searched_artists.as_mut(),
          search_results_row_mapping,
        )
        .loading(is_loading)
        .block(layout_block().default_color())
        .headers(["✔", "Name", "Type", "Status", "Rating", "Genres"])
        .constraints([
          Constraint::Percentage(3),
          Constraint::Percentage(27),
          Constraint::Percentage(12),
          Constraint::Percentage(12),
          Constraint::Percentage(8),
          Constraint::Percentage(38),
        ]);

        f.render_widget(search_results_table, results_area);
      }
      _ => (),
    }
  }

  f.render_widget(
    InputBox::new(block_content)
      .offset(offset)
      .block(title_block_centered("Add Artist")),
    search_box_area,
  );
}

fn draw_confirmation_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    match active_lidarr_block {
      ActiveLidarrBlock::AddArtistSelectMonitor => {
        draw_confirmation_prompt(f, app, area);
        draw_add_artist_select_monitor_popup(f, app);
      }
      ActiveLidarrBlock::AddArtistSelectMonitorNewItems => {
        draw_confirmation_prompt(f, app, area);
        draw_add_artist_select_monitor_new_items_popup(f, app);
      }
      ActiveLidarrBlock::AddArtistSelectQualityProfile => {
        draw_confirmation_prompt(f, app, area);
        draw_add_artist_select_quality_profile_popup(f, app);
      }
      ActiveLidarrBlock::AddArtistSelectMetadataProfile => {
        draw_confirmation_prompt(f, app, area);
        draw_add_artist_select_metadata_profile_popup(f, app);
      }
      ActiveLidarrBlock::AddArtistSelectRootFolder => {
        draw_confirmation_prompt(f, app, area);
        draw_add_artist_select_root_folder_popup(f, app);
      }
      ActiveLidarrBlock::AddArtistPrompt | ActiveLidarrBlock::AddArtistTagsInput => {
        draw_confirmation_prompt(f, app, area)
      }
      _ => (),
    }
  }
}

fn draw_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let searched_artist = app
    .data
    .lidarr_data
    .add_searched_artists
    .as_ref()
    .expect("add_searched_artists must be populated")
    .current_selection();
  let artist_name = &searched_artist.artist_name.text;
  let artist_disambiguation = searched_artist.disambiguation.clone().unwrap_or_default();

  let title = if artist_disambiguation.is_empty() {
    format!("Add - {artist_name}")
  } else {
    format!("Add - {artist_name} ({artist_disambiguation})")
  };
  let yes_no_value = app.data.lidarr_data.prompt_confirm;
  let selected_block = app.data.lidarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveLidarrBlock::AddArtistConfirmPrompt;
  let AddArtistModal {
    monitor_list,
    monitor_new_items_list,
    quality_profile_list,
    metadata_profile_list,
    root_folder_list,
    tags,
    ..
  } = app
    .data
    .lidarr_data
    .add_artist_modal
    .as_ref()
    .expect("add_artist_modal must exist in this context");

  let selected_monitor = monitor_list.current_selection();
  let selected_monitor_new_items = monitor_new_items_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();
  let selected_metadata_profile = metadata_profile_list.current_selection();
  let selected_root_folder = root_folder_list.current_selection();

  f.render_widget(title_block_centered(&title), area);

  let [
    _,
    root_folder_area,
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
    Constraint::Fill(1),
    Constraint::Length(3),
  ])
  .margin(1)
  .areas(area);

  let [add_area, cancel_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  let root_folder_drop_down_button = Button::default()
    .title(&selected_root_folder.path)
    .label("Root Folder")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::AddArtistSelectRootFolder);
  let monitor_drop_down_button = Button::default()
    .title(selected_monitor.to_display_str())
    .label("Monitor")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::AddArtistSelectMonitor);
  let monitor_new_items_drop_down_button = Button::default()
    .title(selected_monitor_new_items.to_display_str())
    .label("Monitor New Items")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::AddArtistSelectMonitorNewItems);
  let quality_profile_drop_down_button = Button::default()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::AddArtistSelectQualityProfile);
  let metadata_profile_drop_down_button = Button::default()
    .title(selected_metadata_profile)
    .label("Metadata Profile")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::AddArtistSelectMetadataProfile);

  f.render_widget(root_folder_drop_down_button, root_folder_area);
  f.render_widget(monitor_drop_down_button, monitor_area);
  f.render_widget(monitor_new_items_drop_down_button, monitor_new_items_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(metadata_profile_drop_down_button, metadata_profile_area);

  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(selected_block == ActiveLidarrBlock::AddArtistTagsInput)
      .selected(active_lidarr_block == ActiveLidarrBlock::AddArtistTagsInput);
    render_selectable_input_box!(tags_input_box, f, tags_area);
  }

  let add_button = Button::default()
    .title("Add")
    .selected(yes_no_value && highlight_yes_no);
  let cancel_button = Button::default()
    .title("Cancel")
    .selected(!yes_no_value && highlight_yes_no);

  f.render_widget(add_button, add_area);
  f.render_widget(cancel_button, cancel_area);
}

fn draw_add_artist_select_monitor_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .add_artist_modal
      .as_mut()
      .expect("add_artist_modal must exist in this context")
      .monitor_list,
    |monitor| ListItem::new(monitor.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_artist_select_monitor_new_items_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_new_items_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .add_artist_modal
      .as_mut()
      .expect("add_artist_modal must exist in this context")
      .monitor_new_items_list,
    |monitor_new_items| ListItem::new(monitor_new_items.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_new_items_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_artist_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .add_artist_modal
      .as_mut()
      .expect("add_artist_modal must exist in this context")
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_artist_select_metadata_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let metadata_profile_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .add_artist_modal
      .as_mut()
      .expect("add_artist_modal must exist in this context")
      .metadata_profile_list,
    |metadata_profile| ListItem::new(metadata_profile.clone()),
  );
  let popup = Popup::new(metadata_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_artist_select_root_folder_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let root_folder_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .add_artist_modal
      .as_mut()
      .expect("add_artist_modal must exist in this context")
      .root_folder_list,
    |root_folder| ListItem::new(root_folder.path.to_owned()),
  );
  let popup = Popup::new(root_folder_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
