use std::sync::atomic::Ordering;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Cell, ListItem, Row};
use ratatui::Frame;

use crate::models::servarr_data::sonarr::modals::AddSeriesModal;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, ADD_SERIES_BLOCKS};
use crate::models::sonarr_models::AddSeriesSearchResult;
use crate::models::Route;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  get_width_from_percentage, layout_block, layout_paragraph_borderless, title_block_centered,
};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{draw_popup, DrawUi};
use crate::{render_selectable_input_box, App};

#[cfg(test)]
#[path = "add_series_ui_tests.rs"]
mod add_series_ui_tests;

pub(super) struct AddSeriesUi;

impl DrawUi for AddSeriesUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return ADD_SERIES_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
      draw_popup(f, app, draw_add_series_search, Size::Large);

      match active_sonarr_block {
        ActiveSonarrBlock::AddSeriesPrompt
        | ActiveSonarrBlock::AddSeriesSelectMonitor
        | ActiveSonarrBlock::AddSeriesSelectSeriesType
        | ActiveSonarrBlock::AddSeriesSelectQualityProfile
        | ActiveSonarrBlock::AddSeriesSelectLanguageProfile
        | ActiveSonarrBlock::AddSeriesSelectRootFolder
        | ActiveSonarrBlock::AddSeriesTagsInput => {
          draw_popup(f, app, draw_confirmation_popup, Size::Long);
        }
        ActiveSonarrBlock::AddSeriesAlreadyInLibrary => {
          f.render_widget(
            Popup::new(Message::new("This series is already in your library")).size(Size::Message),
            f.area(),
          );
        }
        _ => (),
      }
    }
  }
}

fn draw_add_series_search(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let is_loading = app.is_loading || app.data.sonarr_data.add_searched_series.is_none();
  let current_selection =
    if let Some(add_searched_series) = app.data.sonarr_data.add_searched_series.as_ref() {
      add_searched_series.current_selection().clone()
    } else {
      AddSeriesSearchResult::default()
    };

  let [search_box_area, results_area] =
    Layout::vertical([Constraint::Length(3), Constraint::Fill(0)])
      .margin(1)
      .areas(area);
  let block_content = &app
    .data
    .sonarr_data
    .add_series_search
    .as_ref()
    .unwrap()
    .text;
  let offset = app
    .data
    .sonarr_data
    .add_series_search
    .as_ref()
    .unwrap()
    .offset
    .load(Ordering::SeqCst);
  let search_results_row_mapping = |series: &AddSeriesSearchResult| {
    let rating = series.ratings.clone().unwrap_or_default().value;
    let series_rating = if rating == 0.0 {
      String::new()
    } else {
      format!("{rating:.1}")
    };
    let in_library = if app
      .data
      .sonarr_data
      .series
      .items
      .iter()
      .any(|mov| mov.tvdb_id == series.tvdb_id)
    {
      "✔"
    } else {
      ""
    };
    let network = series.network.clone().unwrap_or_default();
    let seasons = if let Some(ref stats) = series.statistics {
      format!("{}", stats.season_count)
    } else {
      String::new()
    };

    series.title.scroll_left_or_reset(
      get_width_from_percentage(area, 27),
      *series == current_selection,
      app.tick_count % app.ticks_until_scroll == 0,
    );

    Row::new(vec![
      Cell::from(in_library),
      Cell::from(series.title.to_string()),
      Cell::from(series.year.to_string()),
      Cell::from(network),
      Cell::from(series_rating),
      Cell::from(seasons),
      Cell::from(series.genres.join(", ")),
    ])
    .primary()
  };

  if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
    match active_sonarr_block {
      ActiveSonarrBlock::AddSeriesSearchInput => {
        let search_box = InputBox::new(block_content)
          .offset(offset)
          .block(title_block_centered("Add Series"));

        search_box.show_cursor(f, search_box_area);
        f.render_widget(layout_block().default(), results_area);
        f.render_widget(search_box, search_box_area);
      }
      ActiveSonarrBlock::AddSeriesEmptySearchResults => {
        let error_message = Message::new("No series found matching your query!");
        let error_message_popup = Popup::new(error_message).size(Size::Message);

        f.render_widget(layout_block().default(), results_area);
        f.render_widget(error_message_popup, f.area());
      }
      ActiveSonarrBlock::AddSeriesSearchResults
      | ActiveSonarrBlock::AddSeriesPrompt
      | ActiveSonarrBlock::AddSeriesSelectMonitor
      | ActiveSonarrBlock::AddSeriesSelectSeriesType
      | ActiveSonarrBlock::AddSeriesSelectQualityProfile
      | ActiveSonarrBlock::AddSeriesSelectLanguageProfile
      | ActiveSonarrBlock::AddSeriesSelectRootFolder
      | ActiveSonarrBlock::AddSeriesAlreadyInLibrary
      | ActiveSonarrBlock::AddSeriesTagsInput => {
        let search_results_table = ManagarrTable::new(
          app.data.sonarr_data.add_searched_series.as_mut(),
          search_results_row_mapping,
        )
        .loading(is_loading)
        .block(layout_block().default())
        .headers([
          "✔", "Title", "Year", "Network", "Seasons", "Rating", "Genres",
        ])
        .constraints([
          Constraint::Percentage(2),
          Constraint::Percentage(27),
          Constraint::Percentage(9),
          Constraint::Percentage(13),
          Constraint::Percentage(9),
          Constraint::Percentage(9),
          Constraint::Percentage(28),
        ]);

        f.render_widget(search_results_table, results_area);
      }
      _ => (),
    }
  }

  f.render_widget(
    InputBox::new(block_content)
      .offset(offset)
      .block(title_block_centered("Add Series")),
    search_box_area,
  );
}

fn draw_confirmation_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
    match active_sonarr_block {
      ActiveSonarrBlock::AddSeriesSelectMonitor => {
        draw_confirmation_prompt(f, app, area);
        draw_add_series_select_monitor_popup(f, app);
      }
      ActiveSonarrBlock::AddSeriesSelectSeriesType => {
        draw_confirmation_prompt(f, app, area);
        draw_add_series_select_series_type_popup(f, app);
      }
      ActiveSonarrBlock::AddSeriesSelectQualityProfile => {
        draw_confirmation_prompt(f, app, area);
        draw_add_series_select_quality_profile_popup(f, app);
      }
      ActiveSonarrBlock::AddSeriesSelectLanguageProfile => {
        draw_confirmation_prompt(f, app, area);
        draw_add_series_select_language_profile_popup(f, app);
      }
      ActiveSonarrBlock::AddSeriesSelectRootFolder => {
        draw_confirmation_prompt(f, app, area);
        draw_add_series_select_root_folder_popup(f, app);
      }
      ActiveSonarrBlock::AddSeriesPrompt | ActiveSonarrBlock::AddSeriesTagsInput => {
        draw_confirmation_prompt(f, app, area)
      }
      _ => (),
    }
  }
}

fn draw_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let (series_title, series_overview) = (
    &app
      .data
      .sonarr_data
      .add_searched_series
      .as_ref()
      .unwrap()
      .current_selection()
      .title
      .text,
    app
      .data
      .sonarr_data
      .add_searched_series
      .as_ref()
      .unwrap()
      .current_selection()
      .overview
      .clone()
      .unwrap_or_default(),
  );
  let title = format!("Add Series - {series_title}");
  let prompt = series_overview;
  let yes_no_value = app.data.sonarr_data.prompt_confirm;
  let selected_block = app.data.sonarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveSonarrBlock::AddSeriesConfirmPrompt;
  let AddSeriesModal {
    monitor_list,
    series_type_list,
    quality_profile_list,
    language_profile_list,
    root_folder_list,
    use_season_folder,
    tags,
    ..
  } = app.data.sonarr_data.add_series_modal.as_ref().unwrap();

  let selected_monitor = monitor_list.current_selection();
  let selected_series_type = series_type_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();
  let selected_language_profile = language_profile_list.current_selection();
  let selected_root_folder = root_folder_list.current_selection();

  f.render_widget(title_block_centered(&title), area);

  let [paragraph_area, root_folder_area, monitor_area, quality_profile_area, language_profile_area, series_type_area, season_folder_area, tags_area, _, buttons_area] =
    Layout::vertical([
      Constraint::Length(6),
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

  let prompt_paragraph = layout_paragraph_borderless(&prompt);
  f.render_widget(prompt_paragraph, paragraph_area);

  let [add_area, cancel_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  let use_season_folder_checkbox = Checkbox::new("Season Folder")
    .checked(*use_season_folder)
    .highlighted(selected_block == ActiveSonarrBlock::AddSeriesToggleUseSeasonFolder);
  let root_folder_drop_down_button = Button::new()
    .title(&selected_root_folder.path)
    .label("Root Folder")
    .icon("▼")
    .selected(selected_block == ActiveSonarrBlock::AddSeriesSelectRootFolder);
  let monitor_drop_down_button = Button::new()
    .title(selected_monitor.to_display_str())
    .label("Monitor")
    .icon("▼")
    .selected(selected_block == ActiveSonarrBlock::AddSeriesSelectMonitor);
  let series_type_drop_down_button = Button::new()
    .title(selected_series_type.to_display_str())
    .label("Series Type")
    .icon("▼")
    .selected(selected_block == ActiveSonarrBlock::AddSeriesSelectSeriesType);
  let quality_profile_drop_down_button = Button::new()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == ActiveSonarrBlock::AddSeriesSelectQualityProfile);
  let language_profile_drop_down_button = Button::new()
    .title(selected_language_profile)
    .label("Language Profile")
    .icon("▼")
    .selected(selected_block == ActiveSonarrBlock::AddSeriesSelectLanguageProfile);

  f.render_widget(root_folder_drop_down_button, root_folder_area);
  f.render_widget(monitor_drop_down_button, monitor_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(language_profile_drop_down_button, language_profile_area);
  f.render_widget(series_type_drop_down_button, series_type_area);
  f.render_widget(use_season_folder_checkbox, season_folder_area);

  if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(selected_block == ActiveSonarrBlock::AddSeriesTagsInput)
      .selected(active_sonarr_block == ActiveSonarrBlock::AddSeriesTagsInput);
    render_selectable_input_box!(tags_input_box, f, tags_area);
  }

  let add_button = Button::new()
    .title("Add")
    .selected(yes_no_value && highlight_yes_no);
  let cancel_button = Button::new()
    .title("Cancel")
    .selected(!yes_no_value && highlight_yes_no);

  f.render_widget(add_button, add_area);
  f.render_widget(cancel_button, cancel_area);
}

fn draw_add_series_select_monitor_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_list = SelectableList::new(
    &mut app
      .data
      .sonarr_data
      .add_series_modal
      .as_mut()
      .unwrap()
      .monitor_list,
    |monitor| ListItem::new(monitor.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_series_select_series_type_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let series_type_list = SelectableList::new(
    &mut app
      .data
      .sonarr_data
      .add_series_modal
      .as_mut()
      .unwrap()
      .series_type_list,
    |series_type| ListItem::new(series_type.to_display_str().to_owned()),
  );
  let popup = Popup::new(series_type_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_series_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .sonarr_data
      .add_series_modal
      .as_mut()
      .unwrap()
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_series_select_language_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let language_profile_list = SelectableList::new(
    &mut app
      .data
      .sonarr_data
      .add_series_modal
      .as_mut()
      .unwrap()
      .language_profile_list,
    |language_profile| ListItem::new(language_profile.clone()),
  );
  let popup = Popup::new(language_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_series_select_root_folder_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let root_folder_list = SelectableList::new(
    &mut app
      .data
      .sonarr_data
      .add_series_modal
      .as_mut()
      .unwrap()
      .root_folder_list,
    |root_folder| ListItem::new(root_folder.path.to_owned()),
  );
  let popup = Popup::new(root_folder_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
