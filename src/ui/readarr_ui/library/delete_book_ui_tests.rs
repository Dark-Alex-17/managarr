#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, DELETE_BOOK_BLOCKS, DELETE_BOOK_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::library::delete_book_ui::{
    DeleteBookPromptHighlights, DeleteBookUi, delete_book_prompt_highlights,
  };

  #[test]
  fn test_delete_book_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if DELETE_BOOK_BLOCKS.contains(&active_readarr_block) {
        assert!(DeleteBookUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!DeleteBookUi::accepts(active_readarr_block.into()));
      }
    });
  }

  #[rstest]
  #[case(
    ActiveReadarrBlock::DeleteBookToggleDeleteFile,
    DeleteBookPromptHighlights {
      delete_files: true,
      add_import_list_exclusion: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::DeleteBookToggleAddListExclusion,
    DeleteBookPromptHighlights {
      delete_files: false,
      add_import_list_exclusion: true,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::DeleteBookConfirmPrompt,
    DeleteBookPromptHighlights {
      delete_files: false,
      add_import_list_exclusion: false,
      confirm: true,
    }
  )]
  #[case(
    ActiveReadarrBlock::BookDetails,
    DeleteBookPromptHighlights {
      delete_files: false,
      add_import_list_exclusion: false,
      confirm: false,
    }
  )]
  fn test_delete_book_prompt_highlights(
    #[case] selected_block: ActiveReadarrBlock,
    #[case] expected_highlights: DeleteBookPromptHighlights,
  ) {
    let highlights = delete_book_prompt_highlights(selected_block);

    assert_eq!(highlights, expected_highlights);
  }

  #[test]
  fn test_delete_book_prompt_highlights_covers_every_selection_step() {
    for step in DELETE_BOOK_SELECTION_BLOCKS {
      let highlights = delete_book_prompt_highlights(step[0]);

      assert_ne!(
        highlights,
        delete_book_prompt_highlights(ActiveReadarrBlock::BookDetails)
      );
    }
  }

  mod snapshot_tests {
    use rstest::rstest;

    use super::*;
    use crate::app::App;
    use crate::models::BlockSelectionState;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    fn seed_distinct_delete_preferences(app: &mut App<'_>) {
      app.data.readarr_data.delete_files = true;
      app.data.readarr_data.add_import_list_exclusion = false;
    }

    #[rstest]
    #[case(0, ActiveReadarrBlock::DeleteBookToggleDeleteFile)]
    #[case(1, ActiveReadarrBlock::DeleteBookToggleAddListExclusion)]
    #[case(2, ActiveReadarrBlock::DeleteBookConfirmPrompt)]
    fn test_delete_book_ui_renders_step(
      #[case] step: usize,
      #[case] selected_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::DeleteBookPrompt.into());
      seed_distinct_delete_preferences(&mut app);
      let mut selected_block = BlockSelectionState::new(DELETE_BOOK_SELECTION_BLOCKS);
      selected_block.set_index(0, step);
      app.data.readarr_data.selected_block = selected_block;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DeleteBookUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("delete_book_step_{step}_{selected_readarr_block}"),
        output
      );
    }

    #[test]
    fn test_delete_book_ui_renders_inverted_delete_preferences() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::DeleteBookPrompt.into());
      app.data.readarr_data.delete_files = false;
      app.data.readarr_data.add_import_list_exclusion = true;
      app.data.readarr_data.selected_block = BlockSelectionState::new(DELETE_BOOK_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DeleteBookUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    #[case(ActiveReadarrBlock::DeleteBookConfirmPrompt)]
    #[case(ActiveReadarrBlock::DeleteBookToggleDeleteFile)]
    #[case(ActiveReadarrBlock::DeleteBookToggleAddListExclusion)]
    fn test_delete_book_ui_renders_nothing_when_route_is_not_the_prompt(
      #[case] active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      seed_distinct_delete_preferences(&mut app);
      app.data.readarr_data.selected_block = BlockSelectionState::new(DELETE_BOOK_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DeleteBookUi::draw(f, app, f.area());
      });

      assert!(!output.contains("Delete Book"));
      assert!(!output.contains("Add List Exclusion"));
    }
  }
}
