#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::{ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS};
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::indexers::edit_indexer_settings_handler::IndexerSettingsHandler;
  use crate::handlers::KeyEventHandler;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::INDEXER_SETTINGS_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;

    use super::*;

    #[rstest]
    fn test_edit_indexer_settings_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.next();

      IndexerSettingsHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::IndexerSettingsPrompt,
        &None,
      )
      .handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::IndexerSettingsMinimumAgeInput
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::IndexerSettingsMaximumSizeInput
        );
      }
    }
  }

  mod test_handle_home_end {}

  mod test_handle_left_right_action {}

  mod test_handle_submit {}

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use crate::app::radarr::radarr_test_utils::utils::create_test_radarr_data;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_edit_indexer_settings_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data = create_test_radarr_data();

      IndexerSettingsHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::IndexerSettingsPrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Indexers.into());
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.indexer_settings, None);
    }
  }

  mod test_handle_key_char {}

  #[test]
  fn test_indexer_settings_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if INDEXER_SETTINGS_BLOCKS.contains(&active_radarr_block) {
        assert!(IndexerSettingsHandler::accepts(&active_radarr_block));
      } else {
        assert!(!IndexerSettingsHandler::accepts(&active_radarr_block));
      }
    })
  }
}
