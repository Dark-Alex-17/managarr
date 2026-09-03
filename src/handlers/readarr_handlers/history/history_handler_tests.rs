#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  use chrono::DateTime;
  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::handlers::readarr_handlers::history::history_sorting_options;
  use crate::models::readarr_models::{ReadarrHistoryEventType, ReadarrHistoryItem};
  use crate::models::servarr_models::{Quality, QualityWrapper};

  #[test]
  fn test_history_sorting_options_source_title() {
    let expected_cmp_fn: fn(&ReadarrHistoryItem, &ReadarrHistoryItem) -> Ordering = |a, b| {
      a.source_title
        .text
        .to_lowercase()
        .cmp(&b.source_title.text.to_lowercase())
    };
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[0].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Source Title");
  }

  #[test]
  fn test_history_sorting_options_event_type() {
    let expected_cmp_fn: fn(&ReadarrHistoryItem, &ReadarrHistoryItem) -> Ordering = |a, b| {
      a.event_type
        .to_string()
        .to_lowercase()
        .cmp(&b.event_type.to_string().to_lowercase())
    };
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[1].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Event Type");
  }

  #[test]
  fn test_history_sorting_options_quality() {
    let expected_cmp_fn: fn(&ReadarrHistoryItem, &ReadarrHistoryItem) -> Ordering = |a, b| {
      a.quality
        .quality
        .name
        .to_lowercase()
        .cmp(&b.quality.quality.name.to_lowercase())
    };
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[2].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  #[test]
  fn test_history_sorting_options_date() {
    let expected_cmp_fn: fn(&ReadarrHistoryItem, &ReadarrHistoryItem) -> Ordering =
      |a, b| a.date.cmp(&b.date);
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[3].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Date");
  }

  fn history_vec() -> Vec<ReadarrHistoryItem> {
    vec![
      ReadarrHistoryItem {
        id: 3,
        source_title: "test 1".into(),
        event_type: ReadarrHistoryEventType::Grabbed,
        quality: QualityWrapper {
          quality: Quality {
            name: "EPUB".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-01-10T07:28:45Z").unwrap()),
        ..ReadarrHistoryItem::default()
      },
      ReadarrHistoryItem {
        id: 2,
        source_title: "test 2".into(),
        event_type: ReadarrHistoryEventType::DownloadImported,
        quality: QualityWrapper {
          quality: Quality {
            name: "AZW3".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
        ..ReadarrHistoryItem::default()
      },
      ReadarrHistoryItem {
        id: 1,
        source_title: "test 3".into(),
        event_type: ReadarrHistoryEventType::BookFileDeleted,
        quality: QualityWrapper {
          quality: Quality {
            name: "EPUB".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-03-10T07:28:45Z").unwrap()),
        ..ReadarrHistoryItem::default()
      },
    ]
  }
}
