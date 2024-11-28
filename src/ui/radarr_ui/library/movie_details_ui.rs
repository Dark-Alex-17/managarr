use std::iter;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use ratatui::Frame;

use crate::app::App;
use crate::models::radarr_models::{Credit, MovieHistoryItem, RadarrRelease};
use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, MOVIE_DETAILS_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::library::draw_library;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, layout_block_bottom_border, layout_block_top_border,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{draw_popup_over, draw_tabs, DrawUi};
use crate::utils::convert_to_gb;

#[cfg(test)]
#[path = "movie_details_ui_tests.rs"]
mod movie_details_ui_tests;

pub(super) struct MovieDetailsUi;

impl DrawUi for MovieDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return MOVIE_DETAILS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = app.get_current_route() {
      let draw_movie_info_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        let content_area = draw_tabs(
          f,
          popup_area,
          "Movie Info",
          &app.data.radarr_data.movie_info_tabs,
        );
        draw_movie_info(f, app, content_area);

        match context_option.unwrap_or(active_radarr_block) {
          ActiveRadarrBlock::AutomaticallySearchMoviePrompt => {
            let prompt = format!(
              "Do you want to trigger an automatic search of your indexers for the movie: {}?",
              app.data.radarr_data.movies.current_selection().title
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Automatic Movie Search")
              .prompt(&prompt)
              .yes_no_value(app.data.radarr_data.prompt_confirm);

            draw_movie_info(f, app, content_area);
            f.render_widget(Popup::new(confirmation_prompt).size(Size::Prompt), f.area());
          }
          ActiveRadarrBlock::UpdateAndScanPrompt => {
            let prompt = format!(
              "Do you want to trigger an update and disk scan for the movie: {}?",
              app.data.radarr_data.movies.current_selection().title
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Update and Scan")
              .prompt(&prompt)
              .yes_no_value(app.data.radarr_data.prompt_confirm);

            f.render_widget(Popup::new(confirmation_prompt).size(Size::Prompt), f.area());
          }
          ActiveRadarrBlock::ManualSearchConfirmPrompt => {
            draw_manual_search_confirm_prompt(f, app);
          }
          _ => (),
        }
      };

      draw_popup_over(
        f,
        app,
        area,
        draw_library,
        draw_movie_info_popup,
        Size::Large,
      );
    }
  }
}

fn draw_movie_info(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Radarr(active_radarr_block, _) =
    app.data.radarr_data.movie_info_tabs.get_active_route()
  {
    match active_radarr_block {
      ActiveRadarrBlock::FileInfo => draw_file_info(f, app, area),
      ActiveRadarrBlock::MovieDetails => draw_movie_details(f, app, area),
      ActiveRadarrBlock::MovieHistory => draw_movie_history(f, app, area),
      ActiveRadarrBlock::Cast => draw_movie_cast(f, app, area),
      ActiveRadarrBlock::Crew => draw_movie_crew(f, app, area),
      ActiveRadarrBlock::ManualSearch => draw_movie_releases(f, app, area),
      _ => (),
    }
  }
}

fn draw_file_info(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  match app.data.radarr_data.movie_details_modal.as_ref() {
    Some(movie_details_modal)
      if !movie_details_modal.file_details.is_empty() && !app.is_loading =>
    {
      let file_info = movie_details_modal.file_details.to_owned();
      let audio_details = movie_details_modal.audio_details.to_owned();
      let video_details = movie_details_modal.video_details.to_owned();
      let [file_details_title_area, file_details_area, audio_details_title_area, audio_details_area, video_details_title_area, video_details_area] =
        Layout::vertical([
          Constraint::Length(2),
          Constraint::Length(5),
          Constraint::Length(1),
          Constraint::Length(6),
          Constraint::Length(1),
          Constraint::Length(7),
        ])
        .areas(area);

      let file_details_title_paragraph =
        Paragraph::new("File Details".bold()).block(layout_block_top_border());
      let audio_details_title_paragraph =
        Paragraph::new("Audio Details".bold()).block(borderless_block());
      let video_details_title_paragraph =
        Paragraph::new("Video Details".bold()).block(borderless_block());

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

      f.render_widget(file_details_title_paragraph, file_details_title_area);
      f.render_widget(file_details_paragraph, file_details_area);
      f.render_widget(audio_details_title_paragraph, audio_details_title_area);
      f.render_widget(audio_details_paragraph, audio_details_area);
      f.render_widget(video_details_title_paragraph, video_details_title_area);
      f.render_widget(video_details_paragraph, video_details_area);
    }
    _ => f.render_widget(
      LoadingBlock::new(app.is_loading, layout_block_top_border()),
      area,
    ),
  }
}

fn draw_movie_details(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  let block = layout_block_top_border();
  let unknown_download_status = "Status: Unknown".to_owned();

  match app.data.radarr_data.movie_details_modal.as_ref() {
    Some(movie_details_modal) if !app.is_loading => {
      let is_monitored = app.data.radarr_data.movies.current_selection().monitored;
      let status = app
        .data
        .radarr_data
        .movies
        .current_selection()
        .status
        .clone();
      let movie_details = &movie_details_modal.movie_details;
      let download_status = movie_details
        .items
        .iter()
        .find(|&line| line.starts_with("Status: "))
        .unwrap_or(&unknown_download_status)
        .split(": ")
        .collect::<Vec<&str>>()[1];
      let text = Text::from(
        movie_details
          .items
          .iter()
          .map(|line| {
            let split = line.split(':').collect::<Vec<&str>>();
            let title = format!("{}:", split[0]);
            let style = style_from_download_status(download_status, is_monitored, status.clone());

            Line::from(vec![
              title.bold().style(style),
              Span::styled(split[1..].join(":"), style),
            ])
          })
          .collect::<Vec<Line<'_>>>(),
      );

      let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((movie_details.offset, 0));

      f.render_widget(paragraph, area);
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.radarr_data.movie_details_modal.is_none(),
        block,
      ),
      area,
    ),
  }
}

fn draw_movie_history(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Some(movie_details_modal) = app.data.radarr_data.movie_details_modal.as_mut() {
    let current_selection = if movie_details_modal.movie_history.items.is_empty() {
      MovieHistoryItem::default()
    } else {
      movie_details_modal
        .movie_history
        .current_selection()
        .clone()
    };
    let history_row_mapping = |movie_history_item: &MovieHistoryItem| {
      let MovieHistoryItem {
        source_title,
        quality,
        languages,
        date,
        event_type,
      } = movie_history_item;

      movie_history_item.source_title.scroll_left_or_reset(
        get_width_from_percentage(area, 34),
        current_selection == *movie_history_item,
        app.tick_count % app.ticks_until_scroll == 0,
      );

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
      .success()
    };
    let help_footer = app
      .data
      .radarr_data
      .movie_info_tabs
      .get_active_tab_contextual_help();
    let history_table = ManagarrTable::new(
      Some(&mut movie_details_modal.movie_history),
      history_row_mapping,
    )
    .block(layout_block_top_border())
    .loading(app.is_loading)
    .footer(help_footer)
    .headers(["Source Title", "Event Type", "Languages", "Quality", "Date"])
    .constraints([
      Constraint::Percentage(34),
      Constraint::Percentage(17),
      Constraint::Percentage(14),
      Constraint::Percentage(14),
      Constraint::Percentage(21),
    ]);

    f.render_widget(history_table, area);
  }
}

fn draw_movie_cast(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.radarr_data.movie_details_modal.as_mut() {
    Some(movie_details_modal) if !app.is_loading => {
      let cast_row_mapping = |cast_member: &Credit| {
        let Credit {
          person_name,
          character,
          ..
        } = cast_member;

        Row::new(vec![
          Cell::from(person_name.to_owned()),
          Cell::from(character.clone().unwrap_or_default()),
        ])
        .success()
      };
      let content = Some(&mut movie_details_modal.movie_cast);
      let help_footer = app
        .data
        .radarr_data
        .movie_info_tabs
        .get_active_tab_contextual_help();
      let cast_table = ManagarrTable::new(content, cast_row_mapping)
        .block(layout_block_top_border())
        .footer(help_footer)
        .loading(app.is_loading)
        .headers(["Cast Member", "Character"])
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]);

      f.render_widget(cast_table, area);
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.radarr_data.movie_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_movie_crew(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.radarr_data.movie_details_modal.as_mut() {
    Some(movie_details_modal) if !app.is_loading => {
      let crew_row_mapping = |crew_member: &Credit| {
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
        .success()
      };
      let content = Some(&mut movie_details_modal.movie_crew);
      let help_footer = app
        .data
        .radarr_data
        .movie_info_tabs
        .get_active_tab_contextual_help();
      let crew_table = ManagarrTable::new(content, crew_row_mapping)
        .block(layout_block_top_border())
        .loading(app.is_loading)
        .headers(["Crew Member", "Job", "Department"])
        .constraints(iter::repeat(Constraint::Ratio(1, 3)).take(3))
        .footer(help_footer);

      f.render_widget(crew_table, area);
    }

    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.radarr_data.movie_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_movie_releases(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
    let (current_selection, is_empty) = match app.data.radarr_data.movie_details_modal.as_ref() {
      Some(movie_details_modal) if !movie_details_modal.movie_releases.items.is_empty() => (
        movie_details_modal
          .movie_releases
          .current_selection()
          .clone(),
        movie_details_modal.movie_releases.items.is_empty(),
      ),
      _ => (RadarrRelease::default(), true),
    };
    let current_route = app.get_current_route();
    let mut default_movie_details_modal = MovieDetailsModal::default();
    let help_footer = app
      .data
      .radarr_data
      .movie_info_tabs
      .get_active_tab_contextual_help();
    let content = Some(
      &mut app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap_or(&mut default_movie_details_modal)
        .movie_releases,
    );
    let releases_row_mapping = |release: &RadarrRelease| {
      let RadarrRelease {
        protocol,
        age,
        title,
        indexer,
        size,
        rejected,
        seeders,
        leechers,
        languages,
        quality,
        ..
      } = release;
      let age = format!("{age} days");
      title.scroll_left_or_reset(
        get_width_from_percentage(area, 30),
        current_selection == *release
          && current_route != ActiveRadarrBlock::ManualSearchConfirmPrompt.into(),
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let size = convert_to_gb(*size);
      let rejected_str = if *rejected { "⛔" } else { "" };
      let peers = if seeders.is_none() || leechers.is_none() {
        Text::from("")
      } else {
        let seeders = seeders.clone().unwrap().as_u64().unwrap();
        let leechers = leechers.clone().unwrap().as_u64().unwrap();

        decorate_peer_style(
          seeders,
          leechers,
          Text::from(format!("{seeders} / {leechers}")),
        )
      };

      let language = if languages.is_some() {
        languages.clone().unwrap()[0].name.clone()
      } else {
        String::new()
      };
      let quality = quality.quality.name.clone();

      Row::new(vec![
        Cell::from(protocol.clone()),
        Cell::from(age),
        Cell::from(rejected_str),
        Cell::from(title.to_string()),
        Cell::from(indexer.clone()),
        Cell::from(format!("{size:.1} GB")),
        Cell::from(peers),
        Cell::from(language),
        Cell::from(quality),
      ])
      .primary()
    };
    let releases_table = ManagarrTable::new(content, releases_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading || is_empty)
      .footer(help_footer)
      .sorting(active_radarr_block == ActiveRadarrBlock::ManualSearchSortPrompt)
      .headers([
        "Source", "Age", "⛔", "Title", "Indexer", "Size", "Peers", "Language", "Quality",
      ])
      .constraints([
        Constraint::Length(9),
        Constraint::Length(10),
        Constraint::Length(5),
        Constraint::Percentage(30),
        Constraint::Percentage(18),
        Constraint::Length(12),
        Constraint::Length(12),
        Constraint::Percentage(7),
        Constraint::Percentage(10),
      ]);

    f.render_widget(releases_table, area);
  }
}

fn draw_manual_search_confirm_prompt(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = app
    .data
    .radarr_data
    .movie_details_modal
    .as_ref()
    .unwrap()
    .movie_releases
    .current_selection();
  let title = if current_selection.rejected {
    "Download Rejected Release"
  } else {
    "Download Release"
  };
  let prompt = if current_selection.rejected {
    format!(
      "Do you really want to download the rejected release: {}?",
      &current_selection.title.text
    )
  } else {
    format!(
      "Do you want to download the release: {}?",
      &current_selection.title.text
    )
  };

  if current_selection.rejected {
    let mut lines_vec = vec![Line::from("Rejection reasons: ".primary().bold())];
    let mut rejections_spans = current_selection
      .rejections
      .clone()
      .unwrap_or_default()
      .iter()
      .map(|item| Line::from(format!("• {item}").primary().bold()))
      .collect::<Vec<Line<'_>>>();
    lines_vec.append(&mut rejections_spans);

    let content_paragraph = Paragraph::new(lines_vec)
      .block(borderless_block())
      .wrap(Wrap { trim: false })
      .left_aligned();
    let confirmation_prompt = ConfirmationPrompt::new()
      .title(title)
      .prompt(&prompt)
      .content(content_paragraph)
      .yes_no_value(app.data.radarr_data.prompt_confirm);

    f.render_widget(Popup::new(confirmation_prompt).size(Size::Small), f.area());
  } else {
    let confirmation_prompt = ConfirmationPrompt::new()
      .title(title)
      .prompt(&prompt)
      .yes_no_value(app.data.radarr_data.prompt_confirm);

    f.render_widget(Popup::new(confirmation_prompt).size(Size::Prompt), f.area());
  }
}

fn style_from_download_status(download_status: &str, is_monitored: bool, status: String) -> Style {
  match download_status {
    "Downloaded" => Style::new().downloaded(),
    "Awaiting Import" => Style::new().awaiting_import(),
    "Downloading" => Style::new().downloading(),
    _ if !is_monitored && download_status == "Missing" => Style::new().unmonitored_missing(),
    _ if status != "released" && download_status == "Missing" => Style::new().unreleased(),
    "Missing" => Style::new().missing(),
    _ => Style::new().downloaded(),
  }
}

fn decorate_peer_style(seeders: u64, leechers: u64, text: Text<'_>) -> Text<'_> {
  if seeders == 0 {
    text.failure()
  } else if seeders < leechers {
    text.warning()
  } else {
    text.success()
  }
}
