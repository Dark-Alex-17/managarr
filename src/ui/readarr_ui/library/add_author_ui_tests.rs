#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::readarr::readarr_data::{
    ADD_AUTHOR_BLOCKS, ADD_AUTHOR_SELECTION_BLOCKS, ActiveReadarrBlock,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::library::add_author_ui::{
    AddAuthorPromptHighlights, AddAuthorUi, add_author_prompt_highlights,
    already_in_library_marker, build_add_author_prompt_title,
  };

  #[test]
  fn test_add_author_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if ADD_AUTHOR_BLOCKS.contains(&active_readarr_block) {
        assert!(AddAuthorUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!AddAuthorUi::accepts(active_readarr_block.into()));
      }
    });
  }

  #[test]
  fn test_build_add_author_prompt_title_without_disambiguation() {
    let title = build_add_author_prompt_title("Test Author", "");

    assert_str_eq!(title, "Add - Test Author");
  }

  #[test]
  fn test_build_add_author_prompt_title_with_disambiguation() {
    let title = build_add_author_prompt_title("Test Author", "American novelist");

    assert_str_eq!(title, "Add - Test Author (American novelist)");
  }

  #[test]
  fn test_already_in_library_marker_when_in_library() {
    let marker = already_in_library_marker(true);

    assert_str_eq!(marker, "✔");
  }

  #[test]
  fn test_already_in_library_marker_when_not_in_library() {
    let marker = already_in_library_marker(false);

    assert_str_eq!(marker, "");
  }

  #[rstest]
  #[case(
    ActiveReadarrBlock::AddAuthorSelectRootFolder,
    AddAuthorPromptHighlights {
      root_folder: true,
      monitor: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddAuthorSelectMonitor,
    AddAuthorPromptHighlights {
      root_folder: false,
      monitor: true,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
    AddAuthorPromptHighlights {
      root_folder: false,
      monitor: false,
      monitor_new_items: true,
      quality_profile: false,
      metadata_profile: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddAuthorSelectQualityProfile,
    AddAuthorPromptHighlights {
      root_folder: false,
      monitor: false,
      monitor_new_items: false,
      quality_profile: true,
      metadata_profile: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
    AddAuthorPromptHighlights {
      root_folder: false,
      monitor: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: true,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddAuthorTagsInput,
    AddAuthorPromptHighlights {
      root_folder: false,
      monitor: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      tags: true,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddAuthorConfirmPrompt,
    AddAuthorPromptHighlights {
      root_folder: false,
      monitor: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      tags: false,
      confirm: true,
    }
  )]
  fn test_add_author_prompt_highlights(
    #[case] selected_block: ActiveReadarrBlock,
    #[case] expected_highlights: AddAuthorPromptHighlights,
  ) {
    let highlights = add_author_prompt_highlights(selected_block);

    assert_eq!(highlights, expected_highlights);
  }

  #[test]
  fn test_add_author_prompt_highlights_covers_every_selection_step() {
    for step in ADD_AUTHOR_SELECTION_BLOCKS {
      let highlights = add_author_prompt_highlights(step[0]);

      assert_ne!(
        highlights,
        add_author_prompt_highlights(ActiveReadarrBlock::Authors)
      );
    }
  }

  mod snapshot_tests {
    use rstest::rstest;

    use super::*;
    use crate::app::App;
    use crate::models::BlockSelectionState;
    use crate::models::HorizontallyScrollableText;
    use crate::models::servarr_data::readarr::readarr_data::ADD_AUTHOR_SELECTION_BLOCKS;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    fn seed_distinct_profile_lists(app: &mut App<'_>) {
      let add_author_modal = app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .expect("add_author_modal must exist in this context");
      add_author_modal
        .quality_profile_list
        .set_items(vec!["eBook".to_owned()]);
      add_author_modal
        .metadata_profile_list
        .set_items(vec!["Author Only".to_owned()]);
    }

    #[test]
    fn test_add_author_ui_renders_loading_for_search() {
      let mut app = App::test_default_fully_populated();
      app.data.readarr_data.add_author_search = Some(HorizontallyScrollableText::default());
      app.data.readarr_data.add_searched_authors = None;
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddAuthorUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_add_author_ui_renders(
      #[values(
        ActiveReadarrBlock::AddAuthorSearchInput,
        ActiveReadarrBlock::AddAuthorSearchResults,
        ActiveReadarrBlock::AddAuthorEmptySearchResults
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddAuthorUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("add_author_ui_{active_readarr_block}"), output);
    }

    #[rstest]
    #[case(0, ActiveReadarrBlock::AddAuthorSelectRootFolder)]
    #[case(1, ActiveReadarrBlock::AddAuthorSelectMonitor)]
    #[case(2, ActiveReadarrBlock::AddAuthorSelectMonitorNewItems)]
    #[case(3, ActiveReadarrBlock::AddAuthorSelectQualityProfile)]
    #[case(4, ActiveReadarrBlock::AddAuthorSelectMetadataProfile)]
    #[case(5, ActiveReadarrBlock::AddAuthorTagsInput)]
    #[case(6, ActiveReadarrBlock::AddAuthorConfirmPrompt)]
    fn test_add_author_modal_ui_renders_step(
      #[case] step: usize,
      #[case] selected_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
      seed_distinct_profile_lists(&mut app);
      let mut selected_block = BlockSelectionState::new(ADD_AUTHOR_SELECTION_BLOCKS);
      selected_block.set_index(0, step);
      app.data.readarr_data.selected_block = selected_block;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddAuthorUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("add_author_modal_step_{step}_{selected_readarr_block}"),
        output
      );
    }

    #[rstest]
    #[case(0, ActiveReadarrBlock::AddAuthorSelectRootFolder)]
    #[case(1, ActiveReadarrBlock::AddAuthorSelectMonitor)]
    #[case(2, ActiveReadarrBlock::AddAuthorSelectMonitorNewItems)]
    #[case(3, ActiveReadarrBlock::AddAuthorSelectQualityProfile)]
    #[case(4, ActiveReadarrBlock::AddAuthorSelectMetadataProfile)]
    #[case(5, ActiveReadarrBlock::AddAuthorTagsInput)]
    fn test_add_author_modal_ui_renders_active_block(
      #[case] step: usize,
      #[case] active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      seed_distinct_profile_lists(&mut app);
      let mut selected_block = BlockSelectionState::new(ADD_AUTHOR_SELECTION_BLOCKS);
      selected_block.set_index(0, step);
      app.data.readarr_data.selected_block = selected_block;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddAuthorUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("add_author_modal_active_{active_readarr_block}"),
        output
      );
    }

    #[test]
    fn test_add_author_already_in_library_ui_renders() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorAlreadyInLibrary.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddAuthorUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
