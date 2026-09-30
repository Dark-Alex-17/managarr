#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::readarr::readarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ActiveReadarrBlock, ROOT_FOLDERS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::root_folders::RootFoldersUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_root_folders_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_readarr_block)
        || ADD_ROOT_FOLDER_BLOCKS.contains(&active_readarr_block)
      {
        assert!(RootFoldersUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!RootFoldersUi::accepts(active_readarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;
    use crate::models::BlockSelectionState;
    use crate::models::Scrollable;
    use crate::models::servarr_data::readarr::readarr_data::ADD_ROOT_FOLDER_SELECTION_BLOCKS;
    use crate::models::servarr_models::{RootFolder, UnmappedFolder};
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    fn seed_distinct_root_folders(app: &mut App<'_>) {
      app.data.readarr_data.root_folders.set_items(vec![
        RootFolder {
          id: 1,
          path: "/nfs/books".to_owned(),
          accessible: true,
          free_space: 219902325555200,
          unmapped_folders: None,
        },
        RootFolder {
          id: 2,
          path: "/nfs/audiobooks".to_owned(),
          accessible: false,
          free_space: 1099511627776,
          unmapped_folders: Some(vec![
            UnmappedFolder {
              name: "Unsorted Anthologies".to_owned(),
              path: "/nfs/audiobooks/unsorted-anthologies".to_owned(),
            },
            UnmappedFolder {
              name: "Pending Imports".to_owned(),
              path: "/nfs/audiobooks/pending-imports".to_owned(),
            },
            UnmappedFolder {
              name: "Damaged Rips".to_owned(),
              path: "/nfs/audiobooks/damaged-rips".to_owned(),
            },
          ]),
        },
      ]);
    }

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

    #[test]
    fn test_root_folders_ui_renders_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_root_folders_ui_renders_empty_root_folders() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_root_folders_ui_renders_root_folders_tab(
      #[values(
        ActiveReadarrBlock::RootFolders,
        ActiveReadarrBlock::DeleteRootFolderPrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_readarr_block.to_string(), output);
    }

    #[rstest]
    fn test_root_folders_ui_renders_distinct_root_folders(
      #[values(
        ActiveReadarrBlock::RootFolders,
        ActiveReadarrBlock::DeleteRootFolderPrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      seed_distinct_root_folders(&mut app);
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("distinct_root_folders_{active_readarr_block}"),
        output
      );
    }

    #[test]
    fn test_root_folders_ui_renders_delete_prompt_for_non_default_selection() {
      let mut app = App::test_default_fully_populated();
      seed_distinct_root_folders(&mut app);
      app.data.readarr_data.root_folders.scroll_down();
      app.push_navigation_stack(ActiveReadarrBlock::DeleteRootFolderPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_root_folders_ui_renders_add_root_folder_popup_over_root_folders_table(
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
      seed_distinct_profile_lists(&mut app);
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_readarr_block.to_string(), output);
    }
  }
}
