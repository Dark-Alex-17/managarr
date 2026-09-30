#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, HISTORY_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::history::HistoryUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_history_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if HISTORY_BLOCKS.contains(&active_readarr_block) {
        assert!(HistoryUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!HistoryUi::accepts(active_readarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use chrono::DateTime;
    use rstest::rstest;

    use crate::models::readarr_models::{
      ReadarrHistoryData, ReadarrHistoryEventType, ReadarrHistoryItem,
    };
    use crate::models::servarr_models::{Quality, QualityWrapper};
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_history_ui_renders_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::History.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_history_ui_renders_empty(
      #[values(ActiveReadarrBlock::History, ActiveReadarrBlock::HistoryItemDetails)]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("empty_history_tab_{active_readarr_block}"), output);
    }

    #[rstest]
    fn test_history_ui_renders(
      #[values(
        ActiveReadarrBlock::History,
        ActiveReadarrBlock::HistoryItemDetails,
        ActiveReadarrBlock::HistorySortPrompt,
        ActiveReadarrBlock::FilterHistory,
        ActiveReadarrBlock::FilterHistoryError,
        ActiveReadarrBlock::SearchHistory,
        ActiveReadarrBlock::SearchHistoryError
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("history_tab_{active_readarr_block}"), output);
    }

    #[rstest]
    fn test_history_ui_renders_second_item_selected(
      #[values(ActiveReadarrBlock::History, ActiveReadarrBlock::HistoryItemDetails)]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.history.set_items(history_vec());
      app.data.readarr_data.history.select_index(Some(1));
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("history_second_item_selected_{active_readarr_block}"),
        output
      );
    }

    fn history_vec() -> Vec<ReadarrHistoryItem> {
      vec![
        ReadarrHistoryItem {
          id: 1,
          author_id: 1,
          book_id: 1,
          source_title: "First Source Title".into(),
          quality: QualityWrapper {
            quality: Quality {
              name: "EPUB".to_owned(),
            },
          },
          date: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap()),
          event_type: ReadarrHistoryEventType::Grabbed,
          data: ReadarrHistoryData {
            indexer: Some("DrunkenSlug".to_owned()),
            nzb_info_url: Some("https://drunkenslug.com/details/first".to_owned()),
            release_group: Some("FirstReleaseGroup".to_owned()),
            age: Some("3".to_owned()),
            published_date: Some(DateTime::from(
              DateTime::parse_from_rfc3339("2023-05-18T08:15:00Z").unwrap(),
            )),
            download_client_name: Some("nzbget".to_owned()),
            ..ReadarrHistoryData::default()
          },
        },
        ReadarrHistoryItem {
          id: 2,
          author_id: 2,
          book_id: 2,
          source_title: "Second Source Title".into(),
          quality: QualityWrapper {
            quality: Quality {
              name: "MOBI".to_owned(),
            },
          },
          date: DateTime::from(DateTime::parse_from_rfc3339("2024-08-14T09:03:47Z").unwrap()),
          event_type: ReadarrHistoryEventType::DownloadFailed,
          data: ReadarrHistoryData {
            download_client_name: Some("sabnzbd".to_owned()),
            message: Some("The download client reported a failure".to_owned()),
            release_group: Some("SecondReleaseGroup".to_owned()),
            indexer: Some("Nyaa".to_owned()),
            ..ReadarrHistoryData::default()
          },
        },
      ]
    }
  }
}
