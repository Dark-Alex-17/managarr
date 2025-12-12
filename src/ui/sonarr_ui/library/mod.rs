use add_series_ui::AddSeriesUi;
use delete_series_ui::DeleteSeriesUi;
use edit_series_ui::EditSeriesUi;
use ratatui::{
  layout::{Constraint, Rect},
  widgets::{Cell, Row},
  Frame,
};
use series_details_ui::SeriesDetailsUi;

use crate::ui::widgets::{
  confirmation_prompt::ConfirmationPrompt,
  popup::{Popup, Size},
};
use crate::utils::convert_to_gb;
use crate::{
  app::App,
  models::{
    servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, LIBRARY_BLOCKS},
    sonarr_models::{Series, SeriesStatus},
    Route,
  },
  ui::{
    styles::ManagarrStyle,
    utils::{get_width_from_percentage, layout_block_top_border},
    widgets::managarr_table::ManagarrTable,
    DrawUi,
  },
};

mod add_series_ui;
mod delete_series_ui;
mod edit_series_ui;
mod series_details_ui;

mod episode_details_ui;
#[cfg(test)]
#[path = "library_ui_tests.rs"]
mod library_ui_tests;
mod season_details_ui;

pub(super) struct LibraryUi;

impl DrawUi for LibraryUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return AddSeriesUi::accepts(route)
        || DeleteSeriesUi::accepts(route)
        || EditSeriesUi::accepts(route)
        || SeriesDetailsUi::accepts(route)
        || LIBRARY_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    let route = app.get_current_route();
    draw_library(f, app, area);

    match route {
      _ if AddSeriesUi::accepts(route) => AddSeriesUi::draw(f, app, area),
      _ if DeleteSeriesUi::accepts(route) => DeleteSeriesUi::draw(f, app, area),
      _ if EditSeriesUi::accepts(route) => EditSeriesUi::draw(f, app, area),
      _ if SeriesDetailsUi::accepts(route) => SeriesDetailsUi::draw(f, app, area),
      Route::Sonarr(ActiveSonarrBlock::UpdateAllSeriesPrompt, _) => {
        let confirmation_prompt = ConfirmationPrompt::new()
          .title("Update All Series")
          .prompt("Do you want to update info and scan your disks for all of your series?")
          .yes_no_value(app.data.sonarr_data.prompt_confirm);

        f.render_widget(
          Popup::new(confirmation_prompt).size(Size::MediumPrompt),
          f.area(),
        );
      }
      _ => (),
    }
  }
}

fn draw_library(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
    let current_selection = if !app.data.sonarr_data.series.items.is_empty() {
      app.data.sonarr_data.series.current_selection().clone()
    } else {
      Series::default()
    };
    let quality_profile_map = &app.data.sonarr_data.quality_profile_map;
    let language_profile_map = &app.data.sonarr_data.language_profiles_map;
    let tags_map = &app.data.sonarr_data.tags_map;
    let content = Some(&mut app.data.sonarr_data.series);

    let series_table_row_mapping = |series: &Series| {
      series.title.scroll_left_or_reset(
        get_width_from_percentage(area, 23),
        *series == current_selection,
        app.tick_count.is_multiple_of(app.ticks_until_scroll),
      );
      let monitored = if series.monitored { "🏷" } else { "" };
      let certification = series.certification.clone().unwrap_or_default();
      let network = series.network.clone().unwrap_or_default();
      let size = series
        .statistics
        .as_ref()
        .map_or(0f64, |stats| convert_to_gb(stats.size_on_disk));
      let quality_profile = quality_profile_map
        .get_by_left(&series.quality_profile_id)
        .unwrap()
        .to_owned();
      let language_profile = language_profile_map
        .get_by_left(&series.language_profile_id)
        .unwrap()
        .to_owned();
      let empty_tag = String::new();
      let tags = if !series.tags.is_empty() {
        series
          .tags
          .iter()
          .map(|tag_id| {
            tags_map
              .get_by_left(&tag_id.as_i64().unwrap())
              .unwrap_or(&empty_tag)
              .clone()
          })
          .collect::<Vec<String>>()
          .join(", ")
      } else {
        String::new()
      };

      decorate_series_row_with_style(
        series,
        Row::new(vec![
          Cell::from(series.title.to_string()),
          Cell::from(series.year.to_string()),
          Cell::from(network),
          Cell::from(series.status.to_display_str()),
          Cell::from(certification),
          Cell::from(series.series_type.to_display_str()),
          Cell::from(quality_profile),
          Cell::from(language_profile),
          Cell::from(format!("{size:.2} GB")),
          Cell::from(monitored.to_owned()),
          Cell::from(tags),
        ]),
      )
    };
    let series_table = ManagarrTable::new(content, series_table_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .sorting(active_sonarr_block == ActiveSonarrBlock::SeriesSortPrompt)
      .searching(active_sonarr_block == ActiveSonarrBlock::SearchSeries)
      .filtering(active_sonarr_block == ActiveSonarrBlock::FilterSeries)
      .search_produced_empty_results(active_sonarr_block == ActiveSonarrBlock::SearchSeriesError)
      .filter_produced_empty_results(active_sonarr_block == ActiveSonarrBlock::FilterSeriesError)
      .headers([
        "Title",
        "Year",
        "Network",
        "Status",
        "Rating",
        "Type",
        "Quality Profile",
        "Language",
        "Size",
        "Monitored",
        "Tags",
      ])
      .constraints([
        Constraint::Percentage(20),
        Constraint::Percentage(4),
        Constraint::Percentage(14),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(11),
        Constraint::Percentage(8),
        Constraint::Percentage(7),
        Constraint::Percentage(6),
        Constraint::Percentage(12),
      ]);

    if [
      ActiveSonarrBlock::SearchSeries,
      ActiveSonarrBlock::FilterSeries,
    ]
    .contains(&active_sonarr_block)
    {
      series_table.show_cursor(f, area);
    }

    f.render_widget(series_table, area);
  }
}

fn decorate_series_row_with_style<'a>(series: &Series, row: Row<'a>) -> Row<'a> {
  if !series.monitored {
    return row.unmonitored();
  }

  match series.status {
    SeriesStatus::Ended => {
      if let Some(ref seasons) = series.seasons {
        return if seasons
          .iter()
          .filter(|season| season.monitored)
          .filter(|season| season.statistics.is_some())
          .all(|season| {
            season
              .statistics
              .as_ref()
              .expect("Season Statistics is undefined")
              .episode_file_count
              == season
                .statistics
                .as_ref()
                .expect("Season statistics is undefined")
                .episode_count
          }) {
          row.downloaded()
        } else {
          row.missing()
        };
      }

      row.indeterminate()
    }
    SeriesStatus::Continuing => {
      if let Some(ref seasons) = series.seasons {
        return if seasons
          .iter()
          .filter(|season| season.monitored)
          .filter(|season| season.statistics.is_some())
          .all(|season| {
            season
              .statistics
              .as_ref()
              .expect("Season Statistics is undefined")
              .episode_file_count
              == season
                .statistics
                .as_ref()
                .expect("Season statistics is undefined")
                .episode_count
          }) {
          row.unreleased()
        } else {
          row.missing()
        };
      }

      row.indeterminate()
    }
    SeriesStatus::Upcoming => row.unreleased(),
    _ => row.indeterminate(),
  }
}
