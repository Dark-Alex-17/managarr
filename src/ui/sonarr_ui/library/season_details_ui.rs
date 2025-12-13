use crate::app::App;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SEASON_DETAILS_BLOCKS};
use crate::models::sonarr_models::{
  DownloadRecord, DownloadStatus, Episode, SonarrHistoryEventType, SonarrHistoryItem, SonarrRelease,
};
use crate::models::Route;
use crate::ui::sonarr_ui::library::episode_details_ui::EpisodeDetailsUi;
use crate::ui::sonarr_ui::sonarr_ui_utils::{
  create_download_failed_history_event_details,
  create_download_folder_imported_history_event_details,
  create_episode_file_deleted_history_event_details,
  create_episode_file_renamed_history_event_details, create_grabbed_history_event_details,
  create_no_data_history_event_details,
};
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  borderless_block, decorate_peer_style, get_width_from_percentage, layout_block_top_border,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{draw_popup, draw_tabs, DrawUi};
use crate::utils::convert_to_gb;
use chrono::Utc;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::prelude::{Line, Style, Stylize, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use ratatui::Frame;
use serde_json::Number;

#[cfg(test)]
#[path = "season_details_ui_tests.rs"]
mod season_details_ui_tests;

pub(super) struct SeasonDetailsUi;

impl DrawUi for SeasonDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return EpisodeDetailsUi::accepts(route)
        || SEASON_DETAILS_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    let route = app.get_current_route();
    if app.data.sonarr_data.season_details_modal.is_some() {
      if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
        let draw_season_details_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
          let content_area = draw_tabs(
            f,
            popup_area,
            &format!(
              "Season {} Details",
              app
                .data
                .sonarr_data
                .seasons
                .current_selection()
                .season_number
            ),
            &app
              .data
              .sonarr_data
              .season_details_modal
              .as_ref()
              .unwrap()
              .season_details_tabs,
          );
          draw_season_details(f, app, content_area);

          match active_sonarr_block {
            ActiveSonarrBlock::AutomaticallySearchSeasonPrompt => {
              let prompt = format!(
                "Do you want to trigger an automatic search of your indexers for season packs for: {}",
                app.data.sonarr_data.seasons.current_selection().title.as_ref().unwrap()
              );
              let confirmation_prompt = ConfirmationPrompt::new()
                .title("Automatic Season Search")
                .prompt(&prompt)
                .yes_no_value(app.data.sonarr_data.prompt_confirm);

              f.render_widget(
                Popup::new(confirmation_prompt).size(Size::MediumPrompt),
                f.area(),
              );
            }
            ActiveSonarrBlock::DeleteEpisodeFilePrompt => {
              let prompt = format!(
                "Do you really want to delete this episode: \n{}?",
                app
                  .data
                  .sonarr_data
                  .season_details_modal
                  .as_ref()
                  .unwrap()
                  .episodes
                  .current_selection()
                  .title
              );
              let confirmation_prompt = ConfirmationPrompt::new()
                .title("Delete Episode")
                .prompt(&prompt)
                .yes_no_value(app.data.sonarr_data.prompt_confirm);

              f.render_widget(
                Popup::new(confirmation_prompt).size(Size::MediumPrompt),
                f.area(),
              );
            }
            ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt => {
              draw_manual_season_search_confirm_prompt(f, app);
            }
            ActiveSonarrBlock::SeasonHistoryDetails => {
              draw_history_item_details_popup(f, app, popup_area);
            }
            _ => (),
          }
        };

        draw_popup(f, app, draw_season_details_popup, Size::XLarge);

        if EpisodeDetailsUi::accepts(route) {
          EpisodeDetailsUi::draw(f, app, _area);
        }
      }
    }
  }
}

pub fn draw_season_details(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Some(season_details_modal) = app.data.sonarr_data.season_details_modal.as_ref() {
    if let Route::Sonarr(active_sonarr_block, _) =
      season_details_modal.season_details_tabs.get_active_route()
    {
      match active_sonarr_block {
        ActiveSonarrBlock::SeasonDetails => draw_episodes_table(f, app, area),
        ActiveSonarrBlock::SeasonHistory => draw_season_history_table(f, app, area),
        ActiveSonarrBlock::ManualSeasonSearch => draw_season_releases(f, app, area),
        _ => (),
      }
    }
  }
}

fn draw_episodes_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
    let episode_files = app
      .data
      .sonarr_data
      .season_details_modal
      .as_ref()
      .expect("Season details modal is unpopulated")
      .episode_files
      .items
      .clone();
    let content = Some(
      &mut app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .expect("Season details modal is unpopulated")
        .episodes,
    );
    let downloads_vec = &app.data.sonarr_data.downloads.items;

    let episode_row_mapping = |episode: &Episode| {
      let Episode {
        episode_number,
        title,
        air_date_utc,
        episode_file_id,
        ..
      } = episode;
      let episode_file = episode_files
        .iter()
        .find(|episode_file| episode_file.id == *episode_file_id);
      let (quality_profile, size_on_disk) = if let Some(episode_file) = episode_file {
        (
          episode_file.quality.quality.name.to_owned(),
          episode_file.size,
        )
      } else {
        (String::new(), 0)
      };

      let episode_monitored = if episode.monitored { "🏷" } else { "" };
      let size = convert_to_gb(size_on_disk);
      let air_date = if let Some(air_date) = air_date_utc.as_ref() {
        air_date.to_string()
      } else {
        String::new()
      };

      decorate_with_row_style(
        downloads_vec,
        episode,
        Row::new(vec![
          Cell::from(episode_monitored.to_owned()),
          Cell::from(episode_number.to_string()),
          Cell::from(title.clone()),
          Cell::from(air_date),
          Cell::from(format!("{size:.2} GB")),
          Cell::from(quality_profile),
        ]),
      )
    };
    let is_searching = active_sonarr_block == ActiveSonarrBlock::SearchEpisodes;
    let season_table = ManagarrTable::new(content, episode_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .searching(is_searching)
      .search_produced_empty_results(active_sonarr_block == ActiveSonarrBlock::SearchEpisodesError)
      .headers([
        "🏷",
        "#",
        "Title",
        "Air Date",
        "Size on Disk",
        "Quality Profile",
      ])
      .constraints([
        Constraint::Percentage(4),
        Constraint::Percentage(4),
        Constraint::Percentage(50),
        Constraint::Percentage(19),
        Constraint::Percentage(10),
        Constraint::Percentage(12),
      ]);

    if is_searching {
      season_table.show_cursor(f, area);
    }

    f.render_widget(season_table, area);
  }
}

fn draw_season_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.sonarr_data.season_details_modal.as_ref() {
    Some(season_details_modal) if !app.is_loading => {
      let current_selection = if season_details_modal.season_history.is_empty() {
        SonarrHistoryItem::default()
      } else {
        season_details_modal
          .season_history
          .current_selection()
          .clone()
      };

      if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
        let history_row_mapping = |history_item: &SonarrHistoryItem| {
          let SonarrHistoryItem {
            source_title,
            languages,
            quality,
            event_type,
            date,
            ..
          } = history_item;

          source_title.scroll_left_or_reset(
            get_width_from_percentage(area, 40),
            current_selection == *history_item,
            app.tick_count.is_multiple_of(app.ticks_until_scroll),
          );

          Row::new(vec![
            Cell::from(source_title.to_string()),
            Cell::from(event_type.to_string()),
            Cell::from(
              languages
                .iter()
                .map(|language| {
                  language
                    .as_ref()
                    .unwrap_or(&Default::default())
                    .name
                    .to_owned()
                })
                .collect::<Vec<String>>()
                .join(","),
            ),
            Cell::from(quality.quality.name.to_owned()),
            Cell::from(date.to_string()),
          ])
          .primary()
        };
        let mut season_history_table = &mut app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .season_history;
        let history_table =
          ManagarrTable::new(Some(&mut season_history_table), history_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading)
            .sorting(active_sonarr_block == ActiveSonarrBlock::SeasonHistorySortPrompt)
            .searching(active_sonarr_block == ActiveSonarrBlock::SearchSeasonHistory)
            .search_produced_empty_results(
              active_sonarr_block == ActiveSonarrBlock::SearchSeasonHistoryError,
            )
            .filtering(active_sonarr_block == ActiveSonarrBlock::FilterSeasonHistory)
            .filter_produced_empty_results(
              active_sonarr_block == ActiveSonarrBlock::FilterSeasonHistoryError,
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
        app.is_loading || app.data.sonarr_data.season_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_season_releases(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.sonarr_data.season_details_modal.as_ref() {
    Some(season_details_modal) if !app.is_loading => {
      let (current_selection, is_empty) = if season_details_modal.season_releases.is_empty() {
        (SonarrRelease::default(), true)
      } else {
        (
          season_details_modal
            .season_releases
            .current_selection()
            .clone(),
          season_details_modal.season_releases.is_empty(),
        )
      };

      if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
        let season_release_row_mapping = |release: &SonarrRelease| {
          let SonarrRelease {
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
              && active_sonarr_block != ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt,
            app.tick_count.is_multiple_of(app.ticks_until_scroll),
          );
          let size = convert_to_gb(*size);
          let rejected_str = if *rejected { "⛔" } else { "" };
          let peers = if seeders.is_none() || leechers.is_none() {
            Text::from("")
          } else {
            let seeders = seeders
              .clone()
              .unwrap_or(Number::from(0u64))
              .as_u64()
              .unwrap();
            let leechers = leechers
              .clone()
              .unwrap_or(Number::from(0u64))
              .as_u64()
              .unwrap();

            decorate_peer_style(
              seeders,
              leechers,
              Text::from(format!("{seeders} / {leechers}")),
            )
          };

          let language = if languages.is_some() {
            languages.clone().unwrap()[0]
              .as_ref()
              .unwrap_or(&Default::default())
              .name
              .clone()
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
        let mut season_release_table = &mut app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .season_releases;
        let release_table =
          ManagarrTable::new(Some(&mut season_release_table), season_release_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading || is_empty)
            .sorting(active_sonarr_block == ActiveSonarrBlock::ManualSeasonSearchSortPrompt)
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

        f.render_widget(release_table, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.sonarr_data.season_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_manual_season_search_confirm_prompt(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = app
    .data
    .sonarr_data
    .season_details_modal
    .as_ref()
    .unwrap()
    .season_releases
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
      .yes_no_value(app.data.sonarr_data.prompt_confirm);

    f.render_widget(Popup::new(confirmation_prompt).size(Size::Small), f.area());
  } else {
    let confirmation_prompt = ConfirmationPrompt::new()
      .title(title)
      .prompt(&prompt)
      .yes_no_value(app.data.sonarr_data.prompt_confirm);

    f.render_widget(
      Popup::new(confirmation_prompt).size(Size::MediumPrompt),
      f.area(),
    );
  }
}

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection =
    if let Some(season_details_modal) = app.data.sonarr_data.season_details_modal.as_ref() {
      if season_details_modal.season_history.is_empty() {
        SonarrHistoryItem::default()
      } else {
        season_details_modal
          .season_history
          .current_selection()
          .clone()
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

fn decorate_with_row_style<'a>(
  downloads_vec: &[DownloadRecord],
  episode: &Episode,
  row: Row<'a>,
) -> Row<'a> {
  if !episode.has_file {
    let default_episode_id = Number::from(-1i64);
    if let Some(download) = downloads_vec.iter().find(|&download| {
      download
        .episode_id
        .as_ref()
        .unwrap_or(&default_episode_id)
        .as_i64()
        .unwrap()
        == episode.id
    }) {
      if download.status == DownloadStatus::Downloading {
        return row.downloading();
      }

      if download.status == DownloadStatus::Completed {
        return row.awaiting_import();
      }
    }

    if !episode.monitored {
      return row.unmonitored_missing();
    }

    if let Some(air_date) = episode.air_date_utc.as_ref() {
      if air_date > &Utc::now() {
        return row.unreleased();
      }
    }

    return row.missing();
  }

  if !episode.monitored {
    row.unmonitored()
  } else {
    row.downloaded()
  }
}
