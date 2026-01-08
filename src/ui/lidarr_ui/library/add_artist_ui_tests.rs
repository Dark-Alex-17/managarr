#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::lidarr::lidarr_data::{ADD_ARTIST_BLOCKS, ActiveLidarrBlock};
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::add_artist_ui::AddArtistUi;

  #[test]
  fn test_add_artist_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ADD_ARTIST_BLOCKS.contains(&active_lidarr_block) {
        assert!(AddArtistUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!AddArtistUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;
    use crate::app::App;
    use crate::models::servarr_data::lidarr::lidarr_data::ADD_ARTIST_SELECTION_BLOCKS;
    use crate::models::{BlockSelectionState, HorizontallyScrollableText};
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};
    use rstest::rstest;

    #[test]
    fn test_add_artist_ui_renders_loading_for_search() {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.add_artist_search = Some(HorizontallyScrollableText::default());
      app.data.lidarr_data.add_searched_artists = None;
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddArtistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_add_artist_ui_renders(
      #[values(
        ActiveLidarrBlock::AddArtistSearchInput,
        ActiveLidarrBlock::AddArtistSearchResults,
        ActiveLidarrBlock::AddArtistEmptySearchResults
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.add_artist_search = Some("Test Artist".into());
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddArtistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("add_artist_ui_{active_lidarr_block}"), output);
    }

    #[rstest]
    #[case(ActiveLidarrBlock::AddArtistPrompt)]
    #[case(ActiveLidarrBlock::AddArtistConfirmPrompt)]
    #[case(ActiveLidarrBlock::AddArtistSelectMonitor)]
    #[case(ActiveLidarrBlock::AddArtistSelectMonitorNewItems)]
    #[case(ActiveLidarrBlock::AddArtistSelectQualityProfile)]
    #[case(ActiveLidarrBlock::AddArtistSelectMetadataProfile)]
    #[case(ActiveLidarrBlock::AddArtistSelectRootFolder)]
    #[case(ActiveLidarrBlock::AddArtistTagsInput)]
    fn test_add_artist_modal_ui_renders(#[case] active_lidarr_block: ActiveLidarrBlock) {
      use crate::models::lidarr_models::{MonitorType, NewItemMonitorType};
      use crate::models::servarr_data::lidarr::modals::AddArtistModal;
      use crate::models::servarr_models::RootFolder;
      use strum::IntoEnumIterator;

      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(ADD_ARTIST_SELECTION_BLOCKS);

      let mut add_artist_modal = AddArtistModal {
        tags: "test".into(),
        ..AddArtistModal::default()
      };
      add_artist_modal
        .monitor_list
        .set_items(Vec::from_iter(MonitorType::iter()));
      add_artist_modal
        .monitor_new_items_list
        .set_items(Vec::from_iter(NewItemMonitorType::iter()));
      add_artist_modal
        .quality_profile_list
        .set_items(vec!["Any".to_owned()]);
      add_artist_modal
        .metadata_profile_list
        .set_items(vec!["Standard".to_owned()]);
      add_artist_modal
        .root_folder_list
        .set_items(vec![RootFolder {
          path: "/nfs/music".to_owned(),
          ..RootFolder::default()
        }]);
      app.data.lidarr_data.add_artist_modal = Some(add_artist_modal);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddArtistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("add_artist_modal_{active_lidarr_block}"), output);
    }

    #[test]
    fn test_add_artist_already_in_library_ui_renders() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistAlreadyInLibrary.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddArtistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
