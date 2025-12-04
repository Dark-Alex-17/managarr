use std::sync::atomic::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Layout;
use ratatui::widgets::ListItem;

use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::radarr::modals::EditMovieModal;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, EDIT_MOVIE_BLOCKS, MOVIE_DETAILS_BLOCKS,
};
use crate::render_selectable_input_box;
use crate::ui::radarr_ui::library::movie_details_ui::MovieDetailsUi;

use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{layout_paragraph_borderless, title_block_centered};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{DrawUi, draw_popup};

#[cfg(test)]
#[path = "edit_movie_ui_tests.rs"]
mod edit_movie_ui_tests;

pub(super) struct EditMovieUi;

impl DrawUi for EditMovieUi {
  fn accepts(route: Route) -> bool {
    let Route::Radarr(active_radarr_block, _) = route else {
      return false;
    };
    EDIT_MOVIE_BLOCKS.contains(&active_radarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = app.get_current_route() {
      if let Some(context) = context_option
        && MOVIE_DETAILS_BLOCKS.contains(&context)
      {
        draw_popup(f, app, MovieDetailsUi::draw, Size::Large);
      }

      draw_popup(f, app, draw_edit_movie_confirmation_prompt, Size::Medium);

      match active_radarr_block {
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability => {
          draw_edit_movie_select_minimum_availability_popup(f, app);
        }
        ActiveRadarrBlock::EditMovieSelectQualityProfile => {
          draw_edit_movie_select_quality_profile_popup(f, app);
        }
        _ => (),
      }
    }
  }
}

fn draw_edit_movie_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let movie_title = app
    .data
    .radarr_data
    .movies
    .current_selection()
    .title
    .text
    .clone();
  let movie_overview = app
    .data
    .radarr_data
    .movies
    .current_selection()
    .overview
    .clone();
  let title = format!("Edit - {movie_title}");
  f.render_widget(title_block_centered(&title), area);
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveRadarrBlock::EditMovieConfirmPrompt;
  let EditMovieModal {
    minimum_availability_list,
    quality_profile_list,
    monitored,
    path,
    tags,
  } = app
    .data
    .radarr_data
    .edit_movie_modal
    .as_ref()
    .expect("edit_movie_modal must exist in this context");
  let selected_minimum_availability = minimum_availability_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();

  let [
    paragraph_area,
    monitored_area,
    min_availability_area,
    quality_profile_area,
    path_area,
    tags_area,
    _,
    buttons_area,
  ] = Layout::vertical([
    Constraint::Length(6),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Fill(1),
    Constraint::Length(3),
  ])
  .margin(1)
  .areas(area);
  let [save_area, cancel_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  let prompt_paragraph = layout_paragraph_borderless(&movie_overview);
  let monitored_checkbox = Checkbox::new("Monitored")
    .checked(monitored.unwrap_or_default())
    .highlighted(selected_block == ActiveRadarrBlock::EditMovieToggleMonitored);
  let min_availability_drop_down_button = Button::new()
    .title(selected_minimum_availability.to_display_str())
    .label("Minimum Availability")
    .icon("▼")
    .selected(selected_block == ActiveRadarrBlock::EditMovieSelectMinimumAvailability);
  let quality_profile_drop_down_button = Button::new()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == ActiveRadarrBlock::EditMovieSelectQualityProfile);

  if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
    let path_input_box = InputBox::new(&path.text)
      .offset(path.offset.load(Ordering::SeqCst))
      .label("Path")
      .highlighted(selected_block == ActiveRadarrBlock::EditMoviePathInput)
      .selected(active_radarr_block == ActiveRadarrBlock::EditMoviePathInput);
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(selected_block == ActiveRadarrBlock::EditMovieTagsInput)
      .selected(active_radarr_block == ActiveRadarrBlock::EditMovieTagsInput);

    match active_radarr_block {
      ActiveRadarrBlock::EditMoviePathInput => path_input_box.show_cursor(f, path_area),
      ActiveRadarrBlock::EditMovieTagsInput => tags_input_box.show_cursor(f, tags_area),
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
  f.render_widget(min_availability_drop_down_button, min_availability_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(save_button, save_area);
  f.render_widget(cancel_button, cancel_area);
}

fn draw_edit_movie_select_minimum_availability_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let minimum_availability_list = SelectableList::new(
    &mut app
      .data
      .radarr_data
      .edit_movie_modal
      .as_mut()
      .expect("edit_movie_modal must exist in this context")
      .minimum_availability_list,
    |minimum_availability| ListItem::new(minimum_availability.to_display_str().to_owned()),
  );
  let popup = Popup::new(minimum_availability_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_edit_movie_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .radarr_data
      .edit_movie_modal
      .as_mut()
      .expect("edit_movie_modal must exist in this context")
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
