#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::HorizontallyScrollableText;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, ROOT_FOLDERS_BLOCKS};
  use crate::models::servarr_models::RootFolder;
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::root_folders::RootFoldersUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_root_folders_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_radarr_block) {
        assert!(RootFoldersUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!RootFoldersUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[test]
  fn test_root_folders_ui_renders_loading_state() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      RootFoldersUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_root_folders_ui_renders_empty_root_folders() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
    app.data.radarr_data.root_folders = StatefulTable::default();

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      RootFoldersUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_root_folders_ui_renders_with_root_folders() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
    app.data.radarr_data.root_folders = StatefulTable::default();
    app.data.radarr_data.root_folders.set_items(vec![
      RootFolder {
        path: "/movies".to_owned(),
        accessible: true,
        free_space: 1024 * 1024 * 1024 * 100,
        ..RootFolder::default()
      },
      RootFolder {
        path: "/media/movies".to_owned(),
        accessible: true,
        free_space: 1024 * 1024 * 1024 * 50,
        ..RootFolder::default()
      },
    ]);

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      RootFoldersUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_root_folders_ui_renders_add_root_folder() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());
    app.data.radarr_data.root_folders = StatefulTable::default();
    app.data.radarr_data.edit_root_folder = Some(HorizontallyScrollableText::default());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      RootFoldersUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
