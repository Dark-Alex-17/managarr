use ratatui::Frame;
use ratatui::layout::Rect;

use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, DELETE_BOOK_BLOCKS};
use crate::ui::DrawUi;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::popup::{Popup, Size};

#[cfg(test)]
#[path = "delete_book_ui_tests.rs"]
mod delete_book_ui_tests;

pub(super) struct DeleteBookUi;

impl DrawUi for DeleteBookUi {
  fn accepts(route: Route) -> bool {
    let Route::Readarr(active_readarr_block, _) = route else {
      return false;
    };
    DELETE_BOOK_BLOCKS.contains(&active_readarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if matches!(
      app.get_current_route(),
      Route::Readarr(ActiveReadarrBlock::DeleteBookPrompt, _)
    ) {
      let selected_block = app.data.readarr_data.selected_block.get_active_block();
      let highlights = delete_book_prompt_highlights(selected_block);
      let prompt = format!(
        "Do you really want to delete the book: \n{}?",
        app.data.readarr_data.books.current_selection().title.text
      );
      let checkboxes = vec![
        Checkbox::new("Delete Book Files")
          .checked(app.data.readarr_data.delete_files)
          .highlighted(highlights.delete_files),
        Checkbox::new("Add List Exclusion")
          .checked(app.data.readarr_data.add_import_list_exclusion)
          .highlighted(highlights.add_import_list_exclusion),
      ];
      let confirmation_prompt = ConfirmationPrompt::new()
        .title("Delete Book")
        .prompt(&prompt)
        .checkboxes(checkboxes)
        .yes_no_highlighted(highlights.confirm)
        .yes_no_value(app.data.readarr_data.prompt_confirm);

      f.render_widget(
        Popup::new(confirmation_prompt).size(Size::MediumPrompt),
        f.area(),
      );
    }
  }
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
struct DeleteBookPromptHighlights {
  delete_files: bool,
  add_import_list_exclusion: bool,
  confirm: bool,
}

fn delete_book_prompt_highlights(selected_block: ActiveReadarrBlock) -> DeleteBookPromptHighlights {
  DeleteBookPromptHighlights {
    delete_files: selected_block == ActiveReadarrBlock::DeleteBookToggleDeleteFile,
    add_import_list_exclusion: selected_block
      == ActiveReadarrBlock::DeleteBookToggleAddListExclusion,
    confirm: selected_block == ActiveReadarrBlock::DeleteBookConfirmPrompt,
  }
}
