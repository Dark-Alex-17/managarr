use crate::models::servarr_data::modals::IndexerTestResultModalItem;
use crate::models::servarr_models::{DiskSpace, QueueEvent};
use chrono::DateTime;

pub fn diskspace() -> DiskSpace {
  DiskSpace {
    free_space: 6500,
    total_space: 8675309,
  }
}

pub fn indexer_test_result() -> IndexerTestResultModalItem {
  IndexerTestResultModalItem {
    name: "DrunkenSlug".to_owned(),
    is_valid: false,
    validation_failures: "Some failure".into(),
  }
}

pub fn queued_event() -> QueueEvent {
  QueueEvent {
    trigger: "manual".to_string(),
    name: "Refresh Monitored Downloads".to_string(),
    command_name: "Refresh Monitored Downloads".to_string(),
    status: "completed".to_string(),
    queued: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:25:16Z").unwrap()),
    started: Some(DateTime::from(
      DateTime::parse_from_rfc3339("2023-05-20T21:25:30Z").unwrap(),
    )),
    ended: Some(DateTime::from(
      DateTime::parse_from_rfc3339("2023-05-20T21:28:33Z").unwrap(),
    )),
    duration: Some("00:03:03".to_owned()),
  }
}
