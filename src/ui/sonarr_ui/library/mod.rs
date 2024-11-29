use ratatui::{
  layout::{Constraint, Rect},
  widgets::{Cell, Row},
  Frame,
};

use crate::{
  app::App,
  models::{
    servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SERIES_BLOCKS},
    sonarr_models::{Series, SeriesStatus},
    EnumDisplayStyle, Route,
  },
  ui::{
    styles::ManagarrStyle,
    utils::{get_width_from_percentage, layout_block_top_border},
    widgets::managarr_table::ManagarrTable,
    DrawUi,
  },
  utils::convert_runtime,
};

#[cfg(test)]
#[path = "library_ui_tests.rs"]
mod library_ui_tests;

pub(super) struct LibraryUi;

impl DrawUi for LibraryUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return SERIES_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    let route = app.get_current_route();
    let mut series_ui_matchers = |active_sonarr_block: ActiveSonarrBlock| match active_sonarr_block
    {
      ActiveSonarrBlock::Series | ActiveSonarrBlock::SeriesSortPrompt => draw_series(f, app, area),
      _ => (),
    };

    match route {
      Route::Sonarr(active_sonarr_block, _) if SERIES_BLOCKS.contains(&active_sonarr_block) => {
        series_ui_matchers(active_sonarr_block)
      }
      _ => (),
    }
  }
}

pub(super) fn draw_series(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
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
    let help_footer = app
      .data
      .sonarr_data
      .main_tabs
      .get_active_tab_contextual_help();

    let series_table_row_mapping = |series: &Series| {
      series.title.scroll_left_or_reset(
        get_width_from_percentage(area, 27),
        *series == current_selection,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let monitored = if series.monitored { "🏷" } else { "" };
      let (hours, minutes) = convert_runtime(series.runtime);
      let certification = series.certification.clone().unwrap_or_default();
      let network = series.network.clone().unwrap_or_default();
      let quality_profile = quality_profile_map
        .get_by_left(&series.quality_profile_id)
        .unwrap()
        .to_owned();
      let language_profile = language_profile_map
        .get_by_left(&series.language_profile_id)
        .unwrap()
        .to_owned();
      let tags = if !series.tags.is_empty() {
        series
          .tags
          .iter()
          .map(|tag_id| {
            tags_map
              .get_by_left(&tag_id.as_i64().unwrap())
              .unwrap()
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
          Cell::from(format!("{hours}h {minutes}m")),
          Cell::from(certification),
          Cell::from(series.series_type.to_display_str()),
          Cell::from(quality_profile),
          Cell::from(language_profile),
          Cell::from(monitored.to_owned()),
          Cell::from(tags),
        ]),
      )
    };
    let series_table = ManagarrTable::new(content, series_table_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .footer(help_footer)
      .sorting(active_sonarr_block == ActiveSonarrBlock::SeriesSortPrompt)
      .headers([
        "Title",
        "Year",
        "Network",
        "Runtime",
        "Rating",
        "Type",
        "Quality Profile",
        "Language Profile",
        "Monitored",
        "Tags",
      ])
      .constraints([
        Constraint::Percentage(27),
        Constraint::Percentage(4),
        Constraint::Percentage(10),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(13),
        Constraint::Percentage(10),
        Constraint::Percentage(6),
        Constraint::Percentage(12),
      ]);

    f.render_widget(series_table, area);
  }
}

fn decorate_series_row_with_style<'a>(series: &Series, row: Row<'a>) -> Row<'a> {
  match series.status {
    SeriesStatus::Ended => {
      if let Some(ref stats) = series.statistics {
        if stats.percent_of_episodes == 100.0 {
          return row.downloaded();
        }
      }

      row.missing()
    }
    SeriesStatus::Continuing => {
      if let Some(ref stats) = series.statistics {
        if stats.percent_of_episodes == 100.0 {
          return row.unreleased();
        }
      }

      row.missing()
    }
    SeriesStatus::Upcoming => row.unreleased(),
    _ => row.missing(),
  }
}
