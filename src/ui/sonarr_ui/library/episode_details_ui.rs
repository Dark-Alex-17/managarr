use crate::app::App;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS};
use crate::models::sonarr_models::{
  DownloadRecord, DownloadStatus, Episode, SonarrHistoryEventType, SonarrHistoryItem, SonarrRelease,
};
use crate::models::Route;
use crate::ui::sonarr_ui::sonarr_ui_utils::{
  create_download_failed_history_event_details,
  create_download_folder_imported_history_event_details,
  create_episode_file_deleted_history_event_details,
  create_episode_file_renamed_history_event_details, create_grabbed_history_event_details,
  create_no_data_history_event_details,
};
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
#[path = "episode_details_ui_tests.rs"]
mod episode_details_ui_tests;

pub(super) struct EpisodeDetailsUi;

impl DrawUi for EpisodeDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return EPISODE_DETAILS_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Some(season_details_modal) = app.data.sonarr_data.season_details_modal.as_ref() {
      if season_details_modal.episode_details_modal.is_some() {
        if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
          let draw_episode_details_popup =
            |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
              let content_area = draw_tabs(
                f,
                popup_area,
                "Episode Details",
                &app
                  .data
                  .sonarr_data
                  .season_details_modal
                  .as_ref()
                  .unwrap()
                  .episode_details_modal
                  .as_ref()
                  .unwrap()
                  .episode_details_tabs,
              );
              draw_episode_details_tabs(f, app, content_area);

              match active_sonarr_block {
                ActiveSonarrBlock::AutomaticallySearchEpisodePrompt => {
                  let prompt = format!(
                "Do you want to trigger an automatic search of your indexers for the episode: {}",
                app.data.sonarr_data.season_details_modal.as_ref().unwrap().episodes.current_selection().title
              );
                  let confirmation_prompt = ConfirmationPrompt::new()
                    .title("Automatic Episode Search")
                    .prompt(&prompt)
                    .yes_no_value(app.data.sonarr_data.prompt_confirm);

                  f.render_widget(
                    Popup::new(confirmation_prompt).size(Size::MediumPrompt),
                    f.area(),
                  );
                }
                ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt => {
                  draw_manual_episode_search_confirm_prompt(f, app);
                }
                ActiveSonarrBlock::EpisodeHistoryDetails => {
                  draw_history_item_details_popup(f, app, popup_area);
                }
                _ => (),
              }
            };

          draw_popup(f, app, draw_episode_details_popup, Size::Large);
        }
      }
    }
  }
}

pub fn draw_episode_details_tabs(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Some(season_details_modal) = app.data.sonarr_data.season_details_modal.as_ref() {
    if let Some(episode_details_modal) = season_details_modal.episode_details_modal.as_ref() {
      if let Route::Sonarr(active_sonarr_block, _) = episode_details_modal
        .episode_details_tabs
        .get_active_route()
      {
        match active_sonarr_block {
          ActiveSonarrBlock::EpisodeDetails => draw_episode_details(f, app, area),
          ActiveSonarrBlock::EpisodeHistory => draw_episode_history_table(f, app, area),
          ActiveSonarrBlock::EpisodeFile => draw_file_info(f, app, area),
          ActiveSonarrBlock::ManualEpisodeSearch => draw_episode_releases(f, app, area),
          _ => (),
        }
      }
    }
  }
}

fn draw_episode_details(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  let block = layout_block_top_border();

  match app.data.sonarr_data.season_details_modal.as_ref() {
    Some(season_details_modal) if !app.is_loading => {
      if let Some(episode_details_modal) = season_details_modal.episode_details_modal.as_ref() {
        let episode = season_details_modal.episodes.current_selection().clone();
        let episode_details = &episode_details_modal.episode_details;
        let default_episode_id = Number::from(-1i64);
        let download = app
          .data
          .sonarr_data
          .downloads
          .items
          .iter()
          .find(|&download| {
            download
              .episode_id
              .as_ref()
              .unwrap_or(&default_episode_id)
              .as_i64()
              .unwrap()
              == episode.id
          });
        let text = Text::from(
          episode_details
            .items
            .iter()
            .map(|line| {
              let split = line.split(':').collect::<Vec<&str>>();
              let title = format!("{}:", split[0]);
              let style = style_from_status(download, &episode);

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
          .scroll((episode_details.offset, 0));

        f.render_widget(paragraph, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading
          || app
            .data
            .sonarr_data
            .season_details_modal
            .as_ref()
            .unwrap()
            .episode_details_modal
            .is_none(),
        block,
      ),
      area,
    ),
  }
}

fn draw_file_info(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  match app.data.sonarr_data.season_details_modal.as_ref() {
    Some(season_details_modal) => match season_details_modal.episode_details_modal.as_ref() {
      Some(episode_details_modal)
        if !episode_details_modal.file_details.is_empty() && !app.is_loading =>
      {
        let file_info = episode_details_modal.file_details.to_owned();
        let audio_details = episode_details_modal.audio_details.to_owned();
        let video_details = episode_details_modal.video_details.to_owned();
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

fn draw_episode_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.sonarr_data.season_details_modal.as_ref() {
    Some(season_details_modal) if !app.is_loading => {
      if let Some(episode_details_modal) = season_details_modal.episode_details_modal.as_ref() {
        let current_selection = if episode_details_modal.episode_history.is_empty() {
          SonarrHistoryItem::default()
        } else {
          episode_details_modal
            .episode_history
            .current_selection()
            .clone()
        };

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
                .map(|language| language.name.to_owned())
                .collect::<Vec<String>>()
                .join(","),
            ),
            Cell::from(quality.quality.name.to_owned()),
            Cell::from(date.to_string()),
          ])
          .primary()
        };
        let mut episode_history_table = &mut app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap()
          .episode_history;
        let history_table =
          ManagarrTable::new(Some(&mut episode_history_table), history_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading)
            .headers(["Source Title", "Event Type", "Language", "Quality", "Date"])
            .constraints([
              Constraint::Percentage(40),
              Constraint::Percentage(15),
              Constraint::Percentage(12),
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
            .sonarr_data
            .season_details_modal
            .as_ref()
            .unwrap()
            .episode_details_modal
            .is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection =
    if let Some(season_details_modal) = app.data.sonarr_data.season_details_modal.as_ref() {
      if let Some(episode_details_modal) = season_details_modal.episode_details_modal.as_ref() {
        if episode_details_modal.episode_history.is_empty() {
          SonarrHistoryItem::default()
        } else {
          episode_details_modal
            .episode_history
            .current_selection()
            .clone()
        }
      } else {
        SonarrHistoryItem::default()
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

fn draw_episode_releases(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.sonarr_data.season_details_modal.as_ref() {
    Some(season_details_modal) if !app.is_loading => {
      if let Some(episode_details_modal) = season_details_modal.episode_details_modal.as_ref() {
        let (current_selection, is_empty) = if episode_details_modal.episode_releases.is_empty() {
          (SonarrRelease::default(), true)
        } else {
          (
            episode_details_modal
              .episode_releases
              .current_selection()
              .clone(),
            episode_details_modal.episode_releases.is_empty(),
          )
        };

        if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
          let episode_release_row_mapping = |release: &SonarrRelease| {
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
                && active_sonarr_block != ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt,
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
          let mut episode_release_table = &mut app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episode_details_modal
            .as_mut()
            .unwrap()
            .episode_releases;
          let release_table = ManagarrTable::new(
            Some(&mut episode_release_table),
            episode_release_row_mapping,
          )
          .block(layout_block_top_border())
          .loading(app.is_loading || is_empty)
          .sorting(active_sonarr_block == ActiveSonarrBlock::ManualEpisodeSearchSortPrompt)
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
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading
          || app
            .data
            .sonarr_data
            .season_details_modal
            .as_ref()
            .unwrap()
            .episode_details_modal
            .is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_manual_episode_search_confirm_prompt(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = app
    .data
    .sonarr_data
    .season_details_modal
    .as_ref()
    .unwrap()
    .episode_details_modal
    .as_ref()
    .unwrap()
    .episode_releases
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

fn style_from_status(download: Option<&DownloadRecord>, episode: &Episode) -> Style {
  if !episode.has_file {
    if let Some(download) = download {
      if download.status == DownloadStatus::Downloading {
        return Style::new().downloading();
      }

      if download.status == DownloadStatus::Completed {
        return Style::new().awaiting_import();
      }
    }
    if !episode.monitored {
      return Style::new().unmonitored_missing();
    }

    if let Some(air_date) = episode.air_date_utc.as_ref() {
      if air_date > &Utc::now() {
        return Style::new().unreleased();
      }
    }

    return Style::new().missing();
  }

  if !episode.monitored {
    Style::new().unmonitored()
  } else {
    Style::new().downloaded()
  }
}
