#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::indexers::IndexersHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::radarr_data::{
    ActiveRadarrBlock, INDEXERS_BLOCKS, INDEXER_SETTINGS_BLOCKS,
  };
  use crate::test_handler_delegation;

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::models::radarr_models::Indexer;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_indexers_scroll,
      IndexersHandler,
      indexers,
      simple_stateful_iterable_vec!(Indexer, String, protocol),
      ActiveRadarrBlock::Indexers,
      None,
      protocol
    );
  }

  mod test_handle_home_end {
    use crate::models::radarr_models::Indexer;
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_indexers_home_end,
      IndexersHandler,
      indexers,
      extended_stateful_iterable_vec!(Indexer, String, protocol),
      ActiveRadarrBlock::Indexers,
      None,
      protocol
    );
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use crate::assert_delete_prompt;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_indexer_prompt() {
      assert_delete_prompt!(
        IndexersHandler,
        ActiveRadarrBlock::Indexers,
        ActiveRadarrBlock::DeleteIndexerPrompt
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_indexers_tab_left() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(4);

      IndexersHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::Indexers,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
    }

    #[test]
    fn test_indexers_tab_right() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(4);

      IndexersHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::Indexers,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::System.into()
      );
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::System.into());
    }

    #[rstest]
    fn test_left_right_delete_indexer_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();

      IndexersHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::DeleteIndexerPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      IndexersHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::DeleteIndexerPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_indexer_submit_aka_edit() {
      let mut app = App::default();

      IndexersHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::Indexers, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditIndexer.into()
      );
    }

    #[test]
    fn test_delete_indexer_prompt_confirm_submit() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteIndexerPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::DeleteIndexer)
      );
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Indexers.into());
    }

    #[test]
    fn test_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteIndexerPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Indexers.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_delete_indexer_prompt_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteIndexerPrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      IndexersHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteIndexerPrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Indexers.into());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_default_esc() {
      let mut app = App::default();
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());

      IndexersHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::Indexers, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Indexers.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use crate::assert_refresh_key;
    use crate::models::servarr_data::radarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS;

    use super::*;

    #[test]
    fn test_indexer_add() {
      let mut app = App::default();

      IndexersHandler::with(
        &DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        &ActiveRadarrBlock::Indexers,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddIndexer.into()
      );
    }

    #[test]
    fn test_refresh_indexers_key() {
      assert_refresh_key!(IndexersHandler, ActiveRadarrBlock::Indexers);
    }

    #[test]
    fn test_indexer_settings_key() {
      let mut app = App::default();

      IndexersHandler::with(
        &DEFAULT_KEYBINDINGS.settings.key,
        &mut app,
        &ActiveRadarrBlock::Indexers,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::IndexerSettingsPrompt.into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block.blocks,
        &INDEXER_SETTINGS_SELECTION_BLOCKS
      );
    }
  }

  #[rstest]
  fn test_delegates_indexer_settings_blocks_to_indexer_settings_handler(
    #[values(
      ActiveRadarrBlock::IndexerSettingsPrompt,
      ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
      ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
      ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveRadarrBlock::IndexerSettingsRetentionInput,
      ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
      ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
      ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
      ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      IndexersHandler,
      ActiveRadarrBlock::Indexers,
      active_radarr_block
    );
  }

  #[test]
  fn test_indexers_handler_accepts() {
    let mut indexers_blocks = Vec::new();
    indexers_blocks.extend(INDEXERS_BLOCKS);
    indexers_blocks.extend(INDEXER_SETTINGS_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if indexers_blocks.contains(&active_radarr_block) {
        assert!(IndexersHandler::accepts(&active_radarr_block));
      } else {
        assert!(!IndexersHandler::accepts(&active_radarr_block));
      }
    })
  }
}
