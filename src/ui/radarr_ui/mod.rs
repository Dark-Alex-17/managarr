use std::iter;
use std::ops::Sub;

use chrono::{Duration, Utc};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Cell, ListItem, Paragraph, Row};
use tui::Frame;

use crate::app::radarr::{
  ActiveRadarrBlock, RadarrData, ADD_MOVIE_BLOCKS, COLLECTION_DETAILS_BLOCKS,
  EDIT_COLLECTION_BLOCKS, EDIT_MOVIE_BLOCKS, FILTER_BLOCKS, MOVIE_DETAILS_BLOCKS, SEARCH_BLOCKS,
};
use crate::app::App;
use crate::logos::RADARR_LOGO;
use crate::models::radarr_models::{Collection, DiskSpace, DownloadRecord, Movie, RootFolder};
use crate::models::{HorizontallyScrollableText, Route};
use crate::ui::radarr_ui::add_movie_ui::draw_add_movie_search_popup;
use crate::ui::radarr_ui::collection_details_ui::draw_collection_details_popup;
use crate::ui::radarr_ui::edit_collection_ui::draw_edit_collection_prompt;
use crate::ui::radarr_ui::edit_movie_ui::draw_edit_movie_prompt;
use crate::ui::radarr_ui::movie_details_ui::draw_movie_info_popup;
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, horizontal_chunks, layout_block,
  layout_block_top_border, line_gauge_with_label, line_gauge_with_title, show_cursor,
  style_awaiting_import, style_bold, style_default, style_failure, style_primary, style_success,
  style_unmonitored, style_warning, title_block, title_block_centered, vertical_chunks_with_margin,
};
use crate::ui::{
  draw_drop_down_list, draw_large_popup_over, draw_medium_popup_over, draw_popup, draw_popup_over,
  draw_prompt_box, draw_prompt_popup_over, draw_table, draw_tabs, loading, TableProps,
};
use crate::utils::{convert_runtime, convert_to_gb};

mod add_movie_ui;
mod collection_details_ui;
mod edit_collection_ui;
mod edit_movie_ui;
mod movie_details_ui;

pub(super) fn draw_radarr_ui<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let (content_rect, _) = draw_tabs(f, area, "Movies", &app.data.radarr_data.main_tabs);

  if let Route::Radarr(active_radarr_block, context_option) = *app.get_current_route() {
    match active_radarr_block {
      ActiveRadarrBlock::Movies => draw_library(f, app, content_rect),
      ActiveRadarrBlock::SearchMovie => {
        draw_popup_over(f, app, content_rect, draw_library, draw_search_box, 30, 11)
      }
      ActiveRadarrBlock::FilterMovies => {
        draw_popup_over(f, app, content_rect, draw_library, draw_filter_box, 30, 11)
      }
      ActiveRadarrBlock::SearchCollection => draw_popup_over(
        f,
        app,
        content_rect,
        draw_collections,
        draw_search_box,
        30,
        11,
      ),
      ActiveRadarrBlock::FilterCollections => draw_popup_over(
        f,
        app,
        content_rect,
        draw_collections,
        draw_filter_box,
        30,
        11,
      ),
      ActiveRadarrBlock::Downloads => draw_downloads(f, app, content_rect),
      ActiveRadarrBlock::RootFolders => draw_root_folders(f, app, content_rect),
      ActiveRadarrBlock::Collections => draw_collections(f, app, content_rect),
      _ if MOVIE_DETAILS_BLOCKS.contains(&active_radarr_block) => {
        draw_large_popup_over(f, app, content_rect, draw_library, draw_movie_info_popup)
      }
      _ if ADD_MOVIE_BLOCKS.contains(&active_radarr_block) => {
        if let Route::Radarr(_, Some(_)) = app.get_current_route() {
          draw_large_popup_over(
            f,
            app,
            content_rect,
            draw_collections,
            draw_add_movie_search_popup,
          )
        } else {
          draw_large_popup_over(
            f,
            app,
            content_rect,
            draw_library,
            draw_add_movie_search_popup,
          )
        }
      }
      _ if COLLECTION_DETAILS_BLOCKS.contains(&active_radarr_block) => draw_large_popup_over(
        f,
        app,
        content_rect,
        draw_collections,
        draw_collection_details_popup,
      ),
      _ if EDIT_MOVIE_BLOCKS.contains(&active_radarr_block) => {
        if let Some(context) = context_option {
          match context {
            ActiveRadarrBlock::Movies => {
              draw_medium_popup_over(f, app, content_rect, draw_library, draw_edit_movie_prompt)
            }
            _ if MOVIE_DETAILS_BLOCKS.contains(&context) => {
              draw_large_popup_over(f, app, content_rect, draw_library, draw_movie_info_popup);
              draw_popup(f, app, draw_edit_movie_prompt, 60, 60);
            }
            _ => (),
          }
        }
      }
      _ if EDIT_COLLECTION_BLOCKS.contains(&active_radarr_block) => {
        if let Some(context) = context_option {
          match context {
            ActiveRadarrBlock::Collections => draw_medium_popup_over(
              f,
              app,
              content_rect,
              draw_collections,
              draw_edit_collection_prompt,
            ),
            _ if COLLECTION_DETAILS_BLOCKS.contains(&context) => {
              draw_large_popup_over(
                f,
                app,
                content_rect,
                draw_collections,
                draw_collection_details_popup,
              );
              draw_popup(f, app, draw_edit_collection_prompt, 60, 60);
            }
            _ => (),
          }
        }
      }
      ActiveRadarrBlock::DeleteMoviePrompt => {
        draw_prompt_popup_over(f, app, content_rect, draw_library, draw_delete_movie_prompt)
      }
      ActiveRadarrBlock::DeleteDownloadPrompt => draw_prompt_popup_over(
        f,
        app,
        content_rect,
        draw_downloads,
        draw_delete_download_prompt,
      ),
      ActiveRadarrBlock::UpdateDownloadsPrompt => draw_prompt_popup_over(
        f,
        app,
        content_rect,
        draw_downloads,
        draw_update_downloads_prompt,
      ),
      ActiveRadarrBlock::UpdateAllMoviesPrompt => draw_prompt_popup_over(
        f,
        app,
        content_rect,
        draw_library,
        draw_update_all_movies_prompt,
      ),
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => draw_prompt_popup_over(
        f,
        app,
        content_rect,
        draw_collections,
        draw_update_all_collections_prompt,
      ),
      _ => (),
    }
  }
}

pub(super) fn draw_radarr_context_row<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let chunks = horizontal_chunks(vec![Constraint::Min(0), Constraint::Length(20)], area);

  let context_chunks = horizontal_chunks(
    vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)],
    chunks[0],
  );

  draw_stats_context(f, app, context_chunks[0]);
  draw_downloads_context(f, app, context_chunks[1]);
  draw_radarr_logo(f, chunks[1]);
}

fn draw_library<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let current_selection = if !app.data.radarr_data.filtered_movies.items.is_empty() {
    app
      .data
      .radarr_data
      .filtered_movies
      .current_selection()
      .clone()
  } else if !app.data.radarr_data.movies.items.is_empty() {
    app.data.radarr_data.movies.current_selection().clone()
  } else {
    Movie::default()
  };
  let quality_profile_map = &app.data.radarr_data.quality_profile_map;
  let tags_map = &app.data.radarr_data.tags_map;
  let downloads_vec = &app.data.radarr_data.downloads.items;
  let content = if !app.data.radarr_data.filtered_movies.items.is_empty()
    && !app.data.radarr_data.is_filtering
  {
    &mut app.data.radarr_data.filtered_movies
  } else {
    &mut app.data.radarr_data.movies
  };

  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content,
      table_headers: vec![
        "Title",
        "Year",
        "Studio",
        "Runtime",
        "Rating",
        "Language",
        "Size",
        "Quality Profile",
        "Monitored",
        "Tags",
      ],
      constraints: vec![
        Constraint::Percentage(27),
        Constraint::Percentage(4),
        Constraint::Percentage(17),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(10),
        Constraint::Percentage(6),
        Constraint::Percentage(12),
      ],
      help: app
        .data
        .radarr_data
        .main_tabs
        .get_active_tab_contextual_help(),
    },
    |movie| {
      movie.title.scroll_left_or_reset(
        get_width_from_percentage(area, 27),
        *movie == current_selection,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let monitored = if movie.monitored { "🏷" } else { "" };
      let (hours, minutes) = convert_runtime(movie.runtime.as_u64().unwrap());
      let file_size: f64 = convert_to_gb(movie.size_on_disk.as_u64().unwrap());
      let certification = movie.certification.clone().unwrap_or_else(|| "".to_owned());
      let quality_profile = quality_profile_map
        .get_by_left(&movie.quality_profile_id.as_u64().unwrap())
        .unwrap()
        .to_owned();
      let tags = movie
        .tags
        .iter()
        .map(|tag_id| {
          tags_map
            .get_by_left(&tag_id.as_u64().unwrap())
            .unwrap()
            .clone()
        })
        .collect::<Vec<String>>()
        .join(", ");

      Row::new(vec![
        Cell::from(movie.title.to_string()),
        Cell::from(movie.year.to_string()),
        Cell::from(movie.studio.to_string()),
        Cell::from(format!("{}h {}m", hours, minutes)),
        Cell::from(certification),
        Cell::from(movie.original_language.name.to_owned()),
        Cell::from(format!("{:.2} GB", file_size)),
        Cell::from(quality_profile),
        Cell::from(monitored.to_owned()),
        Cell::from(tags),
      ])
      .style(determine_row_style(downloads_vec, movie))
    },
    app.is_loading,
  );
}

fn draw_update_all_movies_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "Update All Movies",
    "Do you want to update info and scan your disks for all of your movies?",
    &app.data.radarr_data.prompt_confirm,
  );
}

fn draw_update_downloads_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "Update Downloads",
    "Do you want to update your downloads?",
    &app.data.radarr_data.prompt_confirm,
  );
}

fn draw_update_all_collections_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "Update All Collections",
    "Do you want to update all of your collections?",
    &app.data.radarr_data.prompt_confirm,
  );
}

fn draw_delete_movie_prompt<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, prompt_area: Rect) {
  draw_prompt_box(
    f,
    prompt_area,
    "Delete Movie",
    format!(
      "Do you really want to delete: {}?",
      app.data.radarr_data.movies.current_selection().title
    )
    .as_str(),
    &app.data.radarr_data.prompt_confirm,
  );
}

fn draw_delete_download_prompt<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, prompt_area: Rect) {
  draw_prompt_box(
    f,
    prompt_area,
    "Cancel Download",
    format!(
      "Do you really want to delete this download: {}?",
      app.data.radarr_data.downloads.current_selection().title
    )
    .as_str(),
    &app.data.radarr_data.prompt_confirm,
  );
}

fn draw_search_box<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let chunks =
    vertical_chunks_with_margin(vec![Constraint::Length(3), Constraint::Min(0)], area, 1);
  if !app.data.radarr_data.is_searching {
    let error_msg = match app.get_current_route() {
      Route::Radarr(active_radarr_block, _) => match active_radarr_block {
        ActiveRadarrBlock::SearchMovie => "Movie not found!",
        ActiveRadarrBlock::SearchCollection => "Collection not found!",
        _ => "",
      },
      _ => "",
    };

    let input = Paragraph::new(error_msg)
      .style(style_failure())
      .block(layout_block());

    f.render_widget(input, chunks[0]);
  } else {
    let default_content = String::default();
    let (block_title, offset, block_content) = match app.get_current_route() {
      Route::Radarr(active_radarr_block, _) => match active_radarr_block {
        _ if SEARCH_BLOCKS.contains(active_radarr_block) => (
          "Search",
          *app.data.radarr_data.search.offset.borrow(),
          &app.data.radarr_data.search.text,
        ),
        _ => ("", 0, &default_content),
      },
      _ => ("", 0, &default_content),
    };

    let input = Paragraph::new(block_content.as_str())
      .style(style_default())
      .block(title_block_centered(block_title));
    show_cursor(f, chunks[0], offset, block_content);

    f.render_widget(input, chunks[0]);
  }
}

fn draw_filter_box<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let chunks =
    vertical_chunks_with_margin(vec![Constraint::Length(3), Constraint::Min(0)], area, 1);
  if !app.data.radarr_data.is_filtering {
    let error_msg = match app.get_current_route() {
      Route::Radarr(active_radarr_block, _) => match active_radarr_block {
        ActiveRadarrBlock::FilterMovies => "No movies found matching filter!",
        ActiveRadarrBlock::FilterCollections => "No collections found matching filter!",
        _ => "",
      },
      _ => "",
    };

    let input = Paragraph::new(error_msg)
      .style(style_failure())
      .block(layout_block());

    f.render_widget(input, chunks[0]);
  } else {
    let default_content = String::default();
    let (block_title, offset, block_content) = match app.get_current_route() {
      Route::Radarr(active_radarr_block, _) => match active_radarr_block {
        _ if FILTER_BLOCKS.contains(active_radarr_block) => (
          "Filter",
          *app.data.radarr_data.filter.offset.borrow(),
          &app.data.radarr_data.filter.text,
        ),
        _ => ("", 0, &default_content),
      },
      _ => ("", 0, &default_content),
    };

    let input = Paragraph::new(block_content.as_str())
      .style(style_default())
      .block(title_block_centered(block_title));
    show_cursor(f, chunks[0], offset, block_content);

    f.render_widget(input, chunks[0]);
  }
}

fn draw_radarr_logo<B: Backend>(f: &mut Frame<'_, B>, area: Rect) {
  let mut logo_text = Text::from(RADARR_LOGO);
  logo_text.patch_style(Style::default().fg(Color::LightYellow));
  let logo = Paragraph::new(logo_text)
    .block(layout_block())
    .alignment(Alignment::Center);
  f.render_widget(logo, area);
}

fn draw_stats_context<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let block = title_block("Stats");

  if !app.data.radarr_data.version.is_empty() {
    f.render_widget(block, area);
    let RadarrData {
      disk_space_vec,
      start_time,
      ..
    } = &app.data.radarr_data;

    let mut constraints = vec![
      Constraint::Length(1),
      Constraint::Length(1),
      Constraint::Length(1),
    ];

    constraints.append(
      &mut iter::repeat(Constraint::Min(2))
        .take(disk_space_vec.len())
        .collect(),
    );

    let chunks = vertical_chunks_with_margin(constraints, area, 1);

    let version_paragraph = Paragraph::new(Text::from(format!(
      "Radarr Version:  {}",
      app.data.radarr_data.version
    )))
    .block(borderless_block());

    let uptime = Utc::now().sub(start_time.to_owned());
    let days = uptime.num_days();
    let day_difference = uptime.sub(Duration::days(days));
    let hours = day_difference.num_hours();
    let hour_difference = day_difference.sub(Duration::hours(hours));
    let minutes = hour_difference.num_minutes();
    let seconds = hour_difference
      .sub(Duration::minutes(minutes))
      .num_seconds();

    let uptime_paragraph = Paragraph::new(Text::from(format!(
      "Uptime: {}d {:0width$}:{:0width$}:{:0width$}",
      days,
      hours,
      minutes,
      seconds,
      width = 2
    )))
    .block(borderless_block());

    let storage =
      Paragraph::new(Text::from("Storage:")).block(borderless_block().style(style_bold()));

    f.render_widget(version_paragraph, chunks[0]);
    f.render_widget(uptime_paragraph, chunks[1]);
    f.render_widget(storage, chunks[2]);

    for i in 0..disk_space_vec.len() {
      let DiskSpace {
        free_space,
        total_space,
      } = &disk_space_vec[i];
      let title = format!("Disk {}", i + 1);
      let ratio = if total_space.as_u64().unwrap() == 0 {
        0f64
      } else {
        1f64 - (free_space.as_u64().unwrap() as f64 / total_space.as_u64().unwrap() as f64)
      };

      let space_gauge = line_gauge_with_label(title.as_str(), ratio);

      f.render_widget(space_gauge, chunks[i + 3]);
    }
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn draw_downloads_context<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let block = title_block("Downloads");
  let downloads_vec = &app.data.radarr_data.downloads.items;

  if !downloads_vec.is_empty() {
    f.render_widget(block, area);

    let constraints = iter::repeat(Constraint::Min(2))
      .take(downloads_vec.len())
      .collect::<Vec<Constraint>>();

    let chunks = vertical_chunks_with_margin(constraints, area, 1);

    for i in 0..downloads_vec.len() {
      let DownloadRecord {
        title,
        sizeleft,
        size,
        ..
      } = &downloads_vec[i];
      let percent = 1f64 - (sizeleft.as_f64().unwrap() / size.as_f64().unwrap());
      let download_gauge = line_gauge_with_title(title, percent);

      f.render_widget(download_gauge, chunks[i]);
    }
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn draw_downloads<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let current_selection = if app.data.radarr_data.downloads.items.is_empty() {
    DownloadRecord::default()
  } else {
    app.data.radarr_data.downloads.current_selection().clone()
  };

  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content: &mut app.data.radarr_data.downloads,
      table_headers: vec![
        "Title",
        "Percent Complete",
        "Size",
        "Output Path",
        "Indexer",
        "Download Client",
      ],
      constraints: vec![
        Constraint::Percentage(30),
        Constraint::Percentage(11),
        Constraint::Percentage(11),
        Constraint::Percentage(18),
        Constraint::Percentage(17),
        Constraint::Percentage(13),
      ],
      help: app
        .data
        .radarr_data
        .main_tabs
        .get_active_tab_contextual_help(),
    },
    |download_record| {
      let DownloadRecord {
        title,
        size,
        sizeleft,
        download_client,
        indexer,
        output_path,
        ..
      } = download_record;

      if matches!(output_path, Some(_)) {
        output_path.as_ref().unwrap().scroll_left_or_reset(
          get_width_from_percentage(area, 18),
          current_selection == *download_record,
          app.tick_count % app.ticks_until_scroll == 0,
        );
      }

      let percent = 1f64 - (sizeleft.as_f64().unwrap() / size.as_f64().unwrap());
      let file_size: f64 = convert_to_gb(size.as_u64().unwrap());

      Row::new(vec![
        Cell::from(title.to_owned()),
        Cell::from(format!("{:.0}%", percent * 100.0)),
        Cell::from(format!("{:.2} GB", file_size)),
        Cell::from(
          output_path
            .as_ref()
            .unwrap_or(&HorizontallyScrollableText::default())
            .to_string(),
        ),
        Cell::from(indexer.to_owned()),
        Cell::from(download_client.to_owned()),
      ])
      .style(style_primary())
    },
    app.is_loading,
  );
}

fn draw_collections<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let current_selection = if !app.data.radarr_data.filtered_collections.items.is_empty() {
    app
      .data
      .radarr_data
      .filtered_collections
      .current_selection()
      .clone()
  } else if !app.data.radarr_data.collections.items.is_empty() {
    app.data.radarr_data.collections.current_selection().clone()
  } else {
    Collection::default()
  };
  let quality_profile_map = &app.data.radarr_data.quality_profile_map;
  let content = if !app.data.radarr_data.filtered_collections.items.is_empty()
    && !app.data.radarr_data.is_filtering
  {
    &mut app.data.radarr_data.filtered_collections
  } else {
    &mut app.data.radarr_data.collections
  };
  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content,
      table_headers: vec![
        "Collection",
        "Number of Movies",
        "Root Folder Path",
        "Quality Profile",
        "Search on Add",
        "Monitored",
      ],
      constraints: vec![
        Constraint::Percentage(25),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
      ],
      help: app
        .data
        .radarr_data
        .main_tabs
        .get_active_tab_contextual_help(),
    },
    |collection| {
      let number_of_movies = collection.movies.clone().unwrap_or_default().len();
      collection.title.scroll_left_or_reset(
        get_width_from_percentage(area, 25),
        *collection == current_selection,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let monitored = if collection.monitored { "🏷" } else { "" };
      let search_on_add = if collection.search_on_add {
        "Yes"
      } else {
        "No"
      };

      Row::new(vec![
        Cell::from(collection.title.to_string()),
        Cell::from(number_of_movies.to_string()),
        Cell::from(collection.root_folder_path.clone().unwrap_or_default()),
        Cell::from(
          quality_profile_map
            .get_by_left(&collection.quality_profile_id.as_u64().unwrap())
            .unwrap()
            .to_owned(),
        ),
        Cell::from(search_on_add),
        Cell::from(monitored),
      ])
      .style(style_primary())
    },
    app.is_loading,
  );
}

fn draw_root_folders<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content: &mut app.data.radarr_data.root_folders,
      table_headers: vec!["Path", "Free Space", "Unmapped Folders"],
      constraints: vec![
        Constraint::Percentage(60),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
      ],
      help: app
        .data
        .radarr_data
        .main_tabs
        .get_active_tab_contextual_help(),
    },
    |root_folders| {
      let RootFolder {
        path,
        free_space,
        unmapped_folders,
        ..
      } = root_folders;

      let space: f64 = convert_to_gb(free_space.as_u64().unwrap());

      Row::new(vec![
        Cell::from(path.to_owned()),
        Cell::from(format!("{:.2} GB", space)),
        Cell::from(
          unmapped_folders
            .as_ref()
            .unwrap_or(&Vec::new())
            .len()
            .to_string(),
        ),
      ])
      .style(style_primary())
    },
    app.is_loading,
  );
}

fn determine_row_style(downloads_vec: &[DownloadRecord], movie: &Movie) -> Style {
  if !movie.has_file {
    if let Some(download) = downloads_vec
      .iter()
      .find(|&download| download.movie_id == movie.id)
    {
      if download.status == "downloading" {
        return style_warning();
      }

      if download.status == "completed" {
        return style_awaiting_import();
      }
    }

    return style_failure();
  }

  if !movie.monitored {
    style_unmonitored()
  } else {
    style_success()
  }
}

fn draw_select_minimum_availability_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  popup_area: Rect,
) {
  draw_drop_down_list(
    f,
    popup_area,
    &mut app.data.radarr_data.minimum_availability_list,
    |minimum_availability| ListItem::new(minimum_availability.to_display_str().to_owned()),
  );
}

fn draw_select_quality_profile_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  popup_area: Rect,
) {
  draw_drop_down_list(
    f,
    popup_area,
    &mut app.data.radarr_data.quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
}
