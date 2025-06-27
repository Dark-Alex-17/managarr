use crate::app::App;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, ALBUM_DETAILS_BLOCKS};
use crate::models::lidarr_models::{
  DownloadRecord, DownloadStatus, Track, LidarrHistoryEventType, LidarrHistoryItem, LidarrRelease,
};
use crate::models::Route;
use crate::ui::lidarr_ui::library::track_details_ui::TrackDetailsUi;
// TODO
// use crate::ui::lidarr_ui::lidarr_ui_utils::{
//   create_download_failed_history_event_details,
//   create_download_folder_imported_history_event_details,
//   create_track_file_deleted_history_event_details,
//   create_track_file_renamed_history_event_details, create_grabbed_history_event_details,
//   create_no_data_history_event_details,
// };
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
#[path = "album_details_ui_tests.rs"]
mod album_details_ui_tests;

pub(super) struct AlbumDetailsUi;

impl DrawUi for AlbumDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Lidarr(active_lidarr_block, _) = route {
      return TrackDetailsUi::accepts(route)
        || ALBUM_DETAILS_BLOCKS.contains(&active_lidarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    let route = app.get_current_route();
    if app.data.lidarr_data.album_details_modal.is_some() {
      if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
        let draw_album_details_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
          let content_area = draw_tabs(
            f,
            popup_area,
            &format!(
              "Album {} Details",
              app
                .data
                .lidarr_data
                .albums
                .current_selection()
                .title
                .text
            ),
            &app
              .data
              .lidarr_data
              .album_details_modal
              .as_ref()
              .unwrap()
              .album_details_tabs,
          );
          draw_album_details(f, app, content_area);

          match active_lidarr_block {
            ActiveLidarrBlock::AutomaticallySearchAlbumPrompt => {
              let prompt = format!(
                "Do you want to trigger an automatic search of your indexers for all monitored track(s) for the album: {}", app.data.lidarr_data.albums.current_selection().title
              );
              let confirmation_prompt = ConfirmationPrompt::new()
                .title("Automatic Album Search")
                .prompt(&prompt)
                .yes_no_value(app.data.lidarr_data.prompt_confirm);

              f.render_widget(
                Popup::new(confirmation_prompt).size(Size::MediumPrompt),
                f.area(),
              );
            }
            ActiveLidarrBlock::DeleteTrackFilePrompt => {
              let prompt = format!(
                "Do you really want to delete this track: \n{}?",
                app
                  .data
                  .lidarr_data
                  .album_details_modal
                  .as_ref()
                  .unwrap()
                  .tracks
                  .current_selection()
                  .title
              );
              let confirmation_prompt = ConfirmationPrompt::new()
                .title("Delete Track")
                .prompt(&prompt)
                .yes_no_value(app.data.lidarr_data.prompt_confirm);

              f.render_widget(
                Popup::new(confirmation_prompt).size(Size::MediumPrompt),
                f.area(),
              );
            }
            ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt => {
              draw_manual_album_search_confirm_prompt(f, app);
            }
            ActiveLidarrBlock::AlbumHistoryDetails => {
              draw_history_item_details_popup(f, app, popup_area);
            }
            _ => (),
          }
        };

        draw_popup(f, app, draw_album_details_popup, Size::XLarge);

        if TrackDetailsUi::accepts(route) {
          TrackDetailsUi::draw(f, app, _area);
        }
      }
    }
  }
}

pub fn draw_album_details(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref() {
    if let Route::Lidarr(active_lidarr_block, _) =
      album_details_modal.album_details_tabs.get_active_route()
    {
      match active_lidarr_block {
        ActiveLidarrBlock::AlbumDetails => draw_tracks_table(f, app, area),
        ActiveLidarrBlock::AlbumHistory => draw_album_history_table(f, app, area),
        ActiveLidarrBlock::ManualAlbumSearch => draw_album_releases(f, app, area),
        _ => (),
      }
    }
  }
}

fn draw_tracks_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let help_footer = app
      .data
      .lidarr_data
      .album_details_modal
      .as_ref()
      .expect("Album details modal is unpopulated")
      .album_details_tabs
      .get_active_tab_contextual_help();
    let track_files = app
      .data
      .lidarr_data
      .album_details_modal
      .as_ref()
      .expect("Album details modal is unpopulated")
      .track_files
      .items
      .clone();
    let content = Some(
      &mut app
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .expect("Album details modal is unpopulated")
        .tracks,
    );
    let downloads_vec = &app.data.lidarr_data.downloads.items;

    let track_row_mapping = |track: &Track| {
      let Track {
        track_number,
        title,
        track_file_id,
        ..
      } = track;
      let track_file = track_files
        .iter()
        .find(|track_file| track_file.id == *track_file_id);
      let (quality_profile, size_on_disk) = if let Some(track_file) = track_file {
        (
          track_file.quality.quality.name.to_owned(),
          track_file.size,
        )
      } else {
        (String::new(), 0)
      };

      let track_monitored = if track.monitored { "🏷" } else { "" };
      let size = convert_to_gb(size_on_disk);

      decorate_with_row_style(
        downloads_vec,
        track,
        Row::new(vec![
          Cell::from(track_monitored.to_owned()),
          Cell::from(track_number.to_string()),
          Cell::from(title.clone()),
          Cell::from(format!("{size:.2} GB")),
          Cell::from(quality_profile),
        ]),
      )
    };
    let is_searching = active_lidarr_block == ActiveLidarrBlock::SearchTracks;
    let album_table = ManagarrTable::new(content, track_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .footer(help_footer)
      .searching(is_searching)
      .search_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::SearchTracksError)
      .headers([
        "🏷",
        "#",
        "Title",
        "Size on Disk",
        "Quality Profile",
      ])
      .constraints([
        Constraint::Percentage(4),
        Constraint::Percentage(4),
        Constraint::Percentage(50),
        Constraint::Percentage(10),
        Constraint::Percentage(12),
      ]);

    if is_searching {
      album_table.show_cursor(f, area);
    }

    f.render_widget(album_table, area);
  }
}

fn draw_album_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) if !app.is_loading => {
      let current_selection = if album_details_modal.album_history.is_empty() {
        LidarrHistoryItem::default()
      } else {
        album_details_modal
          .album_history
          .current_selection()
          .clone()
      };
      let album_history_table_footer = album_details_modal
        .album_details_tabs
        .get_active_tab_contextual_help();

      if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
        let history_row_mapping = |history_item: &LidarrHistoryItem| {
          let LidarrHistoryItem {
            source_title,
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
            Cell::from(quality.quality.name.to_owned()),
            Cell::from(date.to_string()),
          ])
          .primary()
        };
        let mut album_history_table = &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .album_history;
        let history_table =
          ManagarrTable::new(Some(&mut album_history_table), history_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading)
            .footer(album_history_table_footer)
            .sorting(active_lidarr_block == ActiveLidarrBlock::AlbumHistorySortPrompt)
            .searching(active_lidarr_block == ActiveLidarrBlock::SearchAlbumHistory)
            .search_produced_empty_results(
              active_lidarr_block == ActiveLidarrBlock::SearchAlbumHistoryError,
            )
            .filtering(active_lidarr_block == ActiveLidarrBlock::FilterAlbumHistory)
            .filter_produced_empty_results(
              active_lidarr_block == ActiveLidarrBlock::FilterAlbumHistoryError,
            )
            .headers(["Source Title", "Event Type", "Quality", "Date"])
            .constraints([
              Constraint::Percentage(40),
              Constraint::Percentage(15),
              Constraint::Percentage(13),
              Constraint::Percentage(20),
            ]);

        if [
          ActiveLidarrBlock::SearchAlbumHistory,
          ActiveLidarrBlock::FilterAlbumHistory,
        ]
        .contains(&active_lidarr_block)
        {
          history_table.show_cursor(f, area);
        }

        f.render_widget(history_table, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.lidarr_data.album_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_album_releases(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) if !app.is_loading => {
      let (current_selection, is_empty) = if album_details_modal.album_releases.is_empty() {
        (LidarrRelease::default(), true)
      } else {
        (
          album_details_modal
            .album_releases
            .current_selection()
            .clone(),
          album_details_modal.album_releases.is_empty(),
        )
      };
      let album_release_table_footer = album_details_modal
        .album_details_tabs
        .get_active_tab_contextual_help();

      if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
        let album_release_row_mapping = |release: &LidarrRelease| {
          let LidarrRelease {
            protocol,
            age,
            title,
            indexer,
            size,
            rejected,
            seeders,
            leechers,
            quality,
            ..
          } = release;

          let age = format!("{age} days");
          title.scroll_left_or_reset(
            get_width_from_percentage(area, 30),
            current_selection == *release
              && active_lidarr_block != ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt,
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

          let quality = quality.quality.name.clone();

          Row::new(vec![
            Cell::from(protocol.clone()),
            Cell::from(age),
            Cell::from(rejected_str),
            Cell::from(title.to_string()),
            Cell::from(indexer.clone()),
            Cell::from(format!("{size:.1} GB")),
            Cell::from(peers),
            Cell::from(quality),
          ])
          .primary()
        };
        let mut album_release_table = &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .album_releases;
        let release_table =
          ManagarrTable::new(Some(&mut album_release_table), album_release_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading || is_empty)
            .footer(album_release_table_footer)
            .sorting(active_lidarr_block == ActiveLidarrBlock::ManualAlbumSearchSortPrompt)
            .headers([
              "Source", "Age", "⛔", "Title", "Indexer", "Size", "Peers", "Quality",
            ])
            .constraints([
              Constraint::Length(9),
              Constraint::Length(10),
              Constraint::Length(5),
              Constraint::Percentage(30),
              Constraint::Percentage(18),
              Constraint::Length(12),
              Constraint::Length(12),
              Constraint::Percentage(10),
            ]);

        f.render_widget(release_table, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.lidarr_data.album_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_manual_album_search_confirm_prompt(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = app
    .data
    .lidarr_data
    .album_details_modal
    .as_ref()
    .unwrap()
    .album_releases
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
      .yes_no_value(app.data.lidarr_data.prompt_confirm);

    f.render_widget(Popup::new(confirmation_prompt).size(Size::Small), f.area());
  } else {
    let confirmation_prompt = ConfirmationPrompt::new()
      .title(title)
      .prompt(&prompt)
      .yes_no_value(app.data.lidarr_data.prompt_confirm);

    f.render_widget(
      Popup::new(confirmation_prompt).size(Size::MediumPrompt),
      f.area(),
    );
  }
}

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection =
    if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref() {
      if album_details_modal.album_history.is_empty() {
        LidarrHistoryItem::default()
      } else {
        album_details_modal
          .album_history
          .current_selection()
          .clone()
      }
    } else {
      LidarrHistoryItem::default()
    };

  // TODO
  // let line_vec = match current_selection.event_type {
  //   LidarrHistoryEventType::Grabbed => create_grabbed_history_event_details(current_selection),
  //   LidarrHistoryEventType::DownloadFolderImported => {
  //     create_download_folder_imported_history_event_details(current_selection)
  //   }
  //   LidarrHistoryEventType::DownloadFailed => {
  //     create_download_failed_history_event_details(current_selection)
  //   }
  //   LidarrHistoryEventType::TrackFileDeleted => {
  //     create_track_file_deleted_history_event_details(current_selection)
  //   }
  //   LidarrHistoryEventType::TrackFileRenamed => {
  //     create_track_file_renamed_history_event_details(current_selection)
  //   }
  //   _ => create_no_data_history_event_details(current_selection),
  // };
  // let text = Text::from(line_vec);
  let text = Text::from(String::new());
  
  let message = Message::new(text)
    .title("Details")
    .style(Style::new().secondary())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowMessage), area);
}

fn decorate_with_row_style<'a>(
  downloads_vec: &[DownloadRecord],
  track: &Track,
  row: Row<'a>,
) -> Row<'a> {
  if !track.has_file {
    let default_track_id = Number::from(-1i64);
    if let Some(download) = downloads_vec.iter().find(|&download| {
      download
        .album_id
        .as_ref()
        .unwrap_or(&default_track_id)
        .as_i64()
        .unwrap()
        == track.id
    }) {
      if download.status == DownloadStatus::Downloading {
        return row.downloading();
      }

      if download.status == DownloadStatus::Completed {
        return row.awaiting_import();
      }
    }

    if !track.monitored {
      return row.unmonitored_missing();
    }

    return row.missing();
  }

  if !track.monitored {
    row.unmonitored()
  } else {
    row.downloaded()
  }
}
