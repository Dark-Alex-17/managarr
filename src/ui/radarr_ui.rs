use std::iter;
use std::ops::Sub;

use chrono::{Duration, Utc};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Cell, Paragraph, Row, Wrap};
use tui::Frame;

use crate::app::radarr::{ActiveRadarrBlock, RadarrData};
use crate::app::{App, Route};
use crate::logos::RADARR_LOGO;
use crate::network::radarr_network::{Credit, DiskSpace, DownloadRecord, Movie, MovieHistoryItem};
use crate::ui::utils::{
  horizontal_chunks, layout_block_top_border, line_gauge_with_label, line_gauge_with_title,
  style_bold, style_failure, style_success, style_warning, title_block,
  vertical_chunks_with_margin,
};
use crate::ui::{draw_large_popup_over, draw_table, draw_tabs, loading, TableProps};
use crate::utils::{convert_runtime, convert_to_gb};

pub(super) fn draw_radarr_ui<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let (content_rect, _) = draw_tabs(f, area, "  Movies  ", &app.data.radarr_data.main_tabs);

  if let Route::Radarr(active_radarr_block) = app.get_current_route() {
    match active_radarr_block {
      ActiveRadarrBlock::Movies => draw_library(f, app, content_rect),
      ActiveRadarrBlock::Downloads => draw_downloads(f, app, content_rect),
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew => {
        draw_large_popup_over(f, app, content_rect, draw_library, draw_movie_info)
      }
      _ => (),
    }
  }
}

pub(super) fn draw_radarr_context_row<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let chunks = horizontal_chunks(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)], area);

  draw_stats_context(f, app, chunks[0]);
  draw_downloads_context(f, app, chunks[1]);
}

fn draw_library<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let quality_profile_map = &app.data.radarr_data.quality_profile_map;
  let downloads_vec = &app.data.radarr_data.downloads.items;

  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content: &mut app.data.radarr_data.movies,
      table_headers: vec![
        "Title",
        "Year",
        "Runtime",
        "Rating",
        "Language",
        "Size",
        "Quality Profile",
      ],
      constraints: vec![
        Constraint::Percentage(25),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
      ],
    },
    |movie| {
      let (hours, minutes) = convert_runtime(movie.runtime.as_u64().unwrap());
      let file_size: f64 = convert_to_gb(movie.size_on_disk.as_u64().unwrap());
      let certification = movie.certification.clone().unwrap_or_else(|| "".to_owned());

      Row::new(vec![
        Cell::from(movie.title.to_owned()),
        Cell::from(movie.year.to_string()),
        Cell::from(format!("{}h {}m", hours, minutes)),
        Cell::from(certification),
        Cell::from(movie.original_language.name.to_owned()),
        Cell::from(format!("{:.2} GB", file_size)),
        Cell::from(
          quality_profile_map
            .get(&movie.quality_profile_id.as_u64().unwrap())
            .unwrap()
            .to_owned(),
        ),
      ])
      .style(determine_row_style(downloads_vec, movie))
    },
    app.is_loading,
  );
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
      let download_gague = line_gauge_with_title(title, percent);

      f.render_widget(download_gague, chunks[i]);
    }
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn draw_downloads<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let current_selection = if app.data.radarr_data.downloads.items.is_empty() {
    DownloadRecord::default()
  } else {
    app.data.radarr_data.downloads.current_selection_clone()
  };
  let width = (area.width as f32 * 0.30) as usize;

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

      if current_selection == *download_record && output_path.text.len() > width {
        output_path.scroll_text()
      } else {
        output_path.reset_offset();
      }

      let percent = 1f64 - (sizeleft.as_f64().unwrap() / size.as_f64().unwrap());
      let file_size: f64 = convert_to_gb(size.as_u64().unwrap());

      Row::new(vec![
        Cell::from(title.to_owned()),
        Cell::from(format!("{:.0}%", percent * 100.0)),
        Cell::from(format!("{:.2} GB", file_size)),
        Cell::from(output_path.to_string()),
        Cell::from(indexer.to_owned()),
        Cell::from(download_client.to_owned()),
      ])
      .style(style_success())
    },
    app.is_loading,
  );
}

fn draw_movie_info<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let (content_area, block) =
    draw_tabs(f, area, "Movie Info", &app.data.radarr_data.movie_info_tabs);

  if let Route::Radarr(active_radarr_block) =
    app.data.radarr_data.movie_info_tabs.get_active_route()
  {
    match active_radarr_block {
      ActiveRadarrBlock::MovieDetails => draw_movie_details(f, app, content_area, block),
      ActiveRadarrBlock::MovieHistory => draw_movie_history(f, app, content_area, block),
      ActiveRadarrBlock::Cast => draw_movie_cast(f, app, content_area, block),
      ActiveRadarrBlock::Crew => draw_movie_crew(f, app, content_area, block),
      _ => (),
    }
  }
}

fn draw_movie_details<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &App,
  content_area: Rect,
  block: Block,
) {
  let movie_details = app.data.radarr_data.movie_details.get_text();

  if !movie_details.is_empty() {
    let download_status = app
      .data
      .radarr_data
      .movie_details
      .items
      .iter()
      .find(|&line| line.starts_with("Status: "))
      .unwrap()
      .split(": ")
      .collect::<Vec<&str>>()[1];
    let mut text = Text::from(movie_details);
    text.patch_style(determine_style_from_download_status(download_status));

    let paragraph = Paragraph::new(text)
      .block(block)
      .wrap(Wrap { trim: false })
      .scroll((app.data.radarr_data.movie_details.offset, 0));

    f.render_widget(paragraph, content_area);
  } else {
    loading(f, block, content_area, app.is_loading);
  }
}

fn draw_movie_history<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  content_area: Rect,
  block: Block,
) {
  let current_selection = if app.data.radarr_data.movie_history.items.is_empty() {
    MovieHistoryItem::default()
  } else {
    app.data.radarr_data.movie_history.current_selection_clone()
  };

  draw_table(
    f,
    content_area,
    block,
    TableProps {
      content: &mut app.data.radarr_data.movie_history,
      table_headers: vec!["Source Title", "Event Type", "Languages", "Quality", "Date"],
      constraints: vec![
        Constraint::Percentage(34),
        Constraint::Percentage(17),
        Constraint::Percentage(14),
        Constraint::Percentage(14),
        Constraint::Percentage(21),
      ],
    },
    |movie_history_item| {
      let MovieHistoryItem {
        source_title,
        quality,
        languages,
        date,
        event_type,
      } = movie_history_item;

      if current_selection == *movie_history_item
        && movie_history_item.source_title.text.len() > (content_area.width as f64 * 0.34) as usize
      {
        source_title.scroll_text();
      } else {
        source_title.reset_offset();
      }

      Row::new(vec![
        Cell::from(source_title.to_string()),
        Cell::from(event_type.to_owned()),
        Cell::from(
          languages
            .iter()
            .map(|language| language.name.to_owned())
            .collect::<Vec<String>>()
            .join(","),
        ),
        Cell::from(quality.quality.name.to_owned()),
        Cell::from(date.to_string()),
      ])
      .style(style_success())
    },
    app.is_loading,
  );
}

fn draw_movie_cast<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  content_area: Rect,
  block: Block,
) {
  draw_table(
    f,
    content_area,
    block,
    TableProps {
      content: &mut app.data.radarr_data.movie_cast,
      constraints: iter::repeat(Constraint::Ratio(1, 2)).take(2).collect(),
      table_headers: vec!["Cast Member", "Character"],
    },
    |cast_member| {
      let Credit {
        person_name,
        character,
        ..
      } = cast_member;

      Row::new(vec![
        Cell::from(person_name.to_owned()),
        Cell::from(character.clone().unwrap_or_default()),
      ])
      .style(style_success())
    },
    app.is_loading,
  )
}

fn draw_movie_crew<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  content_area: Rect,
  block: Block,
) {
  draw_table(
    f,
    content_area,
    block,
    TableProps {
      content: &mut app.data.radarr_data.movie_crew,
      constraints: iter::repeat(Constraint::Ratio(1, 3)).take(3).collect(),
      table_headers: vec!["Crew Member", "Job", "Department"],
    },
    |crew_member| {
      let Credit {
        person_name,
        job,
        department,
        ..
      } = crew_member;

      Row::new(vec![
        Cell::from(person_name.to_owned()),
        Cell::from(job.clone().unwrap_or_default()),
        Cell::from(department.clone().unwrap_or_default()),
      ])
      .style(style_success())
    },
    app.is_loading,
  );
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
      Constraint::Percentage(60),
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
    .block(Block::default());

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
    .block(Block::default());

    let mut logo_text = Text::from(RADARR_LOGO);
    logo_text.patch_style(Style::default().fg(Color::LightYellow));
    let logo = Paragraph::new(logo_text)
      .block(Block::default())
      .alignment(Alignment::Center);
    let storage =
      Paragraph::new(Text::from("Storage:")).block(Block::default().style(style_bold()));

    f.render_widget(logo, chunks[0]);
    f.render_widget(version_paragraph, chunks[1]);
    f.render_widget(uptime_paragraph, chunks[2]);
    f.render_widget(storage, chunks[3]);

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

      f.render_widget(space_gauge, chunks[i + 4]);
    }
  } else {
    loading(f, block, area, app.is_loading);
  }
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
    }

    return style_failure();
  }

  style_success()
}

fn determine_style_from_download_status(download_status: &str) -> Style {
  match download_status {
    "Downloaded" => style_success(),
    "Downloading" => style_warning(),
    "Missing" => style_failure(),
    _ => style_success(),
  }
}
