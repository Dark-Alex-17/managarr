#[cfg(test)]
mod tests {
  use crate::models::servarr_models::{
    DiskSpace, HostConfig, LogResponse, QueueEvent, SecurityConfig, Update,
  };
  use crate::models::sonarr_models::{SonarrSerdeable, SonarrTask, SonarrTaskName, SystemStatus};
  use crate::models::{HorizontallyScrollableText, ScrollableText};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::sonarr_network::SonarrEvent;
  use chrono::DateTime;
  use indoc::formatdoc;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

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
    let (mock, app, _server) = MockServarrApi::get()
      .returns(host_config_response)
      .build_for(SonarrEvent::GetHostConfig)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::HostConfig(host_config) = network
      .handle_sonarr_event(SonarrEvent::GetHostConfig)
      .await
      .unwrap()
    else {
      panic!("Expected HostConfig")
    };
    mock.assert_async().await;
    assert_eq!(host_config, response);
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
    let (mock, app, _server) = MockServarrApi::get()
      .returns(logs_response_json)
      .query("pageSize=500&sortDirection=descending&sortKey=time")
      .build_for(SonarrEvent::GetLogs(500))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::LogResponse(logs) = network
      .handle_sonarr_event(SonarrEvent::GetLogs(500))
      .await
      .unwrap()
    else {
      panic!("Expected LogResponse")
    };
    mock.assert_async().await;
    assert_eq!(app.lock().await.data.sonarr_data.logs.items, expected_logs);
    assert!(
      app
        .lock()
        .await
        .data
        .sonarr_data
        .logs
        .current_selection()
        .text
        .contains("INFO")
    );
    assert_eq!(logs, response);
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_diskspace_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([
        {
          "freeSpace": 1111,
          "totalSpace": 2222,
        },
        {
          "freeSpace": 3333,
          "totalSpace": 4444
        }
      ]))
      .build_for(SonarrEvent::GetDiskSpace)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);
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

    let SonarrSerdeable::DiskSpaces(disk_space) = network
      .handle_sonarr_event(SonarrEvent::GetDiskSpace)
      .await
      .unwrap()
    else {
      panic!("Expected DiskSpaces")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.sonarr_data.disk_space_vec,
      disk_space_vec
    );
    assert_eq!(disk_space, disk_space_vec);
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

    let (mock, app, _server) = MockServarrApi::get()
      .returns(queued_events_json)
      .build_for(SonarrEvent::GetQueuedEvents)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::QueueEvents(events) = network
      .handle_sonarr_event(SonarrEvent::GetQueuedEvents)
      .await
      .unwrap()
    else {
      panic!("Expected QueueEvents")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.sonarr_data.queued_events.items,
      vec![expected_event]
    );
    assert_eq!(events, response);
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
    let (mock, app, _server) = MockServarrApi::get()
      .returns(security_config_response)
      .build_for(SonarrEvent::GetSecurityConfig)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::SecurityConfig(security_config) = network
      .handle_sonarr_event(SonarrEvent::GetSecurityConfig)
      .await
      .unwrap()
    else {
      panic!("Expected SecurityConfig")
    };
    mock.assert_async().await;
    assert_eq!(security_config, response);
  }

  #[tokio::test]
  async fn test_handle_get_status_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({
        "version": "v1",
        "startTime": "2023-02-25T20:16:43Z"
      }))
      .build_for(SonarrEvent::GetStatus)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);
    let date_time = DateTime::from(DateTime::parse_from_rfc3339("2023-02-25T20:16:43Z").unwrap());

    let SonarrSerdeable::SystemStatus(status) = network
      .handle_sonarr_event(SonarrEvent::GetStatus)
      .await
      .unwrap()
    else {
      panic!("Expected SystemStatus")
    };
    mock.assert_async().await;
    assert_str_eq!(app.lock().await.data.sonarr_data.version, "v1");
    assert_eq!(app.lock().await.data.sonarr_data.start_time, date_time);
    assert_eq!(
      status,
      SystemStatus {
        version: "v1".to_owned(),
        start_time: date_time
      }
    );
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
    let (mock, app, _server) = MockServarrApi::get()
      .returns(tasks_json)
      .build_for(SonarrEvent::GetTasks)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::Tasks(tasks) = network
      .handle_sonarr_event(SonarrEvent::GetTasks)
      .await
      .unwrap()
    else {
      panic!("Expected Tasks")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.sonarr_data.tasks.items,
      expected_tasks
    );
    assert_eq!(tasks, response);
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
    let (mock, app, _server) = MockServarrApi::get()
      .returns(updates_json)
      .build_for(SonarrEvent::GetUpdates)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::Updates(updates) = network
      .handle_sonarr_event(SonarrEvent::GetUpdates)
      .await
      .unwrap()
    else {
      panic!("Expected Updates")
    };
    mock.assert_async().await;
    let actual_text = app.lock().await.data.sonarr_data.updates.get_text();
    let expected = expected_text.get_text();

    // Trim trailing whitespace from each line for comparison
    let actual_trimmed: Vec<&str> = actual_text.lines().map(|l| l.trim_end()).collect();
    let expected_trimmed: Vec<&str> = expected.lines().map(|l| l.trim_end()).collect();

    assert_eq!(
      actual_trimmed, expected_trimmed,
      "Updates text mismatch (after trimming trailing whitespace)"
    );
    assert_eq!(updates, response);
  }

  #[tokio::test]
  async fn test_handle_start_sonarr_task_event() {
    let response = json!({ "test": "test"});
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "ApplicationUpdateCheck"
      }))
      .returns(response.clone())
      .build_for(SonarrEvent::StartTask(
        SonarrTaskName::ApplicationUpdateCheck,
      ))
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .tasks
      .set_items(vec![SonarrTask {
        task_name: SonarrTaskName::default(),
        ..SonarrTask::default()
      }]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::Value(value) = network
      .handle_sonarr_event(SonarrEvent::StartTask(
        SonarrTaskName::ApplicationUpdateCheck,
      ))
      .await
      .unwrap()
    else {
      panic!("Expected Value")
    };
    mock.assert_async().await;
    assert_eq!(value, response);
  }
}
