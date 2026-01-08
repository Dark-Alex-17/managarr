use std::sync::atomic::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Cell, ListItem, Row};

use crate::models::Route;
use crate::models::radarr_models::AddMovieSearchResult;
use crate::models::servarr_data::radarr::modals::AddMovieModal;
use crate::models::servarr_data::radarr::radarr_data::{ADD_MOVIE_BLOCKS, ActiveRadarrBlock};
use crate::ui::radarr_ui::collections::CollectionsUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  get_width_from_percentage, layout_block, layout_paragraph_borderless, title_block_centered,
};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{DrawUi, draw_popup};
use crate::utils::convert_runtime;
use crate::{App, render_selectable_input_box};

#[cfg(test)]
#[path = "add_movie_ui_tests.rs"]
mod add_movie_ui_tests;

pub(super) struct AddMovieUi;

impl DrawUi for AddMovieUi {
  fn accepts(route: Route) -> bool {
    let Route::Radarr(active_radarr_block, _) = route else {
      return false;
    };
    ADD_MOVIE_BLOCKS.contains(&active_radarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = app.get_current_route() {
      if context_option.is_some() {
        CollectionsUi::draw(f, app, area);
        draw_popup(f, app, draw_confirmation_popup, Size::Medium);
      } else {
        draw_popup(f, app, draw_add_movie_search, Size::Large);

        match active_radarr_block {
          ActiveRadarrBlock::AddMoviePrompt
          | ActiveRadarrBlock::AddMovieSelectMonitor
          | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
          | ActiveRadarrBlock::AddMovieSelectQualityProfile
          | ActiveRadarrBlock::AddMovieSelectRootFolder
          | ActiveRadarrBlock::AddMovieTagsInput => {
            draw_popup(f, app, draw_confirmation_popup, Size::Medium);
          }
          ActiveRadarrBlock::AddMovieAlreadyInLibrary => {
            f.render_widget(
              Popup::new(Message::new("This film is already in your library")).size(Size::Message),
              f.area(),
            );
          }
          _ => (),
        }
      }
    }
  }
}

fn draw_add_movie_search(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let is_loading = app.is_loading || app.data.radarr_data.add_searched_movies.is_none();
  let current_selection = if let Some(add_searched_movies) =
    app.data.radarr_data.add_searched_movies.as_ref()
    && !add_searched_movies.is_empty()
  {
    add_searched_movies.current_selection().clone()
  } else {
    AddMovieSearchResult::default()
  };

  let [search_box_area, results_area] =
    Layout::vertical([Constraint::Length(3), Constraint::Fill(0)])
      .margin(1)
      .areas(area);
  let block_content = &app
    .data
    .radarr_data
    .add_movie_search
    .as_ref()
    .expect("add_movie_search must be populated")
    .text;
  let offset = app
    .data
    .radarr_data
    .add_movie_search
    .as_ref()
    .expect("add_movie_search must be populated")
    .offset
    .load(Ordering::SeqCst);
  let search_results_row_mapping = |movie: &AddMovieSearchResult| {
    let (hours, minutes) = convert_runtime(movie.runtime);
    let imdb_rating = movie
      .ratings
      .imdb
      .clone()
      .unwrap_or_default()
      .value
      .as_f64()
      .unwrap_or_default();
    let rotten_tomatoes_rating = movie
      .ratings
      .rotten_tomatoes
      .clone()
      .unwrap_or_default()
      .value
      .as_u64()
      .unwrap_or_default();
    let imdb_rating = if imdb_rating == 0.0 {
      String::new()
    } else {
      format!("{imdb_rating:.1}")
    };
    let rotten_tomatoes_rating = if rotten_tomatoes_rating == 0 {
      String::new()
    } else {
      format!("{rotten_tomatoes_rating}%")
    };
    let in_library = if app
      .data
      .radarr_data
      .movies
      .items
      .iter()
      .any(|mov| mov.tmdb_id == movie.tmdb_id)
    {
      "✔"
    } else {
      ""
    };

    movie.title.scroll_left_or_reset(
      get_width_from_percentage(area, 27),
      *movie == current_selection,
      app.ui_scroll_tick_count == 0,
    );

    Row::new(vec![
      Cell::from(in_library),
      Cell::from(movie.title.to_string()),
      Cell::from(movie.year.to_string()),
      Cell::from(format!("{hours}h {minutes}m")),
      Cell::from(imdb_rating),
      Cell::from(rotten_tomatoes_rating),
      Cell::from(movie.genres.join(", ")),
    ])
    .primary()
  };

  if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
    match active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        let search_box = InputBox::new(block_content)
          .offset(offset)
          .block(title_block_centered("Add Movie"));

        search_box.show_cursor(f, search_box_area);
        f.render_widget(layout_block().default_color(), results_area);
        f.render_widget(search_box, search_box_area);
      }
      ActiveRadarrBlock::AddMovieEmptySearchResults => {
        let error_message = Message::new("No movies found matching your query!");
        let error_message_popup = Popup::new(error_message).size(Size::Message);

        f.render_widget(layout_block().default_color(), results_area);
        f.render_widget(error_message_popup, f.area());
      }
      ActiveRadarrBlock::AddMovieSearchResults
      | ActiveRadarrBlock::AddMoviePrompt
      | ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile
      | ActiveRadarrBlock::AddMovieSelectRootFolder
      | ActiveRadarrBlock::AddMovieAlreadyInLibrary
      | ActiveRadarrBlock::AddMovieTagsInput => {
        let search_results_table = ManagarrTable::new(
          app.data.radarr_data.add_searched_movies.as_mut(),
          search_results_row_mapping,
        )
        .loading(is_loading)
        .block(layout_block().default_color())
        .headers([
          "✔",
          "Title",
          "Year",
          "Runtime",
          "IMDB",
          "Rotten Tomatoes",
          "Genres",
        ])
        .constraints([
          Constraint::Percentage(2),
          Constraint::Percentage(27),
          Constraint::Percentage(8),
          Constraint::Percentage(10),
          Constraint::Percentage(8),
          Constraint::Percentage(14),
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
      .block(title_block_centered("Add Movie")),
    search_box_area,
  );
}

fn draw_confirmation_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
    draw_confirmation_prompt(f, app, area);

    match active_radarr_block {
      ActiveRadarrBlock::AddMovieSelectMonitor => {
        draw_add_movie_select_monitor_popup(f, app);
      }
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => {
        draw_add_movie_select_minimum_availability_popup(f, app);
      }
      ActiveRadarrBlock::AddMovieSelectQualityProfile => {
        draw_add_movie_select_quality_profile_popup(f, app);
      }
      ActiveRadarrBlock::AddMovieSelectRootFolder => {
        draw_add_movie_select_root_folder_popup(f, app);
      }
      _ => (),
    }
  }
}

fn draw_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let (movie_title, movie_overview) = if let Route::Radarr(_, Some(_)) = app.get_current_route() {
    (
      &app
        .data
        .radarr_data
        .collection_movies
        .current_selection()
        .title
        .text,
      app
        .data
        .radarr_data
        .collection_movies
        .current_selection()
        .overview
        .clone(),
    )
  } else {
    (
      &app
        .data
        .radarr_data
        .add_searched_movies
        .as_ref()
        .expect("add_searched_movies must be populated")
        .current_selection()
        .title
        .text,
      app
        .data
        .radarr_data
        .add_searched_movies
        .as_ref()
        .expect("add_searched_movies must be populated")
        .current_selection()
        .overview
        .clone(),
    )
  };
  let title = format!("Add Movie - {movie_title}");
  let prompt = movie_overview;
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveRadarrBlock::AddMovieConfirmPrompt;
  let AddMovieModal {
    monitor_list,
    minimum_availability_list,
    quality_profile_list,
    root_folder_list,
    tags,
    ..
  } = app
    .data
    .radarr_data
    .add_movie_modal
    .as_ref()
    .expect("add_movie_modal must exist in this context");

  let selected_monitor = monitor_list.current_selection();
  let selected_minimum_availability = minimum_availability_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();
  let selected_root_folder = root_folder_list.current_selection();

  f.render_widget(title_block_centered(&title), area);

  let [
    paragraph_area,
    root_folder_area,
    monitor_area,
    min_availability_area,
    quality_profile_area,
    tags_area,
    _,
    buttons_area,
  ] = Layout::vertical([
    Constraint::Length(6),
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

  let root_folder_drop_down_button = Button::default()
    .title(&selected_root_folder.path)
    .label("Root Folder")
    .icon("▼")
    .selected(selected_block == ActiveRadarrBlock::AddMovieSelectRootFolder);
  let monitor_drop_down_button = Button::default()
    .title(selected_monitor.to_display_str())
    .label("Monitor")
    .icon("▼")
    .selected(selected_block == ActiveRadarrBlock::AddMovieSelectMonitor);
  let min_availability_drop_down_button = Button::default()
    .title(selected_minimum_availability.to_display_str())
    .label("Minimum Availability")
    .icon("▼")
    .selected(selected_block == ActiveRadarrBlock::AddMovieSelectMinimumAvailability);
  let quality_profile_drop_down_button = Button::default()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == ActiveRadarrBlock::AddMovieSelectQualityProfile);

  f.render_widget(root_folder_drop_down_button, root_folder_area);
  f.render_widget(monitor_drop_down_button, monitor_area);
  f.render_widget(min_availability_drop_down_button, min_availability_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);

  if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(selected_block == ActiveRadarrBlock::AddMovieTagsInput)
      .selected(active_radarr_block == ActiveRadarrBlock::AddMovieTagsInput);
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

fn draw_add_movie_select_monitor_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let monitor_list = SelectableList::new(
    &mut app
      .data
      .radarr_data
      .add_movie_modal
      .as_mut()
      .expect("add_movie_modal must exist in this context")
      .monitor_list,
    |monitor| ListItem::new(monitor.to_display_str().to_owned()),
  );
  let popup = Popup::new(monitor_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_movie_select_minimum_availability_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let minimum_availability_list = SelectableList::new(
    &mut app
      .data
      .radarr_data
      .add_movie_modal
      .as_mut()
      .expect("add_movie_modal must exist in this context")
      .minimum_availability_list,
    |minimum_availability| ListItem::new(minimum_availability.to_display_str().to_owned()),
  );
  let popup = Popup::new(minimum_availability_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_movie_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .radarr_data
      .add_movie_modal
      .as_mut()
      .expect("add_movie_modal must exist in this context")
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_add_movie_select_root_folder_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let root_folder_list = SelectableList::new(
    &mut app
      .data
      .radarr_data
      .add_movie_modal
      .as_mut()
      .expect("add_movie_modal must exist in this context")
      .root_folder_list,
    |root_folder| ListItem::new(root_folder.path.to_owned()),
  );
  let popup = Popup::new(root_folder_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
