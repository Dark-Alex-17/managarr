#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, EDIT_AUTHOR_BLOCKS, EDIT_AUTHOR_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::library::edit_author_ui::{
    EditAuthorPromptHighlights, EditAuthorUi, build_edit_author_prompt_title,
    edit_author_prompt_highlights,
  };

  #[test]
  fn test_edit_author_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if EDIT_AUTHOR_BLOCKS.contains(&active_readarr_block) {
        assert!(EditAuthorUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!EditAuthorUi::accepts(active_readarr_block.into()));
      }
    });
  }

  #[test]
  fn test_build_edit_author_prompt_title_without_disambiguation() {
    let title = build_edit_author_prompt_title("Test Author", "");

    assert_str_eq!(title, "Edit - Test Author");
  }

  #[test]
  fn test_build_edit_author_prompt_title_with_disambiguation() {
    let title = build_edit_author_prompt_title("Test Author", "American novelist");

    assert_str_eq!(title, "Edit - Test Author (American novelist)");
  }

  #[rstest]
  #[case(
    ActiveReadarrBlock::EditAuthorToggleMonitored,
    EditAuthorPromptHighlights {
      monitored: true,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      path: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditAuthorSelectMonitorNewItems,
    EditAuthorPromptHighlights {
      monitored: false,
      monitor_new_items: true,
      quality_profile: false,
      metadata_profile: false,
      path: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditAuthorSelectQualityProfile,
    EditAuthorPromptHighlights {
      monitored: false,
      monitor_new_items: false,
      quality_profile: true,
      metadata_profile: false,
      path: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditAuthorSelectMetadataProfile,
    EditAuthorPromptHighlights {
      monitored: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: true,
      path: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditAuthorPathInput,
    EditAuthorPromptHighlights {
      monitored: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      path: true,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditAuthorTagsInput,
    EditAuthorPromptHighlights {
      monitored: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      path: false,
      tags: true,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditAuthorConfirmPrompt,
    EditAuthorPromptHighlights {
      monitored: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      path: false,
      tags: false,
      confirm: true,
    }
  )]
  fn test_edit_author_prompt_highlights(
    #[case] selected_block: ActiveReadarrBlock,
    #[case] expected_highlights: EditAuthorPromptHighlights,
  ) {
    let highlights = edit_author_prompt_highlights(selected_block);

    assert_eq!(highlights, expected_highlights);
  }

  #[test]
  fn test_edit_author_prompt_highlights_covers_every_selection_step() {
    for step in EDIT_AUTHOR_SELECTION_BLOCKS {
      let highlights = edit_author_prompt_highlights(step[0]);

      assert_ne!(
        highlights,
        edit_author_prompt_highlights(ActiveReadarrBlock::Authors)
      );
    }
  }

  mod snapshot_tests {
    use rstest::rstest;

    use super::*;
    use crate::app::App;
    use crate::models::BlockSelectionState;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    fn seed_distinct_profile_lists(app: &mut App<'_>) {
      let edit_author_modal = app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .expect("edit_author_modal must exist in this context");
      edit_author_modal
        .quality_profile_list
        .set_items(vec!["eBook".to_owned()]);
      edit_author_modal
        .metadata_profile_list
        .set_items(vec!["Author Only".to_owned()]);
    }

    #[rstest]
    #[case(0, ActiveReadarrBlock::EditAuthorToggleMonitored)]
    #[case(1, ActiveReadarrBlock::EditAuthorSelectMonitorNewItems)]
    #[case(2, ActiveReadarrBlock::EditAuthorSelectQualityProfile)]
    #[case(3, ActiveReadarrBlock::EditAuthorSelectMetadataProfile)]
    #[case(4, ActiveReadarrBlock::EditAuthorPathInput)]
    #[case(5, ActiveReadarrBlock::EditAuthorTagsInput)]
    #[case(6, ActiveReadarrBlock::EditAuthorConfirmPrompt)]
    fn test_edit_author_ui_renders_step(
      #[case] step: usize,
      #[case] selected_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      seed_distinct_profile_lists(&mut app);
      let mut selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      selected_block.set_index(0, step);
      app.data.readarr_data.selected_block = selected_block;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditAuthorUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("edit_author_step_{step}_{selected_readarr_block}"),
        output
      );
    }

    #[rstest]
    #[case(1, ActiveReadarrBlock::EditAuthorSelectMonitorNewItems)]
    #[case(2, ActiveReadarrBlock::EditAuthorSelectQualityProfile)]
    #[case(3, ActiveReadarrBlock::EditAuthorSelectMetadataProfile)]
    #[case(4, ActiveReadarrBlock::EditAuthorPathInput)]
    #[case(5, ActiveReadarrBlock::EditAuthorTagsInput)]
    fn test_edit_author_ui_renders_active_block(
      #[case] step: usize,
      #[case] active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      seed_distinct_profile_lists(&mut app);
      let mut selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      selected_block.set_index(0, step);
      app.data.readarr_data.selected_block = selected_block;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditAuthorUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("edit_author_active_{active_readarr_block}"), output);
    }

    #[test]
    fn test_edit_author_ui_renders_unmonitored_author() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      seed_distinct_profile_lists(&mut app);
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .expect("edit_author_modal must exist in this context")
        .monitored = Some(false);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditAuthorUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edit_author_ui_renders_over_author_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      seed_distinct_profile_lists(&mut app);
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app.push_navigation_stack(
        (
          ActiveReadarrBlock::EditAuthorPrompt,
          Some(ActiveReadarrBlock::AuthorDetails),
        )
          .into(),
      );

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditAuthorUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
