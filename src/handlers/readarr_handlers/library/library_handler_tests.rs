#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::Number;
  use std::cmp::Ordering;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::library::{LibraryHandler, authors_sorting_options};
  use crate::models::HorizontallyScrollableText;
  use crate::models::readarr_models::{Author, AuthorStatistics, AuthorStatus};
  use crate::models::servarr_data::readarr::readarr_data::{
    AUTHOR_DETAILS_BLOCKS, ActiveReadarrBlock, BOOK_DETAILS_BLOCKS, EDITION_DETAILS_BLOCKS,
    LIBRARY_BLOCKS,
  };
  use crate::test_handler_delegation;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_str_eq;

    use super::*;
    use crate::simple_stateful_iterable_vec;

    #[rstest]
    fn test_authors_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(simple_stateful_iterable_vec!(
          Author,
          HorizontallyScrollableText,
          author_name
        ));

      LibraryHandler::new(key, &mut app, ActiveReadarrBlock::Authors, None).handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .authors
          .current_selection()
          .author_name
          .text,
        "Test 2"
      );

      LibraryHandler::new(key, &mut app, ActiveReadarrBlock::Authors, None).handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .authors
          .current_selection()
          .author_name
          .text,
        "Test 1"
      );
    }

    #[rstest]
    fn test_authors_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(simple_stateful_iterable_vec!(
          Author,
          HorizontallyScrollableText,
          author_name
        ));

      LibraryHandler::new(key, &mut app, ActiveReadarrBlock::Authors, None).handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .authors
          .current_selection()
          .author_name
          .text,
        "Test 1"
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_str_eq;

    use super::*;
    use crate::extended_stateful_iterable_vec;

    #[test]
    fn test_authors_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(extended_stateful_iterable_vec!(
          Author,
          HorizontallyScrollableText,
          author_name
        ));

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .authors
          .current_selection()
          .author_name
          .text,
        "Test 3"
      );

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .authors
          .current_selection()
          .author_name
          .text,
        "Test 1"
      );
    }

    #[test]
    fn test_authors_home_end_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(extended_stateful_iterable_vec!(
          Author,
          HorizontallyScrollableText,
          author_name
        ));

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .authors
          .current_selection()
          .author_name
          .text,
        "Test 1"
      );
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_delete_prompt;
    use crate::models::servarr_data::readarr::readarr_data::DELETE_AUTHOR_SELECTION_BLOCKS;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_author_delete() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      assert_delete_prompt!(
        LibraryHandler,
        app,
        ActiveReadarrBlock::Authors,
        ActiveReadarrBlock::DeleteAuthorPrompt
      );
      assert_eq!(
        app.data.readarr_data.selected_block.blocks,
        DELETE_AUTHOR_SELECTION_BLOCKS
      );
    }

    #[test]
    fn test_author_delete_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      LibraryHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::Authors, None).handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_pushed;

    #[rstest]
    fn test_authors_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(0);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::System.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::System.into());
    }

    #[rstest]
    fn test_authors_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(0);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::Downloads.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::Downloads.into());
    }

    #[rstest]
    fn test_left_right_update_all_authors_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

      LibraryHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::UpdateAllAuthorsPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);

      LibraryHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::UpdateAllAuthorsPrompt,
        None,
      )
      .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_navigation_popped;
    use crate::assert_navigation_pushed;
    use crate::network::readarr_network::ReadarrEvent;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_author_details_submit() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      LibraryHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::Authors, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AuthorDetails.into());
    }

    #[test]
    fn test_author_details_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      LibraryHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::Authors, None).handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
    }

    #[test]
    fn test_update_all_authors_prompt_confirm_submit() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);
      app.data.readarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::UpdateAllAuthorsPrompt.into());

      LibraryHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::UpdateAllAuthorsPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &ReadarrEvent::UpdateAllAuthors
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
    }

    #[test]
    fn test_update_all_authors_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::UpdateAllAuthorsPrompt.into());

      LibraryHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::UpdateAllAuthorsPrompt,
        None,
      )
      .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_update_all_authors_prompt_blocks_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::UpdateAllAuthorsPrompt.into());
      app.data.readarr_data.prompt_confirm = true;

      LibraryHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::UpdateAllAuthorsPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      LibraryHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::Authors, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::models::servarr_data::readarr::readarr_data::EDIT_AUTHOR_SELECTION_BLOCKS;
    use crate::network::readarr_network::ReadarrEvent;
    use crate::{
      assert_modal_absent, assert_modal_present, assert_navigation_popped, assert_navigation_pushed,
    };

    #[test]
    fn test_author_add_key() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AddAuthorSearchInput.into());
      assert!(app.ignore_special_keys_for_textbox_input);
      assert_modal_present!(app.data.readarr_data.add_author_search);
    }

    #[test]
    fn test_author_add_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_modal_absent!(app.data.readarr_data.add_author_search);
    }

    #[test]
    fn test_author_edit_key() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);
      app.data.readarr_data.quality_profile_map =
        bimap::BiMap::from_iter([(0i64, "Default Quality".to_owned())]);
      app.data.readarr_data.metadata_profile_map =
        bimap::BiMap::from_iter([(0i64, "Default Metadata".to_owned())]);
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::EditAuthorPrompt.into());
      assert_modal_present!(app.data.readarr_data.edit_author_modal);
      assert_eq!(
        app.data.readarr_data.selected_block.blocks,
        EDIT_AUTHOR_SELECTION_BLOCKS
      );
    }

    #[test]
    fn test_author_edit_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
      assert_modal_absent!(app.data.readarr_data.edit_author_modal);
    }

    #[test]
    fn test_toggle_monitoring_key() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.is_routing = false;

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
      assert!(app.data.readarr_data.prompt_confirm);
      assert!(app.is_routing);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &ReadarrEvent::ToggleAuthorMonitoring(0)
      );
    }

    #[test]
    fn test_toggle_monitoring_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.is_routing = false;

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
      assert!(!app.data.readarr_data.prompt_confirm);
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert!(!app.is_routing);
    }

    #[test]
    fn test_update_all_authors_key() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::UpdateAllAuthorsPrompt.into());
    }

    #[test]
    fn test_update_all_authors_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
    }

    #[test]
    fn test_refresh_authors_key() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_authors_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::Authors,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Authors.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_update_all_authors_prompt_confirm() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![Author::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::UpdateAllAuthorsPrompt.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::UpdateAllAuthorsPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &ReadarrEvent::UpdateAllAuthors
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
    }
  }

  #[test]
  fn test_authors_sorting_options_name() {
    let expected_cmp_fn: fn(&Author, &Author) -> Ordering = |a, b| {
      a.author_name
        .text
        .to_lowercase()
        .cmp(&b.author_name.text.to_lowercase())
    };
    let mut expected_authors_vec = authors_vec();
    expected_authors_vec.sort_by(expected_cmp_fn);

    let sort_option = authors_sorting_options()[0].clone();
    let mut sorted_authors_vec = authors_vec();
    sorted_authors_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_authors_vec, expected_authors_vec);
    assert_str_eq!(sort_option.name, "Name");
  }

  #[test]
  fn test_authors_sorting_options_status() {
    let expected_cmp_fn: fn(&Author, &Author) -> Ordering = |a, b| {
      a.status
        .to_string()
        .to_lowercase()
        .cmp(&b.status.to_string().to_lowercase())
    };
    let mut expected_authors_vec = authors_vec();
    expected_authors_vec.sort_by(expected_cmp_fn);

    let sort_option = authors_sorting_options()[1].clone();
    let mut sorted_authors_vec = authors_vec();
    sorted_authors_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_authors_vec, expected_authors_vec);
    assert_str_eq!(sort_option.name, "Status");
  }

  #[test]
  fn test_authors_sorting_options_quality_profile() {
    let expected_cmp_fn: fn(&Author, &Author) -> Ordering =
      |a, b| a.quality_profile_id.cmp(&b.quality_profile_id);
    let mut expected_authors_vec = authors_vec();
    expected_authors_vec.sort_by(expected_cmp_fn);

    let sort_option = authors_sorting_options()[2].clone();
    let mut sorted_authors_vec = authors_vec();
    sorted_authors_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_authors_vec, expected_authors_vec);
    assert_str_eq!(sort_option.name, "Quality Profile");
  }

  #[test]
  fn test_authors_sorting_options_metadata_profile() {
    let expected_cmp_fn: fn(&Author, &Author) -> Ordering =
      |a, b| a.metadata_profile_id.cmp(&b.metadata_profile_id);
    let mut expected_authors_vec = authors_vec();
    expected_authors_vec.sort_by(expected_cmp_fn);

    let sort_option = authors_sorting_options()[3].clone();
    let mut sorted_authors_vec = authors_vec();
    sorted_authors_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_authors_vec, expected_authors_vec);
    assert_str_eq!(sort_option.name, "Metadata Profile");
  }

  #[test]
  fn test_authors_sorting_options_books() {
    let expected_cmp_fn: fn(&Author, &Author) -> Ordering = |a, b| {
      a.statistics
        .as_ref()
        .map_or(0, |stats| stats.book_count)
        .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.book_count))
    };
    let mut expected_authors_vec = authors_vec();
    expected_authors_vec.sort_by(expected_cmp_fn);

    let sort_option = authors_sorting_options()[4].clone();
    let mut sorted_authors_vec = authors_vec();
    sorted_authors_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_authors_vec, expected_authors_vec);
    assert_str_eq!(sort_option.name, "Books");
  }

  #[test]
  fn test_authors_sorting_options_size() {
    let expected_cmp_fn: fn(&Author, &Author) -> Ordering = |a, b| {
      a.statistics
        .as_ref()
        .map_or(0, |stats| stats.size_on_disk)
        .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.size_on_disk))
    };
    let mut expected_authors_vec = authors_vec();
    expected_authors_vec.sort_by(expected_cmp_fn);

    let sort_option = authors_sorting_options()[5].clone();
    let mut sorted_authors_vec = authors_vec();
    sorted_authors_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_authors_vec, expected_authors_vec);
    assert_str_eq!(sort_option.name, "Size");
  }

  #[test]
  fn test_authors_sorting_options_monitored() {
    let expected_cmp_fn: fn(&Author, &Author) -> Ordering = |a, b| a.monitored.cmp(&b.monitored);
    let mut expected_authors_vec = authors_vec();
    expected_authors_vec.sort_by(expected_cmp_fn);

    let sort_option = authors_sorting_options()[6].clone();
    let mut sorted_authors_vec = authors_vec();
    sorted_authors_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_authors_vec, expected_authors_vec);
    assert_str_eq!(sort_option.name, "Monitored");
  }

  #[test]
  fn test_authors_sorting_options_tags() {
    let expected_cmp_fn: fn(&Author, &Author) -> Ordering = |a, b| {
      let a_str = a
        .tags
        .iter()
        .map(|tag| tag.as_i64().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
      let b_str = b
        .tags
        .iter()
        .map(|tag| tag.as_i64().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");

      a_str.cmp(&b_str)
    };
    let mut expected_authors_vec = authors_vec();
    expected_authors_vec.sort_by(expected_cmp_fn);

    let sort_option = authors_sorting_options()[7].clone();
    let mut sorted_authors_vec = authors_vec();
    sorted_authors_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_authors_vec, expected_authors_vec);
    assert_str_eq!(sort_option.name, "Tags");
  }

  #[test]
  fn test_delegates_book_details_blocks_to_book_details_handler() {
    test_handler_delegation!(
      LibraryHandler,
      ActiveReadarrBlock::AuthorDetails,
      ActiveReadarrBlock::BookDetails
    );
  }

  #[test]
  fn test_library_handler_accepts() {
    let mut library_handler_blocks = Vec::new();
    library_handler_blocks.extend(LIBRARY_BLOCKS);
    library_handler_blocks.extend(AUTHOR_DETAILS_BLOCKS);
    library_handler_blocks.extend(BOOK_DETAILS_BLOCKS);
    library_handler_blocks.extend(EDITION_DETAILS_BLOCKS);

    ActiveReadarrBlock::iter().for_each(|readarr_block| {
      if library_handler_blocks.contains(&readarr_block) {
        assert!(
          LibraryHandler::accepts(readarr_block),
          "{readarr_block} is not accepted by the LibraryHandler"
        );
      } else {
        assert!(!LibraryHandler::accepts(readarr_block));
      }
    });
  }

  #[rstest]
  fn test_library_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_library_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;
    app
      .data
      .readarr_data
      .authors
      .set_items(vec![Author::default()]);

    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Authors,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_library_handler_not_ready_when_authors_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Authors,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_library_handler_ready_when_not_loading_and_authors_is_not_empty() {
    let mut app = App::test_default();
    app.is_loading = false;
    app
      .data
      .readarr_data
      .authors
      .set_items(vec![Author::default()]);

    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Authors,
      None,
    );

    assert!(handler.is_ready());
  }

  fn authors_vec() -> Vec<Author> {
    vec![
      Author {
        id: 3,
        author_name: "Test Author 1".into(),
        status: AuthorStatus::Ended,
        quality_profile_id: 1,
        metadata_profile_id: 1,
        monitored: false,
        tags: vec![Number::from(1), Number::from(2)],
        statistics: Some(AuthorStatistics {
          book_count: 5,
          size_on_disk: 789,
          ..AuthorStatistics::default()
        }),
        ..Author::default()
      },
      Author {
        id: 2,
        author_name: "Test Author 2".into(),
        status: AuthorStatus::Continuing,
        quality_profile_id: 2,
        metadata_profile_id: 2,
        monitored: false,
        tags: vec![Number::from(1), Number::from(3)],
        statistics: Some(AuthorStatistics {
          book_count: 10,
          size_on_disk: 456,
          ..AuthorStatistics::default()
        }),
        ..Author::default()
      },
      Author {
        id: 1,
        author_name: "Test Author 3".into(),
        status: AuthorStatus::Deleted,
        quality_profile_id: 3,
        metadata_profile_id: 3,
        monitored: true,
        tags: vec![Number::from(2), Number::from(3)],
        statistics: Some(AuthorStatistics {
          book_count: 3,
          size_on_disk: 123,
          ..AuthorStatistics::default()
        }),
        ..Author::default()
      },
    ]
  }
}
