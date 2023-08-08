use std::iter;

use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::style::Style;
use tui::text::{Spans, Text};
use tui::widgets::{Block, Cell, Paragraph, Row, Wrap};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::radarr_models::{Credit, MovieHistoryItem};
use crate::models::Route;
use crate::ui::utils::{
  borderless_block, layout_block_bottom_border, spans_info_default, style_bold, style_default,
  style_failure, style_success, style_warning, vertical_chunks,
};
use crate::ui::{draw_table, draw_tabs, loading, TableProps};

pub(super) fn draw_movie_info<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let (content_area, block) =
    draw_tabs(f, area, "Movie Info", &app.data.radarr_data.movie_info_tabs);

  if let Route::Radarr(active_radarr_block) =
    app.data.radarr_data.movie_info_tabs.get_active_route()
  {
    match active_radarr_block {
      ActiveRadarrBlock::FileInfo => draw_file_info(f, app, content_area, block),
      ActiveRadarrBlock::MovieDetails => draw_movie_details(f, app, content_area, block),
      ActiveRadarrBlock::MovieHistory => draw_movie_history(f, app, content_area, block),
      ActiveRadarrBlock::Cast => draw_movie_cast(f, app, content_area, block),
      ActiveRadarrBlock::Crew => draw_movie_crew(f, app, content_area, block),
      _ => (),
    }
  }
}

fn draw_file_info<B: Backend>(f: &mut Frame<'_, B>, app: &App, content_area: Rect, block: Block) {
  let file_info = app.data.radarr_data.file_details.to_owned();

  if !file_info.is_empty() {
    let audio_details = app.data.radarr_data.audio_details.to_owned();
    let video_details = app.data.radarr_data.video_details.to_owned();
    let chunks = vertical_chunks(
      vec![
        Constraint::Length(1),
        Constraint::Length(5),
        Constraint::Length(1),
        Constraint::Length(6),
        Constraint::Length(1),
        Constraint::Length(7),
      ],
      content_area,
    );
    let mut file_details_title = Text::from("File Details");
    let mut audio_details_title = Text::from("Audio Details");
    let mut video_details_title = Text::from("Video Details");
    file_details_title.patch_style(style_bold());
    audio_details_title.patch_style(style_bold());
    video_details_title.patch_style(style_bold());

    let file_details_title_paragraph = Paragraph::new(file_details_title).block(borderless_block());
    let audio_details_title_paragraph =
      Paragraph::new(audio_details_title).block(borderless_block());
    let video_details_title_paragraph =
      Paragraph::new(video_details_title).block(borderless_block());

    let file_details = Text::from(file_info);
    let audio_details = Text::from(audio_details);
    let video_details = Text::from(video_details);

    let file_details_paragraph = Paragraph::new(file_details)
      .block(layout_block_bottom_border())
      .wrap(Wrap { trim: false });
    let audio_details_paragraph = Paragraph::new(audio_details)
      .block(layout_block_bottom_border())
      .wrap(Wrap { trim: false });
    let video_details_paragraph = Paragraph::new(video_details)
      .block(borderless_block())
      .wrap(Wrap { trim: false });

    f.render_widget(file_details_title_paragraph, chunks[0]);
    f.render_widget(file_details_paragraph, chunks[1]);
    f.render_widget(audio_details_title_paragraph, chunks[2]);
    f.render_widget(audio_details_paragraph, chunks[3]);
    f.render_widget(video_details_title_paragraph, chunks[4]);
    f.render_widget(video_details_paragraph, chunks[5]);
  } else {
    loading(f, block, content_area, app.is_loading);
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
    let mut text = Text::from(
      app
        .data
        .radarr_data
        .movie_details
        .items
        .iter()
        .map(|line| {
          let split = line.split(':').collect::<Vec<&str>>();
          let title = format!("{}:", split[0]);

          spans_info_default(title, split[1..].join(":"))
        })
        .collect::<Vec<Spans>>(),
    );
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

  if app.data.radarr_data.movie_history.items.is_empty() && !app.is_loading {
    let no_history_paragraph = Paragraph::new(Text::from("No history"))
      .style(style_default())
      .block(block);

    f.render_widget(no_history_paragraph, content_area);
  } else {
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
          && movie_history_item.source_title.text.len()
            > (content_area.width as f64 * 0.34) as usize
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

fn determine_style_from_download_status(download_status: &str) -> Style {
  match download_status {
    "Downloaded" => style_success(),
    "Downloading" => style_warning(),
    "Missing" => style_failure(),
    _ => style_success(),
  }
}
