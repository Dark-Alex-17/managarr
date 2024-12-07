use deunicode::deunicode;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use ratatui::Frame;
use regex::Regex;

use crate::app::App;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SERIES_DETAILS_BLOCKS};
use crate::models::sonarr_models::{
  Season, SeasonStatistics, SonarrHistoryEventType, SonarrHistoryItem,
};
use crate::models::{EnumDisplayStyle, Route};
use crate::ui::sonarr_ui::sonarr_ui_utils::{
  create_download_failed_history_event_details,
  create_download_folder_imported_history_event_details,
  create_episode_file_deleted_history_event_details,
  create_episode_file_renamed_history_event_details, create_grabbed_history_event_details,
  create_no_data_history_event_details,
};
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, layout_block_top_border, title_block,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{draw_popup_over, draw_tabs, DrawUi};
use crate::utils::convert_to_gb;

use super::draw_library;

#[cfg(test)]
#[path = "series_details_ui_tests.rs"]
mod series_details_ui_tests;

pub(super) struct SeriesDetailsUi;

impl DrawUi for SeriesDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return SERIES_DETAILS_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
      let draw_series_details_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        f.render_widget(
          title_block(&app.data.sonarr_data.series.current_selection().title.text),
          popup_area,
        );
        let [description_area, detail_area] =
          Layout::vertical([Constraint::Percentage(37), Constraint::Fill(0)])
            .margin(1)
            .areas(popup_area);
        draw_series_description(f, app, description_area);
        let content_area = draw_tabs(
          f,
          detail_area,
          "Series Details",
          &app.data.sonarr_data.series_info_tabs,
        );
        draw_series_details(f, app, content_area);

        match active_sonarr_block {
          ActiveSonarrBlock::AutomaticallySearchSeriesPrompt => {
            let prompt = format!(
              "Do you want to trigger an automatic search of your indexers for all monitored episode(s) for the series: {}", app.data.sonarr_data.series.current_selection().title
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Automatic Series Search")
              .prompt(&prompt)
              .yes_no_value(app.data.sonarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          ActiveSonarrBlock::UpdateAndScanSeriesPrompt => {
            let prompt = format!(
              "Do you want to trigger an update and disk scan for the series: {}?",
              app.data.sonarr_data.series.current_selection().title
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Update and Scan")
              .prompt(&prompt)
              .yes_no_value(app.data.sonarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          ActiveSonarrBlock::SeriesHistoryDetails => {
            draw_history_item_details_popup(f, app, popup_area);
          }
          _ => (),
        };
      };

      draw_popup_over(
        f,
        app,
        area,
        draw_library,
        draw_series_details_popup,
        Size::XXLarge,
      );
    }
  }
}

pub fn draw_series_description(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection = app.data.sonarr_data.series.current_selection();
  let monitored = if current_selection.monitored {
    "Yes"
  } else {
    "No"
  };
  let quality_profile = app
    .data
    .sonarr_data
    .quality_profile_map
    .get_by_left(&current_selection.quality_profile_id)
    .unwrap()
    .to_owned();
  let language_profile = app
    .data
    .sonarr_data
    .language_profiles_map
    .get_by_left(&current_selection.language_profile_id)
    .unwrap()
    .to_owned();
  let overview = Regex::new(r"[\r\n\t]")
    .unwrap()
    .replace_all(
      &deunicode(
        current_selection
          .overview
          .as_ref()
          .unwrap_or(&String::new()),
      ),
      "",
    )
    .to_string();

  let mut series_description = vec![
    Line::from(vec![
      "Title: ".primary().bold(),
      current_selection.title.text.clone().primary().bold(),
    ]),
    Line::from(vec!["Overview: ".primary().bold(), overview.default()]),
    Line::from(vec![
      "Network: ".primary().bold(),
      current_selection
        .network
        .clone()
        .unwrap_or_default()
        .default(),
    ]),
    Line::from(vec![
      "Status: ".primary().bold(),
      current_selection.status.to_display_str().default(),
    ]),
    Line::from(vec![
      "Genres: ".primary().bold(),
      current_selection.genres.join(", ").default(),
    ]),
    Line::from(vec![
      "Rating: ".primary().bold(),
      format!("{}%", (current_selection.ratings.value * 10.0) as i32).default(),
    ]),
    Line::from(vec![
      "Year: ".primary().bold(),
      current_selection.year.to_string().default(),
    ]),
    Line::from(vec![
      "Runtime: ".primary().bold(),
      format!("{} minutes", current_selection.runtime).default(),
    ]),
    Line::from(vec![
      "Path: ".primary().bold(),
      current_selection.path.clone().default(),
    ]),
    Line::from(vec![
      "Quality Profile: ".primary().bold(),
      quality_profile.default(),
    ]),
    Line::from(vec![
      "Language Profile: ".primary().bold(),
      language_profile.default(),
    ]),
    Line::from(vec!["Monitored: ".primary().bold(), monitored.default()]),
  ];
  if let Some(stats) = current_selection.statistics.as_ref() {
    let size = convert_to_gb(stats.size_on_disk);
    series_description.extend(vec![Line::from(vec![
      "Size on Disk: ".primary().bold(),
      format!("{size:.2} GB").default(),
    ])]);
  }

  let description_paragraph = Paragraph::new(series_description)
    .block(borderless_block())
    .wrap(Wrap { trim: true });
  f.render_widget(description_paragraph, area);
}

pub fn draw_series_details(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Sonarr(active_sonarr_block, _) =
    app.data.sonarr_data.series_info_tabs.get_active_route()
  {
    match active_sonarr_block {
      ActiveSonarrBlock::SeriesDetails => draw_seasons_table(f, app, area),
      ActiveSonarrBlock::SeriesHistory => draw_series_history_table(f, app, area),
      _ => (),
    }
  }
}

fn draw_seasons_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
    let content = Some(&mut app.data.sonarr_data.seasons);
    let help_footer = app
      .data
      .sonarr_data
      .series_info_tabs
      .get_active_tab_contextual_help();
    let season_row_mapping = |season: &Season| {
      let Season {
        title,
        monitored,
        statistics,
        ..
      } = season;
      let SeasonStatistics {
        episode_count,
        total_episode_count,
        size_on_disk,
        ..
      } = statistics;
      let season_monitored = if season.monitored { "🏷" } else { "" };
      let size = convert_to_gb(*size_on_disk);

      let row = Row::new(vec![
        Cell::from(season_monitored.to_owned()),
        Cell::from(title.clone().unwrap()),
        Cell::from(format!("{}/{}", episode_count, total_episode_count)),
        Cell::from(format!("{size:.2} GB")),
      ]);
      if episode_count == total_episode_count {
        row.downloaded()
      } else if !monitored {
        row.unmonitored()
      } else {
        row.missing()
      }
    };
    let is_searching = active_sonarr_block == ActiveSonarrBlock::SearchSeason;
    let season_table = ManagarrTable::new(content, season_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .footer(help_footer)
      .searching(is_searching)
      .search_produced_empty_results(active_sonarr_block == ActiveSonarrBlock::SearchSeasonError)
      .headers(["Monitored", "Season", "Episode Count", "Size on Disk"])
      .constraints([
        Constraint::Percentage(6),
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
      ]);

    if is_searching {
      season_table.show_cursor(f, area);
    }

    f.render_widget(season_table, area);
  }
}

fn draw_series_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.sonarr_data.series_history.as_ref() {
    Some(series_history) if !app.is_loading => {
      let current_selection = if series_history.is_empty() {
        SonarrHistoryItem::default()
      } else {
        series_history.current_selection().clone()
      };
      let series_history_table_footer = app
        .data
        .sonarr_data
        .series_info_tabs
        .get_active_tab_contextual_help();

      if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
        let history_row_mapping = |history_item: &SonarrHistoryItem| {
          let SonarrHistoryItem {
            source_title,
            language,
            quality,
            event_type,
            date,
            ..
          } = history_item;

          source_title.scroll_left_or_reset(
            get_width_from_percentage(area, 40),
            current_selection == *history_item,
            app.tick_count % app.ticks_until_scroll == 0,
          );

          Row::new(vec![
            Cell::from(source_title.to_string()),
            Cell::from(event_type.to_string()),
            Cell::from(language.name.to_owned()),
            Cell::from(quality.quality.name.to_owned()),
            Cell::from(date.to_string()),
          ])
          .primary()
        };
        let mut series_history_table = app.data.sonarr_data.series_history.as_mut().unwrap();
        let history_table =
          ManagarrTable::new(Some(&mut series_history_table), history_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading)
            .footer(series_history_table_footer)
            .sorting(active_sonarr_block == ActiveSonarrBlock::SeriesHistorySortPrompt)
            .searching(active_sonarr_block == ActiveSonarrBlock::SearchSeriesHistory)
            .search_produced_empty_results(
              active_sonarr_block == ActiveSonarrBlock::SearchSeriesHistoryError,
            )
            .filtering(active_sonarr_block == ActiveSonarrBlock::FilterSeriesHistory)
            .filter_produced_empty_results(
              active_sonarr_block == ActiveSonarrBlock::FilterSeriesHistoryError,
            )
            .headers(["Source Title", "Event Type", "Language", "Quality", "Date"])
            .constraints([
              Constraint::Percentage(40),
              Constraint::Percentage(15),
              Constraint::Percentage(12),
              Constraint::Percentage(13),
              Constraint::Percentage(20),
            ]);

        if [
          ActiveSonarrBlock::SearchSeriesHistory,
          ActiveSonarrBlock::FilterSeriesHistory,
        ]
        .contains(&active_sonarr_block)
        {
          history_table.show_cursor(f, area);
        }

        f.render_widget(history_table, area);
      }
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

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection =
    if let Some(series_history_items) = app.data.sonarr_data.series_history.as_ref() {
      if series_history_items.is_empty() {
        SonarrHistoryItem::default()
      } else {
        series_history_items.current_selection().clone()
      }
    } else {
      SonarrHistoryItem::default()
    };

  let line_vec = match current_selection.event_type {
    SonarrHistoryEventType::Grabbed => create_grabbed_history_event_details(current_selection),
    SonarrHistoryEventType::DownloadFolderImported => {
      create_download_folder_imported_history_event_details(current_selection)
    }
    SonarrHistoryEventType::DownloadFailed => {
      create_download_failed_history_event_details(current_selection)
    }
    SonarrHistoryEventType::EpisodeFileDeleted => {
      create_episode_file_deleted_history_event_details(current_selection)
    }
    SonarrHistoryEventType::EpisodeFileRenamed => {
      create_episode_file_renamed_history_event_details(current_selection)
    }
    _ => create_no_data_history_event_details(current_selection),
  };
  let text = Text::from(line_vec);

  let message = Message::new(text)
    .title("Details")
    .style(Style::new().secondary())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowMessage), area);
}
