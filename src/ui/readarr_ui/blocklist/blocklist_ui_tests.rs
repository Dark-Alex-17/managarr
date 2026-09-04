#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, BLOCKLIST_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::blocklist::BlocklistUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_blocklist_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if BLOCKLIST_BLOCKS.contains(&active_readarr_block) {
        assert!(BlocklistUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!BlocklistUi::accepts(active_readarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use chrono::DateTime;
    use rstest::rstest;

    use crate::models::readarr_models::{Author, BlocklistItem};
    use crate::models::servarr_models::{Quality, QualityWrapper};
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_blocklist_ui_renders_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Blocklist.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_blocklist_ui_renders_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Blocklist.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_blocklist_ui_renders(
      #[values(
        ActiveReadarrBlock::Blocklist,
        ActiveReadarrBlock::BlocklistItemDetails,
        ActiveReadarrBlock::DeleteBlocklistItemPrompt,
        ActiveReadarrBlock::BlocklistClearAllItemsPrompt,
        ActiveReadarrBlock::BlocklistSortPrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("blocklist_tab_{active_readarr_block}"), output);
    }

    #[rstest]
    fn test_blocklist_ui_renders_second_item_selected(
      #[values(
        ActiveReadarrBlock::Blocklist,
        ActiveReadarrBlock::BlocklistItemDetails,
        ActiveReadarrBlock::DeleteBlocklistItemPrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.blocklist.set_items(blocklist_vec());
      app.data.readarr_data.blocklist.select_index(Some(1));
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("blocklist_second_item_selected_{active_readarr_block}"),
        output
      );
    }

    fn blocklist_vec() -> Vec<BlocklistItem> {
      vec![
        BlocklistItem {
          id: 1,
          author_id: 1,
          source_title: "First Source Title".to_owned(),
          quality: QualityWrapper {
            quality: Quality {
              name: "EPUB".to_owned(),
            },
          },
          date: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap()),
          protocol: "usenet".to_owned(),
          indexer: "DrunkenSlug".to_owned(),
          message: "first blocklist message".to_owned(),
          author: Author {
            id: 1,
            author_name: "First Author".into(),
            ..Author::default()
          },
          ..BlocklistItem::default()
        },
        BlocklistItem {
          id: 2,
          author_id: 2,
          source_title: "Second Source Title".to_owned(),
          quality: QualityWrapper {
            quality: Quality {
              name: "MOBI".to_owned(),
            },
          },
          date: DateTime::from(DateTime::parse_from_rfc3339("2024-08-14T09:03:47Z").unwrap()),
          protocol: "torrent".to_owned(),
          indexer: "Nyaa".to_owned(),
          message: "second blocklist message".to_owned(),
          author: Author {
            id: 2,
            author_name: "Second Author".into(),
            ..Author::default()
          },
          ..BlocklistItem::default()
        },
      ]
    }
  }
}
