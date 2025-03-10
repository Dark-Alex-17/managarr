use std::sync::atomic::Ordering;

use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Layout;
use ratatui::text::Text;
use ratatui::widgets::{ListItem, Paragraph};
use ratatui::Frame;

use crate::app::context_clues::{build_context_clue_string, CONFIRMATION_PROMPT_CONTEXT_CLUES};
use crate::app::App;
use crate::models::servarr_data::sonarr::modals::EditSeriesModal;
use crate::models::servarr_data::sonarr::sonarr_data::{
  ActiveSonarrBlock, EDIT_SERIES_BLOCKS, SERIES_DETAILS_BLOCKS,
};
use crate::models::Route;
use crate::render_selectable_input_box;

use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{layout_paragraph_borderless, title_block_centered};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{draw_popup, DrawUi};

use super::series_details_ui::SeriesDetailsUi;

#[cfg(test)]
#[path = "edit_series_ui_tests.rs"]
mod edit_series_ui_tests;

pub(super) struct EditSeriesUi;

impl DrawUi for EditSeriesUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return EDIT_SERIES_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Sonarr(active_sonarr_block, context_option) = app.get_current_route() {
      if let Some(context) = context_option {
        if SERIES_DETAILS_BLOCKS.contains(&context) {
          draw_popup(f, app, SeriesDetailsUi::draw, Size::Large);
        }
      }

      let draw_edit_series_prompt = |f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect| {
        draw_edit_series_confirmation_prompt(f, app, prompt_area);

        match active_sonarr_block {
          ActiveSonarrBlock::EditSeriesSelectSeriesType => {
            draw_edit_series_select_series_type_popup(f, app);
          }
          ActiveSonarrBlock::EditSeriesSelectQualityProfile => {
            draw_edit_series_select_quality_profile_popup(f, app);
          }
          ActiveSonarrBlock::EditSeriesSelectLanguageProfile => {
            draw_edit_series_select_language_profile_popup(f, app);
          }
          _ => (),
        }
      };

      draw_popup(f, app, draw_edit_series_prompt, Size::Long);
    }
  }
}

fn draw_edit_series_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let series_title = app
    .data
    .sonarr_data
    .series
    .current_selection()
    .title
    .text
    .clone();
  let series_overview = app
    .data
    .sonarr_data
    .series
    .current_selection()
    .overview
    .clone()
    .unwrap_or_default();
  let title = format!("Edit - {series_title}");
  f.render_widget(title_block_centered(&title), area);

  let yes_no_value = app.data.sonarr_data.prompt_confirm;
  let selected_block = app.data.sonarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveSonarrBlock::EditSeriesConfirmPrompt;
  let EditSeriesModal {
    series_type_list,
    quality_profile_list,
    language_profile_list,
    monitored,
    use_season_folders,
    path,
    tags,
  } = app.data.sonarr_data.edit_series_modal.as_ref().unwrap();
  let selected_series_type = series_type_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();
  let selected_language_profile = language_profile_list.current_selection();

  let [paragraph_area, monitored_area, season_folder_area, quality_profile_area, language_profile_area, series_type_area, path_area, tags_area, _, buttons_area, help_area] =
    Layout::vertical([
      Constraint::Length(6),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(1),
      Constraint::Length(3),
      Constraint::Length(1),
    ])
    .margin(1)
    .areas(area);
  let [save_area, cancel_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  let help_text = Text::from(build_context_clue_string(&CONFIRMATION_PROMPT_CONTEXT_CLUES).help());
  let help_paragraph = Paragraph::new(help_text).centered();
  let prompt_paragraph = layout_paragraph_borderless(&series_overview);
  let monitored_checkbox = Checkbox::new("Monitored")
    .checked(monitored.unwrap_or_default())
    .highlighted(selected_block == ActiveSonarrBlock::EditSeriesToggleMonitored);
  let season_folder_checkbox = Checkbox::new("Season Folder")
    .checked(use_season_folders.unwrap_or_default())
    .highlighted(selected_block == ActiveSonarrBlock::EditSeriesToggleSeasonFolder);
  let series_type_drop_down_button = Button::new()
    .title(selected_series_type.to_display_str())
    .label("Series Type")
    .icon("▼")
    .selected(selected_block == ActiveSonarrBlock::EditSeriesSelectSeriesType);
  let quality_profile_drop_down_button = Button::new()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == ActiveSonarrBlock::EditSeriesSelectQualityProfile);
  let language_profile_drop_down_button = Button::new()
    .title(selected_language_profile)
    .label("Language Profile")
    .icon("▼")
    .selected(selected_block == ActiveSonarrBlock::EditSeriesSelectLanguageProfile);

  if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
    let path_input_box = InputBox::new(&path.text)
      .offset(path.offset.load(Ordering::SeqCst))
      .label("Path")
      .highlighted(selected_block == ActiveSonarrBlock::EditSeriesPathInput)
      .selected(active_sonarr_block == ActiveSonarrBlock::EditSeriesPathInput);
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(selected_block == ActiveSonarrBlock::EditSeriesTagsInput)
      .selected(active_sonarr_block == ActiveSonarrBlock::EditSeriesTagsInput);

    match active_sonarr_block {
      ActiveSonarrBlock::EditSeriesPathInput => path_input_box.show_cursor(f, path_area),
      ActiveSonarrBlock::EditSeriesTagsInput => tags_input_box.show_cursor(f, tags_area),
      _ => (),
    }

    render_selectable_input_box!(path_input_box, f, path_area);
    render_selectable_input_box!(tags_input_box, f, tags_area);
  }

  let save_button = Button::new()
    .title("Save")
    .selected(yes_no_value && highlight_yes_no);
  let cancel_button = Button::new()
    .title("Cancel")
    .selected(!yes_no_value && highlight_yes_no);

  f.render_widget(prompt_paragraph, paragraph_area);
  f.render_widget(monitored_checkbox, monitored_area);
  f.render_widget(season_folder_checkbox, season_folder_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(language_profile_drop_down_button, language_profile_area);
  f.render_widget(series_type_drop_down_button, series_type_area);
  f.render_widget(save_button, save_area);
  f.render_widget(cancel_button, cancel_area);
  f.render_widget(help_paragraph, help_area);
}

fn draw_edit_series_select_series_type_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let series_type_list = SelectableList::new(
    &mut app
      .data
      .sonarr_data
      .edit_series_modal
      .as_mut()
      .unwrap()
      .series_type_list,
    |series_type| ListItem::new(series_type.to_display_str().to_owned()),
  );
  let popup = Popup::new(series_type_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_edit_series_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .sonarr_data
      .edit_series_modal
      .as_mut()
      .unwrap()
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_edit_series_select_language_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let language_profile_list = SelectableList::new(
    &mut app
      .data
      .sonarr_data
      .edit_series_modal
      .as_mut()
      .unwrap()
      .language_profile_list,
    |language_profile| ListItem::new(language_profile.clone()),
  );
  let popup = Popup::new(language_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
