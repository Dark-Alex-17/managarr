#[cfg(test)]
mod tests {
  use crate::models::servarr_models::{
    DiskSpace, HostConfig, LogResponse, QueueEvent, SecurityConfig, Update,
  };
  use crate::models::sonarr_models::{SonarrSerdeable, SonarrTask, SonarrTaskName, SystemStatus};
  use crate::models::{HorizontallyScrollableText, ScrollableText};
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::{Network, RequestMethod};
  use chrono::DateTime;
  use indoc::formatdoc;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use reqwest::Client;
  use serde_json::json;
  use tokio_util::sync::CancellationToken;

  #[tokio::test]
  async fn test_handle_get_sonarr_host_config_event() {
    let host_config_response = json!({
      "bindAddress": "*",
      "port": 7878,
      "urlBase": "some.test.site/sonarr",
      "instanceName": "Sonarr",
      "applicationUrl": "https://some.test.site:7878/sonarr",
      "enableSsl": true,
      "sslPort": 9898,
      "sslCertPath": "/app/sonarr.pfx",
      "sslCertPassword": "test"
    });
    let response: HostConfig = serde_json::from_value(host_config_response.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(host_config_response),
      None,
      SonarrEvent::GetHostConfig,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::HostConfig(host_config) = network
      .handle_sonarr_event(SonarrEvent::GetHostConfig)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(host_config, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_logs_event() {
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|SonarrError|Some.Big.Bad.Exception|test exception",
      ),
      HorizontallyScrollableText::from("2023-05-20 21:29:16 UTC|INFO|TestLogger|test message"),
    ];
    let logs_response_json = json!({
      "page": 1,
      "pageSize": 500,
      "sortKey": "time",
      "sortDirection": "descending",
      "totalRecords": 2,
      "records": [
          {
              "time": "2023-05-20T21:29:16Z",
              "level": "info",
              "logger": "TestLogger",
              "message": "test message",
              "id": 1
          },
          {
              "time": "2023-05-20T21:29:16Z",
              "level": "fatal",
              "logger": "SonarrError",
              "exception": "test exception",
              "exceptionType": "Some.Big.Bad.Exception",
              "id": 2
          }
        ]
    });
    let response: LogResponse = serde_json::from_value(logs_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(logs_response_json),
      None,
      SonarrEvent::GetLogs(500),
      None,
      Some("pageSize=500&sortDirection=descending&sortKey=time"),
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::LogResponse(logs) = network
      .handle_sonarr_event(SonarrEvent::GetLogs(500))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.logs.items,
        expected_logs
      );
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .logs
        .current_selection()
        .text
        .contains("INFO"));
      assert_eq!(logs, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_diskspace_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([
        {
          "freeSpace": 1111,
          "totalSpace": 2222,
        },
        {
          "freeSpace": 3333,
          "totalSpace": 4444
        }
      ])),
      None,
      SonarrEvent::GetDiskSpace,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let disk_space_vec = vec![
      DiskSpace {
        free_space: 1111,
        total_space: 2222,
      },
      DiskSpace {
        free_space: 3333,
        total_space: 4444,
      },
    ];

    if let SonarrSerdeable::DiskSpaces(disk_space) = network
      .handle_sonarr_event(SonarrEvent::GetDiskSpace)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.disk_space_vec,
        disk_space_vec
      );
      assert_eq!(disk_space, disk_space_vec);
    }
  }

  #[tokio::test]
  async fn test_handle_get_queued_sonarr_events_event() {
    let queued_events_json = json!([{
        "name": "RefreshMonitoredDownloads",
        "commandName": "Refresh Monitored Downloads",
        "status": "completed",
        "queued": "2023-05-20T21:29:16Z",
        "started": "2023-05-20T21:29:16Z",
        "ended": "2023-05-20T21:29:16Z",
        "duration": "00:00:00.5111547",
        "trigger": "scheduled",
    }]);
    let response: Vec<QueueEvent> = serde_json::from_value(queued_events_json.clone()).unwrap();
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_event = QueueEvent {
      name: "RefreshMonitoredDownloads".to_owned(),
      command_name: "Refresh Monitored Downloads".to_owned(),
      status: "completed".to_owned(),
      queued: timestamp,
      started: Some(timestamp),
      ended: Some(timestamp),
      duration: Some("00:00:00.5111547".to_owned()),
      trigger: "scheduled".to_owned(),
    };

    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(queued_events_json),
      None,
      SonarrEvent::GetQueuedEvents,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::QueueEvents(events) = network
      .handle_sonarr_event(SonarrEvent::GetQueuedEvents)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.queued_events.items,
        vec![expected_event]
      );
      assert_eq!(events, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_security_config_event() {
    let security_config_response = json!({
      "authenticationMethod": "forms",
      "authenticationRequired": "disabledForLocalAddresses",
      "username": "test",
      "password": "some password",
      "apiKey": "someApiKey12345",
      "certificateValidation": "disabledForLocalAddresses",
    });
    let response: SecurityConfig =
      serde_json::from_value(security_config_response.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(security_config_response),
      None,
      SonarrEvent::GetSecurityConfig,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SecurityConfig(security_config) = network
      .handle_sonarr_event(SonarrEvent::GetSecurityConfig)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(security_config, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_status_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!({
        "version": "v1",
        "startTime": "2023-02-25T20:16:43Z"
      })),
      None,
      SonarrEvent::GetStatus,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let date_time = DateTime::from(DateTime::parse_from_rfc3339("2023-02-25T20:16:43Z").unwrap());

    if let SonarrSerdeable::SystemStatus(status) = network
      .handle_sonarr_event(SonarrEvent::GetStatus)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_str_eq!(app_arc.lock().await.data.sonarr_data.version, "v1");
      assert_eq!(app_arc.lock().await.data.sonarr_data.start_time, date_time);
      assert_eq!(
        status,
        SystemStatus {
          version: "v1".to_owned(),
          start_time: date_time
        }
      );
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_tasks_event() {
    let tasks_json = json!([{
        "name": "Application Update Check",
        "taskName": "ApplicationUpdateCheck",
        "interval": 360,
        "lastExecution": "2023-05-20T21:29:16Z",
        "nextExecution": "2023-05-20T21:29:16Z",
    },
    {
        "name": "Backup",
        "taskName": "Backup",
        "interval": 10080,
        "lastExecution": "2023-05-20T21:29:16Z",
        "nextExecution": "2023-05-20T21:29:16Z",
    }]);
    let response: Vec<SonarrTask> = serde_json::from_value(tasks_json.clone()).unwrap();
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_tasks = vec![
      SonarrTask {
        name: "Application Update Check".to_owned(),
        task_name: SonarrTaskName::ApplicationUpdateCheck,
        interval: 360,
        last_execution: timestamp,
        next_execution: timestamp,
      },
      SonarrTask {
        name: "Backup".to_owned(),
        task_name: SonarrTaskName::Backup,
        interval: 10080,
        last_execution: timestamp,
        next_execution: timestamp,
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(tasks_json),
      None,
      SonarrEvent::GetTasks,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Tasks(tasks) = network
      .handle_sonarr_event(SonarrEvent::GetTasks)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.tasks.items,
        expected_tasks
      );
      assert_eq!(tasks, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_updates_event() {
    let updates_json = json!([{
      "version": "4.3.2.1",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": true,
      "installedOn": "2023-04-15T02:02:53Z",
      "latest": true,
      "changes": {
        "new": [
          "Cool new thing"
        ],
        "fixed": [
          "Some bugs killed"
        ]
      },
    },
      {
        "version": "3.2.1.0",
        "releaseDate": "2023-04-15T02:02:53Z",
        "installed": false,
        "installedOn": "2023-04-15T02:02:53Z",
        "latest": false,
        "changes": {
          "new": [
            "Cool new thing (old)",
            "Other cool new thing (old)"
            ],
        },
    },
    {
      "version": "2.1.0",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": false,
      "latest": false,
      "changes": {
        "fixed": [
          "Killed bug 1",
          "Fixed bug 2"
        ]
      },
    }]);
    let response: Vec<Update> = serde_json::from_value(updates_json.clone()).unwrap();
    let line_break = "-".repeat(200);
    let expected_text = ScrollableText::with_string(formatdoc!(
      "
    The latest version of Sonarr is already installed

    4.3.2.1 - 2023-04-15 02:02:53 UTC (Currently Installed)
    {line_break}
    New:
      * Cool new thing
    Fixed:
      * Some bugs killed
    
    
    3.2.1.0 - 2023-04-15 02:02:53 UTC (Previously Installed)
    {line_break}
    New:
      * Cool new thing (old)
      * Other cool new thing (old)
    
    
    2.1.0 - 2023-04-15 02:02:53 UTC 
    {line_break}
    Fixed:
      * Killed bug 1
      * Fixed bug 2"
    ));
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(updates_json),
      None,
      SonarrEvent::GetUpdates,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Updates(updates) = network
      .handle_sonarr_event(SonarrEvent::GetUpdates)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_str_eq!(
        app_arc.lock().await.data.sonarr_data.updates.get_text(),
        expected_text.get_text()
      );
      assert_eq!(updates, response);
    }
  }

  #[tokio::test]
  async fn test_handle_start_sonarr_task_event() {
    let response = json!({ "test": "test"});
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "ApplicationUpdateCheck"
      })),
      Some(response.clone()),
      None,
      SonarrEvent::StartTask(SonarrTaskName::ApplicationUpdateCheck),
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .tasks
      .set_items(vec![SonarrTask {
        task_name: SonarrTaskName::default(),
        ..SonarrTask::default()
      }]);
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Value(value) = network
      .handle_sonarr_event(SonarrEvent::StartTask(
        SonarrTaskName::ApplicationUpdateCheck,
      ))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(value, response);
    }
  }
}
