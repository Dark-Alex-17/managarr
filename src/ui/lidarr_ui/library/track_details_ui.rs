use crate::app::App;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, TRACK_DETAILS_BLOCKS};
use crate::models::lidarr_models::{
  DownloadRecord, DownloadStatus, Track, LidarrHistoryEventType, LidarrHistoryItem, LidarrRelease,
};
use crate::models::Route;
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
  borderless_block, decorate_peer_style, get_width_from_percentage, layout_block_bottom_border,
  layout_block_top_border,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{draw_popup, draw_tabs, DrawUi};
use crate::utils::convert_to_gb;
use chrono::Utc;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use ratatui::Frame;
use serde_json::Number;

#[cfg(test)]
#[path = "track_details_ui_tests.rs"]
mod track_details_ui_tests;

pub(super) struct TrackDetailsUi;

impl DrawUi for TrackDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Lidarr(active_lidarr_block, _) = route {
      return TRACK_DETAILS_BLOCKS.contains(&active_lidarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref() {
      if album_details_modal.track_details_modal.is_some() {
        if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
          let draw_track_details_popup =
            |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
              let content_area = draw_tabs(
                f,
                popup_area,
                "Track Details",
                &app
                  .data
                  .lidarr_data
                  .album_details_modal
                  .as_ref()
                  .unwrap()
                  .track_details_modal
                  .as_ref()
                  .unwrap()
                  .track_details_tabs,
              );
              draw_track_details_tabs(f, app, content_area);

              match active_lidarr_block {
                ActiveLidarrBlock::AutomaticallySearchTrackPrompt => {
                  let prompt = format!(
                "Do you want to trigger an automatic search of your indexers for the track: {}",
                app.data.lidarr_data.album_details_modal.as_ref().unwrap().tracks.current_selection().title
              );
                  let confirmation_prompt = ConfirmationPrompt::new()
                    .title("Automatic Track Search")
                    .prompt(&prompt)
                    .yes_no_value(app.data.lidarr_data.prompt_confirm);

                  f.render_widget(
                    Popup::new(confirmation_prompt).size(Size::MediumPrompt),
                    f.area(),
                  );
                }
                ActiveLidarrBlock::ManualTrackSearchConfirmPrompt => {
                  draw_manual_track_search_confirm_prompt(f, app);
                }
                ActiveLidarrBlock::TrackHistoryDetails => {
                  draw_history_item_details_popup(f, app, popup_area);
                }
                _ => (),
              }
            };

          draw_popup(f, app, draw_track_details_popup, Size::Large);
        }
      }
    }
  }
}

pub fn draw_track_details_tabs(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref() {
    if let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref() {
      if let Route::Lidarr(active_lidarr_block, _) = track_details_modal
        .track_details_tabs
        .get_active_route()
      {
        match active_lidarr_block {
          ActiveLidarrBlock::TrackDetails => draw_track_details(f, app, area),
          ActiveLidarrBlock::TrackHistory => draw_track_history_table(f, app, area),
          ActiveLidarrBlock::TrackFile => draw_file_info(f, app, area),
          ActiveLidarrBlock::ManualTrackSearch => draw_track_releases(f, app, area),
          _ => (),
        }
      }
    }
  }
}

fn draw_track_details(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  let block = layout_block_top_border();

  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) if !app.is_loading => {
      if let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref() {
        let track = album_details_modal.tracks.current_selection().clone();
        let track_details = &track_details_modal.track_details;
        let default_album_id = Number::from(-1i64);
        let download = app
          .data
          .lidarr_data
          .downloads
          .items
          .iter()
          .find(|&download| {
            download
              .album_id
              .as_ref()
              .unwrap_or(&default_album_id)
              .as_i64()
              .unwrap()
              == track.id
          });
        let text = Text::from(
          track_details
            .items
            .iter()
            .map(|line| {
              let split = line.split(':').collect::<Vec<&str>>();
              let title = format!("{}:", split[0]);
              let style = style_from_status(download, &track);

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
          .scroll((track_details.offset, 0));

        f.render_widget(paragraph, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading
          || app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .unwrap()
            .track_details_modal
            .is_none(),
        block,
      ),
      area,
    ),
  }
}

fn draw_file_info(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) => match album_details_modal.track_details_modal.as_ref() {
      Some(track_details_modal)
        if !track_details_modal.file_details.is_empty() && !app.is_loading =>
      {
        let file_info = track_details_modal.file_details.to_owned();
        let audio_details = track_details_modal.audio_details.to_owned();
        let video_details = track_details_modal.video_details.to_owned();
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
      _ => f.render_widget(layout_block_top_border(), area),
    },
    _ => f.render_widget(
      LoadingBlock::new(app.is_loading, layout_block_top_border()),
      area,
    ),
  }
}

fn draw_track_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) if !app.is_loading => {
      if let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref() {
        let current_selection = if track_details_modal.track_history.is_empty() {
          LidarrHistoryItem::default()
        } else {
          track_details_modal
            .track_history
            .current_selection()
            .clone()
        };
        let track_history_table_footer = track_details_modal
          .track_details_tabs
          .get_active_tab_contextual_help();

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
        let mut track_history_table = &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .track_details_modal
          .as_mut()
          .unwrap()
          .track_history;
        let history_table =
          ManagarrTable::new(Some(&mut track_history_table), history_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading)
            .footer(track_history_table_footer)
            .headers(["Source Title", "Event Type", "Quality", "Date"])
            .constraints([
              Constraint::Percentage(40),
              Constraint::Percentage(15),
              Constraint::Percentage(13),
              Constraint::Percentage(20),
            ]);

        f.render_widget(history_table, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading
          || app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .unwrap()
            .track_details_modal
            .is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection =
    if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref() {
      if let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref() {
        if track_details_modal.track_history.is_empty() {
          LidarrHistoryItem::default()
        } else {
          track_details_modal
            .track_history
            .current_selection()
            .clone()
        }
      } else {
        LidarrHistoryItem::default()
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

fn draw_track_releases(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) if !app.is_loading => {
      if let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref() {
        let (current_selection, is_empty) = if track_details_modal.track_releases.is_empty() {
          (LidarrRelease::default(), true)
        } else {
          (
            track_details_modal
              .track_releases
              .current_selection()
              .clone(),
            track_details_modal.track_releases.is_empty(),
          )
        };
        let track_release_table_footer = track_details_modal
          .track_details_tabs
          .get_active_tab_contextual_help();

        if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
          let track_release_row_mapping = |release: &LidarrRelease| {
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
                && active_lidarr_block != ActiveLidarrBlock::ManualTrackSearchConfirmPrompt,
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
          let mut track_release_table = &mut app
            .data
            .lidarr_data
            .album_details_modal
            .as_mut()
            .unwrap()
            .track_details_modal
            .as_mut()
            .unwrap()
            .track_releases;
          let release_table = ManagarrTable::new(
            Some(&mut track_release_table),
            track_release_row_mapping,
          )
          .block(layout_block_top_border())
          .loading(app.is_loading || is_empty)
          .footer(track_release_table_footer)
          .sorting(active_lidarr_block == ActiveLidarrBlock::ManualTrackSearchSortPrompt)
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
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading
          || app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .unwrap()
            .track_details_modal
            .is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_manual_track_search_confirm_prompt(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = app
    .data
    .lidarr_data
    .album_details_modal
    .as_ref()
    .unwrap()
    .track_details_modal
    .as_ref()
    .unwrap()
    .track_releases
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

fn style_from_status(download: Option<&DownloadRecord>, track: &Track) -> Style {
  if !track.has_file {
    let default_track_id = Number::from(-1i64);
    if let Some(download) = download {
      if download.album_id.as_ref().unwrap_or(&default_track_id).as_i64().unwrap() == track.id {
        if download.status == DownloadStatus::Downloading {
          return Style::new().downloading();
        }

        if download.status == DownloadStatus::Completed {
          return Style::new().awaiting_import();
        }
      }
    }
    if !track.monitored {
      return Style::new().unmonitored_missing();
    }

    return Style::new().missing();
  }

  if !track.monitored {
    Style::new().unmonitored()
  } else {
    Style::new().downloaded()
  }
}
