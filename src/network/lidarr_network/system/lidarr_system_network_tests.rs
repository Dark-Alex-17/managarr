#[cfg(test)]
mod tests {
  use crate::models::HorizontallyScrollableText;
  use crate::models::lidarr_models::{LidarrSerdeable, LidarrTask, LidarrTaskName, SystemStatus};
  use crate::models::servarr_models::{
    DiskSpace, HostConfig, LogResponse, QueueEvent, SecurityConfig, Update,
  };
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::updates;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use chrono::DateTime;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_get_diskspace_event() {
    let diskspace_json = json!([
      {
        "freeSpace": 1111,
        "totalSpace": 2222,
      },
      {
        "freeSpace": 3333,
        "totalSpace": 4444
      }
    ]);
    let response: Vec<DiskSpace> = serde_json::from_value(diskspace_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(diskspace_json)
      .build_for(LidarrEvent::GetDiskSpace)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network.handle_lidarr_event(LidarrEvent::GetDiskSpace).await;

    mock.assert_async().await;

    let LidarrSerdeable::DiskSpaces(disk_spaces) = result.unwrap() else {
      panic!("Expected DiskSpaces");
    };

    assert_eq!(disk_spaces, response);
    assert!(!app.lock().await.data.lidarr_data.disk_space_vec.is_empty());
  }

  #[tokio::test]
  async fn test_handle_get_host_config_event() {
    let host_config_json = json!({
      "bindAddress": "*",
      "port": 8686,
      "urlBase": "some.test.site/lidarr",
      "instanceName": "Lidarr",
      "applicationUrl": "https://some.test.site:8686/lidarr",
      "enableSsl": true,
      "sslPort": 6868,
      "sslCertPath": "/app/lidarr.pfx",
      "sslCertPassword": "test"
    });
    let response: HostConfig = serde_json::from_value(host_config_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(host_config_json)
      .build_for(LidarrEvent::GetHostConfig)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetHostConfig)
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::HostConfig(host_config) = result.unwrap() else {
      panic!("Expected HostConfig");
    };

    assert_eq!(host_config, response);
  }

  #[tokio::test]
  async fn test_handle_get_security_config_event() {
    let security_config_json = json!({
      "authenticationMethod": "forms",
      "authenticationRequired": "disabledForLocalAddresses",
      "username": "test",
      "password": "some password",
      "apiKey": "someApiKey12345",
      "certificateValidation": "disabledForLocalAddresses"
    });
    let response: SecurityConfig = serde_json::from_value(security_config_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(security_config_json)
      .build_for(LidarrEvent::GetSecurityConfig)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetSecurityConfig)
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::SecurityConfig(security_config) = result.unwrap() else {
      panic!("Expected SecurityConfig");
    };

    assert_eq!(security_config, response);
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_logs_event() {
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|LidarrError|Some.Big.Bad.Exception|test exception",
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
              "logger": "LidarrError",
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
      .build_for(LidarrEvent::GetLogs(500))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::LogResponse(logs) = network
      .handle_lidarr_event(LidarrEvent::GetLogs(500))
      .await
      .unwrap()
    else {
      panic!("Expected LogResponse")
    };
    mock.assert_async().await;
    assert_eq!(app.lock().await.data.lidarr_data.logs.items, expected_logs);
    assert!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .logs
        .current_selection()
        .text
        .contains("INFO")
    );
    assert_eq!(logs, response);
  }

  #[tokio::test]
  async fn test_handle_get_queued_lidarr_events_event() {
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
      .build_for(LidarrEvent::GetQueuedEvents)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::QueueEvents(events) = network
      .handle_lidarr_event(LidarrEvent::GetQueuedEvents)
      .await
      .unwrap()
    else {
      panic!("Expected QueueEvents")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.lidarr_data.queued_events.items,
      vec![expected_event]
    );
    assert_eq!(events, response);
  }

  #[tokio::test]
  async fn test_handle_get_status_event() {
    let status_json = json!({
      "version": "1.0.0",
      "startTime": "2023-01-01T00:00:00Z"
    });
    let response: SystemStatus = serde_json::from_value(status_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(status_json)
      .build_for(LidarrEvent::GetStatus)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network.handle_lidarr_event(LidarrEvent::GetStatus).await;

    mock.assert_async().await;

    let LidarrSerdeable::SystemStatus(status) = result.unwrap() else {
      panic!("Expected SystemStatus");
    };

    assert_eq!(status, response);
    assert_eq!(app.lock().await.data.lidarr_data.version, "1.0.0");
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_tasks_event() {
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
    let response: Vec<LidarrTask> = serde_json::from_value(tasks_json.clone()).unwrap();
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_tasks = vec![
      LidarrTask {
        name: "Application Update Check".to_owned(),
        task_name: LidarrTaskName::ApplicationUpdateCheck,
        interval: 360,
        last_execution: timestamp,
        next_execution: timestamp,
      },
      LidarrTask {
        name: "Backup".to_owned(),
        task_name: LidarrTaskName::Backup,
        interval: 10080,
        last_execution: timestamp,
        next_execution: timestamp,
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(tasks_json)
      .build_for(LidarrEvent::GetTasks)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::Tasks(tasks) = network
      .handle_lidarr_event(LidarrEvent::GetTasks)
      .await
      .unwrap()
    else {
      panic!("Expected Tasks")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.lidarr_data.tasks.items,
      expected_tasks
    );
    assert_eq!(tasks, response);
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_updates_event() {
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
    let expected_text = updates();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(updates_json)
      .build_for(LidarrEvent::GetUpdates)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::Updates(updates) = network
      .handle_lidarr_event(LidarrEvent::GetUpdates)
      .await
      .unwrap()
    else {
      panic!("Expected Updates")
    };
    mock.assert_async().await;
    let actual_text = app.lock().await.data.lidarr_data.updates.get_text();
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
  async fn test_handle_start_lidarr_task_event() {
    let response = json!({ "test": "test"});
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "ApplicationUpdateCheck"
      }))
      .returns(response.clone())
      .build_for(LidarrEvent::StartTask(
        LidarrTaskName::ApplicationUpdateCheck,
      ))
      .await;
    app
      .lock()
      .await
      .data
      .lidarr_data
      .tasks
      .set_items(vec![LidarrTask {
        task_name: LidarrTaskName::default(),
        ..LidarrTask::default()
      }]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::Value(value) = network
      .handle_lidarr_event(LidarrEvent::StartTask(
        LidarrTaskName::ApplicationUpdateCheck,
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
