#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::LidarrHandler;
  use crate::models::lidarr_models::Artist;
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::models::servarr_data::lidarr::modals::EditArtistModal;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  #[rstest]
  fn test_lidarr_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = LidarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_lidarr_handler_is_ready() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = LidarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_lidarr_handler_accepts() {
    for lidarr_block in ActiveLidarrBlock::iter() {
      assert!(LidarrHandler::accepts(lidarr_block));
    }
  }

  #[rstest]
  fn test_delegates_library_blocks_to_library_handler(
    #[values(
      ActiveLidarrBlock::Artists,
      ActiveLidarrBlock::ArtistsSortPrompt,
      ActiveLidarrBlock::FilterArtists,
      ActiveLidarrBlock::FilterArtistsError,
      ActiveLidarrBlock::SearchArtists,
      ActiveLidarrBlock::SearchArtistsError,
      ActiveLidarrBlock::UpdateAllArtistsPrompt,
      ActiveLidarrBlock::DeleteArtistPrompt,
      ActiveLidarrBlock::EditArtistPrompt,
      ActiveLidarrBlock::EditArtistPathInput,
      ActiveLidarrBlock::EditArtistSelectMetadataProfile,
      ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
      ActiveLidarrBlock::EditArtistSelectQualityProfile,
      ActiveLidarrBlock::EditArtistTagsInput
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.push_navigation_stack(active_lidarr_block.into());

    LidarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      active_lidarr_block,
      None,
    )
    .handle();

    assert_eq!(app.get_current_route(), ActiveLidarrBlock::Artists.into());
  }
}
