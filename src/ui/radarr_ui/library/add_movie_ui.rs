use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Cell, ListItem, Paragraph, Row};
use ratatui::Frame;

use crate::app::context_clues::{build_context_clue_string, BARE_POPUP_CONTEXT_CLUES};
use crate::app::radarr::radarr_context_clues::ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES;
use crate::models::radarr_models::AddMovieSearchResult;
use crate::models::servarr_data::radarr::modals::AddMovieModal;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, ADD_MOVIE_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::collections::{draw_collection_details, draw_collections};
use crate::ui::radarr_ui::library::draw_library;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, layout_block, layout_paragraph_borderless,
  title_block_centered,
};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::{
  draw_drop_down_popup, draw_error_popup, draw_error_popup_over, draw_large_popup_over,
  draw_medium_popup_over, draw_selectable_list, draw_table, DrawUi, TableProps,
};
use crate::utils::convert_runtime;
use crate::{render_selectable_input_box, App};

#[cfg(test)]
#[path = "add_movie_ui_tests.rs"]
mod add_movie_ui_tests;

pub(super) struct AddMovieUi;

impl DrawUi for AddMovieUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return ADD_MOVIE_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = *app.get_current_route() {
      let draw_add_movie_search_popup =
        |f: &mut Frame<'_>, app: &mut App<'_>, area: Rect| match active_radarr_block {
          ActiveRadarrBlock::AddMovieSearchInput
          | ActiveRadarrBlock::AddMovieSearchResults
          | ActiveRadarrBlock::AddMovieEmptySearchResults => {
            draw_add_movie_search(f, app, area);
          }
          ActiveRadarrBlock::AddMoviePrompt
          | ActiveRadarrBlock::AddMovieSelectMonitor
          | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
          | ActiveRadarrBlock::AddMovieSelectQualityProfile
          | ActiveRadarrBlock::AddMovieSelectRootFolder
          | ActiveRadarrBlock::AddMovieTagsInput => {
            if context_option.is_some() {
              draw_medium_popup_over(
                f,
                app,
                area,
                draw_collection_details,
                draw_confirmation_popup,
              );
            } else {
              draw_medium_popup_over(f, app, area, draw_add_movie_search, draw_confirmation_popup);
            }
          }
          ActiveRadarrBlock::AddMovieAlreadyInLibrary => draw_error_popup_over(
            f,
            app,
            area,
            "This film is already in your library",
            draw_add_movie_search,
          ),
          _ => (),
        };

      match active_radarr_block {
        _ if ADD_MOVIE_BLOCKS.contains(&active_radarr_block) => {
          if context_option.is_some() {
            draw_large_popup_over(f, app, area, draw_collections, draw_add_movie_search_popup)
          } else {
            draw_large_popup_over(f, app, area, draw_library, draw_add_movie_search_popup)
          }
        }
        _ => (),
      }
    }
  }
}

fn draw_add_movie_search(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let is_loading = app.is_loading || app.data.radarr_data.add_searched_movies.is_none();
  let current_selection =
    if let Some(add_searched_movies) = app.data.radarr_data.add_searched_movies.as_ref() {
      add_searched_movies.current_selection().clone()
    } else {
      AddMovieSearchResult::default()
    };

  let [search_box_area, results_area, help_area] = Layout::vertical([
    Constraint::Length(3),
    Constraint::Fill(0),
    Constraint::Length(3),
  ])
  .margin(1)
  .areas(area);
  let block_content = &app.data.radarr_data.search.as_ref().unwrap().text;
  let offset = *app
    .data
    .radarr_data
    .search
    .as_ref()
    .unwrap()
    .offset
    .borrow();

  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    match active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        let search_box = InputBox::new(block_content)
          .offset(offset)
          .block(title_block_centered("Add Movie"));
        let help_text = Text::from(build_context_clue_string(&BARE_POPUP_CONTEXT_CLUES).help());
        let help_paragraph = Paragraph::new(help_text)
          .block(borderless_block())
          .alignment(Alignment::Center);

        search_box.show_cursor(f, search_box_area);
        f.render_widget(layout_block(), results_area);
        f.render_widget(search_box, search_box_area);
        f.render_widget(help_paragraph, help_area);
      }
      ActiveRadarrBlock::AddMovieEmptySearchResults => {
        f.render_widget(layout_block(), results_area);
        draw_error_popup(f, "No movies found matching your query!");
      }
      ActiveRadarrBlock::AddMovieSearchResults
      | ActiveRadarrBlock::AddMoviePrompt
      | ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile
      | ActiveRadarrBlock::AddMovieSelectRootFolder
      | ActiveRadarrBlock::AddMovieAlreadyInLibrary
      | ActiveRadarrBlock::AddMovieTagsInput => {
        let help_text =
          Text::from(build_context_clue_string(&ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES).help());
        let help_paragraph = Paragraph::new(help_text)
          .block(borderless_block())
          .alignment(Alignment::Center);
        f.render_widget(help_paragraph, help_area);

        draw_table(
          f,
          results_area,
          layout_block(),
          TableProps {
            content: None,
            wrapped_content: Some(app.data.radarr_data.add_searched_movies.as_mut()),
            table_headers: vec![
              "✔",
              "Title",
              "Year",
              "Runtime",
              "IMDB",
              "Rotten Tomatoes",
              "Genres",
            ],
            constraints: vec![
              Constraint::Percentage(2),
              Constraint::Percentage(27),
              Constraint::Percentage(8),
              Constraint::Percentage(10),
              Constraint::Percentage(8),
              Constraint::Percentage(14),
              Constraint::Percentage(28),
            ],
            help: None,
          },
          |movie| {
            let (hours, minutes) = convert_runtime(movie.runtime);
            let imdb_rating = movie
              .ratings
              .imdb
              .clone()
              .unwrap_or_default()
              .value
              .as_f64()
              .unwrap();
            let rotten_tomatoes_rating = movie
              .ratings
              .rotten_tomatoes
              .clone()
              .unwrap_or_default()
              .value
              .as_u64()
              .unwrap();
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
              app.tick_count % app.ticks_until_scroll == 0,
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
          },
          is_loading,
          true,
        );
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
  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    match active_radarr_block {
      ActiveRadarrBlock::AddMovieSelectMonitor => {
        draw_drop_down_popup(
          f,
          app,
          area,
          draw_confirmation_prompt,
          draw_add_movie_select_monitor_popup,
        );
      }
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => {
        draw_drop_down_popup(
          f,
          app,
          area,
          draw_confirmation_prompt,
          draw_add_movie_select_minimum_availability_popup,
        );
      }
      ActiveRadarrBlock::AddMovieSelectQualityProfile => {
        draw_drop_down_popup(
          f,
          app,
          area,
          draw_confirmation_prompt,
          draw_add_movie_select_quality_profile_popup,
        );
      }
      ActiveRadarrBlock::AddMovieSelectRootFolder => {
        draw_drop_down_popup(
          f,
          app,
          area,
          draw_confirmation_prompt,
          draw_add_movie_select_root_folder_popup,
        );
      }
      ActiveRadarrBlock::AddMoviePrompt | ActiveRadarrBlock::AddMovieTagsInput => {
        draw_confirmation_prompt(f, app, area)
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
        .unwrap()
        .current_selection()
        .title
        .text,
      app
        .data
        .radarr_data
        .add_searched_movies
        .as_ref()
        .unwrap()
        .current_selection()
        .overview
        .clone(),
    )
  };
  let title = format!("Add Movie - {movie_title}");
  let prompt = movie_overview;
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == &ActiveRadarrBlock::AddMovieConfirmPrompt;
  let AddMovieModal {
    monitor_list,
    minimum_availability_list,
    quality_profile_list,
    root_folder_list,
    tags,
    ..
  } = app.data.radarr_data.add_movie_modal.as_ref().unwrap();

  let selected_monitor = monitor_list.current_selection();
  let selected_minimum_availability = minimum_availability_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();
  let selected_root_folder = root_folder_list.current_selection();

  f.render_widget(title_block_centered(&title), area);

  let [paragraph_area, root_folder_area, monitor_area, min_availability_area, quality_profile_area, tags_area, _, buttons_area] =
    Layout::vertical([
      Constraint::Length(6),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(0),
      Constraint::Length(3),
    ])
    .margin(1)
    .areas(area);

  let prompt_paragraph = layout_paragraph_borderless(&prompt);
  f.render_widget(prompt_paragraph, paragraph_area);

  let [add_area, cancel_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  let root_folder_drop_down_button = Button::new()
    .title(&selected_root_folder.path)
    .label("Root Folder")
    .icon("▼")
    .selected(selected_block == &ActiveRadarrBlock::AddMovieSelectRootFolder);
  let monitor_drop_down_button = Button::new()
    .title(selected_monitor.to_display_str())
    .label("Monitor")
    .icon("▼")
    .selected(selected_block == &ActiveRadarrBlock::AddMovieSelectMonitor);
  let min_availability_drop_down_button = Button::new()
    .title(selected_minimum_availability.to_display_str())
    .label("Minimum Availability")
    .icon("▼")
    .selected(selected_block == &ActiveRadarrBlock::AddMovieSelectMinimumAvailability);
  let quality_profile_drop_down_button = Button::new()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == &ActiveRadarrBlock::AddMovieSelectQualityProfile);

  f.render_widget(root_folder_drop_down_button, root_folder_area);
  f.render_widget(monitor_drop_down_button, monitor_area);
  f.render_widget(min_availability_drop_down_button, min_availability_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);

  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    let tags_input_box = InputBox::new(&tags.text)
      .offset(*tags.offset.borrow())
      .label("Tags")
      .highlighted(selected_block == &ActiveRadarrBlock::AddMovieTagsInput)
      .selected(active_radarr_block == ActiveRadarrBlock::AddMovieTagsInput);
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

fn draw_add_movie_select_monitor_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_selectable_list(
    f,
    area,
    &mut app
      .data
      .radarr_data
      .add_movie_modal
      .as_mut()
      .unwrap()
      .monitor_list,
    |monitor| ListItem::new(monitor.to_display_str().to_owned()),
  );
}

fn draw_add_movie_select_minimum_availability_popup(
  f: &mut Frame<'_>,
  app: &mut App<'_>,
  area: Rect,
) {
  draw_selectable_list(
    f,
    area,
    &mut app
      .data
      .radarr_data
      .add_movie_modal
      .as_mut()
      .unwrap()
      .minimum_availability_list,
    |minimum_availability| ListItem::new(minimum_availability.to_display_str().to_owned()),
  );
}

fn draw_add_movie_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_selectable_list(
    f,
    area,
    &mut app
      .data
      .radarr_data
      .add_movie_modal
      .as_mut()
      .unwrap()
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
}

fn draw_add_movie_select_root_folder_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_selectable_list(
    f,
    area,
    &mut app
      .data
      .radarr_data
      .add_movie_modal
      .as_mut()
      .unwrap()
      .root_folder_list,
    |root_folder| ListItem::new(root_folder.path.to_owned()),
  );
}
