#[cfg(test)]
mod tests {
  use chrono::Utc;
  use ratatui::{style::Stylize, text::Line};

  use crate::{
    models::sonarr_models::{SonarrHistoryData, SonarrHistoryItem},
    ui::{
      sonarr_ui::sonarr_ui_utils::{
        create_download_failed_history_event_details,
        create_download_folder_imported_history_event_details,
        create_episode_file_deleted_history_event_details,
        create_episode_file_renamed_history_event_details, create_grabbed_history_event_details,
        create_no_data_history_event_details,
      },
      styles::ManagarrStyle,
    },
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_create_grabbed_history_event_details() {
    let history_item = sonarr_history_item();
    let SonarrHistoryItem {
      source_title, data, ..
    } = history_item.clone();
    let SonarrHistoryData {
      indexer,
      release_group,
      series_match_type,
      nzb_info_url,
      download_client_name,
      age,
      published_date,
      ..
    } = data;
    let expected_vec = vec![
      Line::from(vec![
        "Source Title: ".bold().secondary(),
        source_title.text.secondary(),
      ]),
      Line::from(vec![
        "Indexer: ".bold().secondary(),
        indexer.unwrap_or_default().secondary(),
      ]),
      Line::from(vec![
        "Release Group: ".bold().secondary(),
        release_group.unwrap_or_default().secondary(),
      ]),
      Line::from(vec![
        "Series Match Type: ".bold().secondary(),
        series_match_type.unwrap_or_default().secondary(),
      ]),
      Line::from(vec![
        "NZB Info URL: ".bold().secondary(),
        nzb_info_url.unwrap_or_default().secondary(),
      ]),
      Line::from(vec![
        "Download Client Name: ".bold().secondary(),
        download_client_name.unwrap_or_default().secondary(),
      ]),
      Line::from(vec![
        "Age: ".bold().secondary(),
        format!("{} days", age.unwrap_or("0".to_owned())).secondary(),
      ]),
      Line::from(vec![
        "Published Date: ".bold().secondary(),
        published_date.unwrap_or_default().to_string().secondary(),
      ]),
    ];

    let history_details_vec = create_grabbed_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_download_folder_imported_history_event_details() {
    let history_item = sonarr_history_item();
    let SonarrHistoryItem {
      source_title, data, ..
    } = history_item.clone();
    let SonarrHistoryData {
      dropped_path,
      imported_path,
      ..
    } = data;
    let expected_vec = vec![
      Line::from(vec![
        "Source Title: ".bold().secondary(),
        source_title.text.secondary(),
      ]),
      Line::from(vec![
        "Dropped Path: ".bold().secondary(),
        dropped_path.unwrap_or_default().secondary(),
      ]),
      Line::from(vec![
        "Imported Path: ".bold().secondary(),
        imported_path.unwrap_or_default().secondary(),
      ]),
    ];

    let history_details_vec = create_download_folder_imported_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_download_failed_history_event_details() {
    let history_item = sonarr_history_item();
    let SonarrHistoryItem {
      source_title, data, ..
    } = history_item.clone();
    let SonarrHistoryData { message, .. } = data;
    let expected_vec = vec![
      Line::from(vec![
        "Source Title: ".bold().secondary(),
        source_title.text.secondary(),
      ]),
      Line::from(vec![
        "Message: ".bold().secondary(),
        message.unwrap_or_default().secondary(),
      ]),
    ];

    let history_details_vec = create_download_failed_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_episode_file_deleted_history_event_details() {
    let history_item = sonarr_history_item();
    let SonarrHistoryItem {
      source_title, data, ..
    } = history_item.clone();
    let SonarrHistoryData { reason, .. } = data;
    let expected_vec = vec![
      Line::from(vec![
        "Source Title: ".bold().secondary(),
        source_title.text.secondary(),
      ]),
      Line::from(vec![
        "Reason: ".bold().secondary(),
        reason.unwrap_or_default().secondary(),
      ]),
    ];

    let history_details_vec = create_episode_file_deleted_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_episode_file_renamed_history_event_details() {
    let history_item = sonarr_history_item();
    let SonarrHistoryItem {
      source_title, data, ..
    } = history_item.clone();
    let SonarrHistoryData {
      source_path,
      source_relative_path,
      path,
      relative_path,
      ..
    } = data;
    let expected_vec = vec![
      Line::from(vec![
        "Source Title: ".bold().secondary(),
        source_title.text.secondary(),
      ]),
      Line::from(vec![
        "Source Path: ".bold().secondary(),
        source_path.unwrap_or_default().secondary(),
      ]),
      Line::from(vec![
        "Source Relative Path: ".bold().secondary(),
        source_relative_path.unwrap_or_default().secondary(),
      ]),
      Line::from(vec![
        "Destination Path: ".bold().secondary(),
        path.unwrap_or_default().secondary(),
      ]),
      Line::from(vec![
        "Destination Relative Path: ".bold().secondary(),
        relative_path.unwrap_or_default().secondary(),
      ]),
    ];

    let history_details_vec = create_episode_file_renamed_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_no_data_history_event_details() {
    let history_item = sonarr_history_item();
    let SonarrHistoryItem { source_title, .. } = history_item.clone();
    let expected_vec = vec![
      Line::from(vec![
        "Source Title: ".bold().secondary(),
        source_title.text.secondary(),
      ]),
      Line::from(vec![String::new().secondary()]),
      Line::from(vec!["No additional data available".bold().secondary()]),
    ];

    let history_details_vec = create_no_data_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  fn sonarr_history_item() -> SonarrHistoryItem {
    SonarrHistoryItem {
      source_title: "test.source.title".into(),
      data: sonarr_history_data(),
      ..SonarrHistoryItem::default()
    }
  }

  fn sonarr_history_data() -> SonarrHistoryData {
    SonarrHistoryData {
      dropped_path: Some("/dropped/test".into()),
      imported_path: Some("/imported/test".into()),
      indexer: Some("Test Indexer".into()),
      release_group: Some("test release group".into()),
      series_match_type: Some("test match type".into()),
      nzb_info_url: Some("test url".into()),
      download_client_name: Some("test download client".into()),
      age: Some("1".into()),
      published_date: Some(Utc::now()),
      message: Some("test message".into()),
      reason: Some("test reason".into()),
      source_path: Some("/source/path".into()),
      source_relative_path: Some("/relative/source/path".into()),
      path: Some("/path".into()),
      relative_path: Some("/relative/path".into()),
    }
  }
}
