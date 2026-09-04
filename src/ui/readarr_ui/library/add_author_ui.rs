use std::sync::atomic::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Cell, ListItem, Row};

use crate::App;
use crate::models::Route;
use crate::models::readarr_models::AddAuthorSearchResult;
use crate::models::servarr_data::readarr::modals::AddAuthorModal;
use crate::models::servarr_data::readarr::readarr_data::{ADD_AUTHOR_BLOCKS, ActiveReadarrBlock};
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
#[path = "add_author_ui_tests.rs"]
mod add_author_ui_tests;

pub(super) struct AddAuthorUi;

impl DrawUi for AddAuthorUi {
  fn accepts(route: Route) -> bool {
    let Route::Readarr(active_readarr_block, _) = route else {
      return false;
    };
    ADD_AUTHOR_BLOCKS.contains(&active_readarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
      draw_popup(f, app, draw_add_author_search, Size::Large);

      match active_readarr_block {
        ActiveReadarrBlock::AddAuthorPrompt
        | ActiveReadarrBlock::AddAuthorSelectMonitor
        | ActiveReadarrBlock::AddAuthorSelectMonitorNewItems
        | ActiveReadarrBlock::AddAuthorSelectQualityProfile
        | ActiveReadarrBlock::AddAuthorSelectMetadataProfile
        | ActiveReadarrBlock::AddAuthorSelectRootFolder
        | ActiveReadarrBlock::AddAuthorTagsInput => {
          draw_popup(f, app, draw_confirmation_popup, Size::Long);
        }
        ActiveReadarrBlock::AddAuthorAlreadyInLibrary => {
          f.render_widget(
            Popup::new(Message::new("This author is already in your library")).size(Size::Message),
            f.area(),
          );
        }
        _ => (),
      }
    }
  }
}

fn build_add_author_prompt_title(author_name: &str, disambiguation: &str) -> String {
  if disambiguation.is_empty() {
    format!("Add - {author_name}")
  } else {
    format!("Add - {author_name} ({disambiguation})")
  }
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
struct AddAuthorPromptHighlights {
  root_folder: bool,
  monitor: bool,
  monitor_new_items: bool,
  quality_profile: bool,
  metadata_profile: bool,
  tags: bool,
  confirm: bool,
}

fn add_author_prompt_highlights(selected_block: ActiveReadarrBlock) -> AddAuthorPromptHighlights {
  AddAuthorPromptHighlights {
    root_folder: selected_block == ActiveReadarrBlock::AddAuthorSelectRootFolder,
    monitor: selected_block == ActiveReadarrBlock::AddAuthorSelectMonitor,
    monitor_new_items: selected_block == ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
    quality_profile: selected_block == ActiveReadarrBlock::AddAuthorSelectQualityProfile,
    metadata_profile: selected_block == ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
    tags: selected_block == ActiveReadarrBlock::AddAuthorTagsInput,
    confirm: selected_block == ActiveReadarrBlock::AddAuthorConfirmPrompt,
  }
}

fn already_in_library_marker(is_in_library: bool) -> &'static str {
  if is_in_library { "✔" } else { "" }
}

fn draw_add_author_search(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let is_loading = app.is_loading || app.data.readarr_data.add_searched_authors.is_none();
  let current_selection = if let Some(add_searched_authors) =
    app.data.readarr_data.add_searched_authors.as_ref()
    && !add_searched_authors.is_empty()
  {
    add_searched_authors.current_selection().clone()
  } else {
    AddAuthorSearchResult::default()
  };

  let [search_box_area, results_area] =
    Layout::vertical([Constraint::Length(3), Constraint::Fill(0)])
      .margin(1)
      .areas(area);
  let block_content = &app
    .data
    .readarr_data
    .add_author_search
    .as_ref()
    .expect("add_author_search must be populated")
    .text;
  let offset = app
    .data
    .readarr_data
    .add_author_search
    .as_ref()
    .expect("add_author_search must be populated")
    .offset
    .load(Ordering::SeqCst);

  let search_results_row_mapping = |author: &AddAuthorSearchResult| {
    let rating = author
      .ratings
      .as_ref()
      .map_or(String::new(), |r| format!("{:.1}", r.value));
    let in_library = already_in_library_marker(
      app
        .data
        .readarr_data
        .authors
        .items
        .iter()
        .any(|a| a.foreign_author_id == author.foreign_author_id),
    );

    author.author_name.scroll_left_or_reset(
      get_width_from_percentage(area, 27),
      *author == current_selection,
      app.should_text_scroll,
    );

    Row::new(vec![
      Cell::from(in_library),
      Cell::from(author.author_name.to_string()),
      Cell::from(author.author_type.clone().unwrap_or_default()),
      Cell::from(author.status.to_display_str()),
      Cell::from(rating),
      Cell::from(author.genres.join(", ")),
    ])
    .primary()
  };

  if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
    match active_readarr_block {
      ActiveReadarrBlock::AddAuthorSearchInput => {
        let search_box = InputBox::new(block_content)
          .offset(offset)
          .block(title_block_centered("Add Author"));

        search_box.show_cursor(f, search_box_area);
        f.render_widget(layout_block().default_color(), results_area);
        f.render_widget(search_box, search_box_area);
      }
      ActiveReadarrBlock::AddAuthorEmptySearchResults => {
        let error_message = Message::new("No authors found matching your query!");
        let error_message_popup = Popup::new(error_message).size(Size::Message);

        f.render_widget(layout_block().default_color(), results_area);
        f.render_widget(error_message_popup, f.area());
      }
      ActiveReadarrBlock::AddAuthorSearchResults
      | ActiveReadarrBlock::AddAuthorPrompt
      | ActiveReadarrBlock::AddAuthorSelectMonitor
      | ActiveReadarrBlock::AddAuthorSelectMonitorNewItems
      | ActiveReadarrBlock::AddAuthorSelectQualityProfile
      | ActiveReadarrBlock::AddAuthorSelectMetadataProfile
      | ActiveReadarrBlock::AddAuthorSelectRootFolder
      | ActiveReadarrBlock::AddAuthorAlreadyInLibrary
      | ActiveReadarrBlock::AddAuthorTagsInput => {
        let search_results_table = ManagarrTable::new(
          app.data.readarr_data.add_searched_authors.as_mut(),
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
      .block(title_block_centered("Add Author")),
    search_box_area,
  );
}

fn draw_confirmation_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
    match active_readarr_block {
      ActiveReadarrBlock::AddAuthorSelectMonitor => {
        draw_confirmation_prompt(f, app, area);
        draw_add_author_select_monitor_popup(f, app);
      }
      ActiveReadarrBlock::AddAuthorSelectMonitorNewItems => {
        draw_confirmation_prompt(f, app, area);
        draw_add_author_select_monitor_new_items_popup(f, app);
      }
      ActiveReadarrBlock::AddAuthorSelectQualityProfile => {
        draw_confirmation_prompt(f, app, area);
        draw_add_author_select_quality_profile_popup(f, app);
      }
      ActiveReadarrBlock::AddAuthorSelectMetadataProfile => {
        draw_confirmation_prompt(f, app, area);
        draw_add_author_select_metadata_profile_popup(f, app);
      }
      ActiveReadarrBlock::AddAuthorSelectRootFolder => {
        draw_confirmation_prompt(f, app, area);
        draw_add_author_select_root_folder_popup(f, app);
      }
      ActiveReadarrBlock::AddAuthorPrompt | ActiveReadarrBlock::AddAuthorTagsInput => {
        draw_confirmation_prompt(f, app, area)
      }
      _ => (),
    }
  }
}

fn draw_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let searched_author = app
    .data
    .readarr_data
    .add_searched_authors
    .as_ref()
    .expect("add_searched_authors must be populated")
    .current_selection();
  let author_name = &searched_author.author_name.text;
  let author_disambiguation = searched_author.disambiguation.clone().unwrap_or_default();

  let title = build_add_author_prompt_title(author_name, &author_disambiguation);
  let yes_no_value = app.data.readarr_data.prompt_confirm;
  let selected_block = app.data.readarr_data.selected_block.get_active_block();
  let highlights = add_author_prompt_highlights(selected_block);
  let AddAuthorModal {
    monitor_list,
    monitor_new_items_list,
    quality_profile_list,
    metadata_profile_list,
    root_folder_list,
    tags,
    ..
  } = app
    .data
    .readarr_data
    .add_author_modal
    .as_ref()
    .expect("add_author_modal must exist in this context");

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
    .selected(highlights.root_folder);
  let monitor_drop_down_button = Button::default()
    .title(selected_monitor.to_display_str())
    .label("Monitor")
    .icon("▼")
    .selected(highlights.monitor);
  let monitor_new_items_drop_down_button = Button::default()
    .title(selected_monitor_new_items.to_display_str())
    .label("Monitor New Items")
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

  f.render_widget(root_folder_drop_down_button, root_folder_area);
  f.render_widget(monitor_drop_down_button, monitor_area);
  f.render_widget(monitor_new_items_drop_down_button, monitor_new_items_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(metadata_profile_drop_down_button, metadata_profile_area);

  if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(highlights.tags)
      .selected(active_readarr_block == ActiveReadarrBlock::AddAuthorTagsInput);
    render_selectable_input_box!(tags_input_box, f, tags_area);
  }

  let add_button = Button::default()
    .title("Add")
    .selected(yes_no_value && highlights.confirm);
  let cancel_button = Button::default()
    .title("Cancel")
    .selected(!yes_no_value && highlights.confirm);

  f.render_widget(add_button, add_area);
  f.render_widget(cancel_button, cancel_area);
}

fn draw_add_author_select_monitor_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_list = SelectableList::new(
    &mut app
      .data
      .readarr_data
      .add_author_modal
      .as_mut()
      .expect("add_author_modal must exist in this context")
      .monitor_list,
    |monitor| ListItem::new(monitor.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_author_select_monitor_new_items_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_new_items_list = SelectableList::new(
    &mut app
      .data
      .readarr_data
      .add_author_modal
      .as_mut()
      .expect("add_author_modal must exist in this context")
      .monitor_new_items_list,
    |monitor_new_items| ListItem::new(monitor_new_items.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_new_items_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_author_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .readarr_data
      .add_author_modal
      .as_mut()
      .expect("add_author_modal must exist in this context")
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_author_select_metadata_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let metadata_profile_list = SelectableList::new(
    &mut app
      .data
      .readarr_data
      .add_author_modal
      .as_mut()
      .expect("add_author_modal must exist in this context")
      .metadata_profile_list,
    |metadata_profile| ListItem::new(metadata_profile.clone()),
  );
  let popup = Popup::new(metadata_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_author_select_root_folder_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let root_folder_list = SelectableList::new(
    &mut app
      .data
      .readarr_data
      .add_author_modal
      .as_mut()
      .expect("add_author_modal must exist in this context")
      .root_folder_list,
    |root_folder| ListItem::new(root_folder.path.to_owned()),
  );
  let popup = Popup::new(root_folder_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
