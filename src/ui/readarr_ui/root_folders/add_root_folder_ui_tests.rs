#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::readarr::readarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ADD_ROOT_FOLDER_SELECTION_BLOCKS, ActiveReadarrBlock,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::root_folders::add_root_folder_ui::{
    AddRootFolderPromptHighlights, AddRootFolderUi, add_root_folder_prompt_highlights,
  };

  #[test]
  fn test_add_root_folder_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if ADD_ROOT_FOLDER_BLOCKS.contains(&active_readarr_block) {
        assert!(AddRootFolderUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!AddRootFolderUi::accepts(active_readarr_block.into()));
      }
    });
  }

  #[rstest]
  #[case(
    ActiveReadarrBlock::AddRootFolderNameInput,
    AddRootFolderPromptHighlights {
      name: true,
      path: false,
      monitor: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddRootFolderPathInput,
    AddRootFolderPromptHighlights {
      name: false,
      path: true,
      monitor: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddRootFolderSelectMonitor,
    AddRootFolderPromptHighlights {
      name: false,
      path: false,
      monitor: true,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems,
    AddRootFolderPromptHighlights {
      name: false,
      path: false,
      monitor: false,
      monitor_new_items: true,
      quality_profile: false,
      metadata_profile: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
    AddRootFolderPromptHighlights {
      name: false,
      path: false,
      monitor: false,
      monitor_new_items: false,
      quality_profile: true,
      metadata_profile: false,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddRootFolderSelectMetadataProfile,
    AddRootFolderPromptHighlights {
      name: false,
      path: false,
      monitor: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: true,
      tags: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddRootFolderTagsInput,
    AddRootFolderPromptHighlights {
      name: false,
      path: false,
      monitor: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      tags: true,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::AddRootFolderConfirmPrompt,
    AddRootFolderPromptHighlights {
      name: false,
      path: false,
      monitor: false,
      monitor_new_items: false,
      quality_profile: false,
      metadata_profile: false,
      tags: false,
      confirm: true,
    }
  )]
  fn test_add_root_folder_prompt_highlights(
    #[case] selected_block: ActiveReadarrBlock,
    #[case] expected_highlights: AddRootFolderPromptHighlights,
  ) {
    let highlights = add_root_folder_prompt_highlights(selected_block);

    assert_eq!(highlights, expected_highlights);
  }

  #[test]
  fn test_add_root_folder_prompt_highlights_covers_every_selection_step() {
    for step in ADD_ROOT_FOLDER_SELECTION_BLOCKS {
      let highlights = add_root_folder_prompt_highlights(step[0]);

      assert_ne!(
        highlights,
        add_root_folder_prompt_highlights(ActiveReadarrBlock::RootFolders)
      );
    }
  }

  mod snapshot_tests {
    use super::*;
    use crate::app::App;
    use crate::models::BlockSelectionState;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    fn seed_distinct_profile_lists(app: &mut App<'_>) {
      let add_root_folder_modal = app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .expect("add_root_folder_modal must exist in this context");
      add_root_folder_modal
        .quality_profile_list
        .set_items(vec!["eBook".to_owned()]);
      add_root_folder_modal
        .metadata_profile_list
        .set_items(vec!["Author Only".to_owned()]);
    }

    #[rstest]
    fn test_add_root_folder_ui_renders(
      #[values(
        ActiveReadarrBlock::AddRootFolderPrompt,
        ActiveReadarrBlock::AddRootFolderConfirmPrompt,
        ActiveReadarrBlock::AddRootFolderNameInput,
        ActiveReadarrBlock::AddRootFolderPathInput,
        ActiveReadarrBlock::AddRootFolderSelectMonitor,
        ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems,
        ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
        ActiveReadarrBlock::AddRootFolderSelectMetadataProfile,
        ActiveReadarrBlock::AddRootFolderTagsInput
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      seed_distinct_profile_lists(&mut app);
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddRootFolderUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("add_root_folder_{active_readarr_block}"), output);
    }

    #[rstest]
    #[case(0, ActiveReadarrBlock::AddRootFolderNameInput)]
    #[case(1, ActiveReadarrBlock::AddRootFolderPathInput)]
    #[case(2, ActiveReadarrBlock::AddRootFolderSelectMonitor)]
    #[case(3, ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems)]
    #[case(4, ActiveReadarrBlock::AddRootFolderSelectQualityProfile)]
    #[case(5, ActiveReadarrBlock::AddRootFolderSelectMetadataProfile)]
    #[case(6, ActiveReadarrBlock::AddRootFolderTagsInput)]
    #[case(7, ActiveReadarrBlock::AddRootFolderConfirmPrompt)]
    fn test_add_root_folder_modal_ui_renders_step(
      #[case] step: usize,
      #[case] selected_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      seed_distinct_profile_lists(&mut app);
      let mut selected_block = BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      selected_block.set_index(0, step);
      app.data.readarr_data.selected_block = selected_block;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddRootFolderUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("add_root_folder_step_{step}_{selected_readarr_block}"),
        output
      );
    }
  }
}
