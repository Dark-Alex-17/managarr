#[cfg(test)]
mod tests {
  use crate::models::HorizontallyScrollableText;
  use crate::models::radarr_models::{RadarrSerdeable, RadarrTask, RadarrTaskName, SystemStatus};
  use crate::models::servarr_models::{
    DiskSpace, HostConfig, LogResponse, QueueEvent, SecurityConfig, Update,
  };
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::updates;
  use chrono::DateTime;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_get_radarr_diskspace_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([
        {
          "path": "/path1",
          "freeSpace": 1111,
          "totalSpace": 2222,
        },
        {
          "path": "/path2",
          "freeSpace": 3333,
          "totalSpace": 4444
        }
      ]))
      .build_for(RadarrEvent::GetDiskSpace)
      .await;
    let mut network = test_network(&app);
    let disk_space_vec = vec![
      DiskSpace {
        path: Some("/path1".to_owned()),
        free_space: 1111,
        total_space: 2222,
      },
      DiskSpace {
        path: Some("/path2".to_owned()),
        free_space: 3333,
        total_space: 4444,
      },
    ];

    let RadarrSerdeable::DiskSpaces(disk_space) = network
      .handle_radarr_event(RadarrEvent::GetDiskSpace)
      .await
      .unwrap()
    else {
      panic!("Expected DiskSpaces")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.disk_space_vec,
      disk_space_vec
    );
    assert_eq!(disk_space, disk_space_vec);
  }

  #[tokio::test]
  async fn test_handle_get_radarr_host_config_event() {
    let host_config_response = json!({
      "bindAddress": "*",
      "port": 7878,
      "urlBase": "some.test.site/radarr",
      "instanceName": "Radarr",
      "applicationUrl": "https://some.test.site:7878/radarr",
      "enableSsl": true,
      "sslPort": 9898,
      "sslCertPath": "/app/radarr.pfx",
      "sslCertPassword": "test"
    });
    let response: HostConfig = serde_json::from_value(host_config_response.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(host_config_response)
      .build_for(RadarrEvent::GetHostConfig)
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::HostConfig(host_config) = network
      .handle_radarr_event(RadarrEvent::GetHostConfig)
      .await
      .unwrap()
    else {
      panic!("Expected HostConfig")
    };
    mock.assert_async().await;
    assert_eq!(host_config, response);
  }

  #[tokio::test]
  async fn test_handle_get_radarr_logs_event() {
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|RadarrError|Some.Big.Bad.Exception|test exception",
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
              "logger": "RadarrError",
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
      .build_for(RadarrEvent::GetLogs(500))
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::LogResponse(logs) = network
      .handle_radarr_event(RadarrEvent::GetLogs(500))
      .await
      .unwrap()
    else {
      panic!("Expected LogResponse")
    };
    mock.assert_async().await;
    assert_eq!(app.lock().await.data.radarr_data.logs.items, expected_logs);
    assert!(
      app
        .lock()
        .await
        .data
        .radarr_data
        .logs
        .current_selection()
        .text
        .contains("INFO")
    );
    assert_eq!(logs, response);
  }

  #[tokio::test]
  async fn test_handle_get_queued_radarr_events_event() {
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
      .build_for(RadarrEvent::GetQueuedEvents)
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::QueueEvents(events) = network
      .handle_radarr_event(RadarrEvent::GetQueuedEvents)
      .await
      .unwrap()
    else {
      panic!("Expected QueueEvents")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.queued_events.items,
      vec![expected_event]
    );
    assert_eq!(events, response);
  }

  #[tokio::test]
  async fn test_handle_get_radarr_security_config_event() {
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
      .build_for(RadarrEvent::GetSecurityConfig)
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::SecurityConfig(security_config) = network
      .handle_radarr_event(RadarrEvent::GetSecurityConfig)
      .await
      .unwrap()
    else {
      panic!("Expected SecurityConfig")
    };
    mock.assert_async().await;
    assert_eq!(security_config, response);
  }

  #[tokio::test]
  async fn test_handle_get_radarr_status_event() {
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(json!({
        "version": "v1",
        "startTime": "2023-02-25T20:16:43Z"
      }))
      .build_for(RadarrEvent::GetStatus)
      .await;
    let mut network = test_network(&app);
    let date_time = DateTime::from(DateTime::parse_from_rfc3339("2023-02-25T20:16:43Z").unwrap());

    let RadarrSerdeable::SystemStatus(status) = network
      .handle_radarr_event(RadarrEvent::GetStatus)
      .await
      .unwrap()
    else {
      panic!("Expected SystemStatus")
    };
    async_server.assert_async().await;
    assert_str_eq!(app.lock().await.data.radarr_data.version, "v1");
    assert_eq!(app.lock().await.data.radarr_data.start_time, date_time);
    assert_eq!(
      status,
      SystemStatus {
        version: "v1".to_owned(),
        start_time: date_time
      }
    );
  }

  #[tokio::test]
  async fn test_handle_get_radarr_updates_event() {
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
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(updates_json)
      .build_for(RadarrEvent::GetUpdates)
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::Updates(updates) = network
      .handle_radarr_event(RadarrEvent::GetUpdates)
      .await
      .unwrap()
    else {
      panic!("Expected Updates")
    };
    async_server.assert_async().await;
    assert_str_eq!(
      app.lock().await.data.radarr_data.updates.get_text(),
      expected_text.get_text()
    );
    assert_eq!(updates, response);
  }

  #[tokio::test]
  async fn test_handle_get_radarr_tasks_event() {
    let tasks_json = json!([{
        "name": "Application Check Update",
        "taskName": "ApplicationCheckUpdate",
        "interval": 360,
        "lastExecution": "2023-05-20T21:29:16Z",
        "nextExecution": "2023-05-20T21:29:16Z",
        "lastDuration": "00:00:00.5111547",
    },
    {
        "name": "Backup",
        "taskName": "Backup",
        "interval": 10080,
        "lastExecution": "2023-05-20T21:29:16Z",
        "nextExecution": "2023-05-20T21:29:16Z",
        "lastDuration": "00:00:00.5111547",
    }]);
    let response: Vec<RadarrTask> = serde_json::from_value(tasks_json.clone()).unwrap();
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_tasks = vec![
      RadarrTask {
        name: "Application Check Update".to_owned(),
        task_name: RadarrTaskName::ApplicationCheckUpdate,
        interval: 360,
        last_execution: timestamp,
        next_execution: timestamp,
        last_duration: "00:00:00.5111547".to_owned(),
      },
      RadarrTask {
        name: "Backup".to_owned(),
        task_name: RadarrTaskName::Backup,
        interval: 10080,
        last_execution: timestamp,
        next_execution: timestamp,
        last_duration: "00:00:00.5111547".to_owned(),
      },
    ];
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(tasks_json)
      .build_for(RadarrEvent::GetTasks)
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::Tasks(tasks) = network
      .handle_radarr_event(RadarrEvent::GetTasks)
      .await
      .unwrap()
    else {
      panic!("Expected Tasks")
    };
    async_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.tasks.items,
      expected_tasks
    );
    assert_eq!(tasks, response);
  }

  #[tokio::test]
  async fn test_handle_start_radarr_task_event() {
    let response = json!({ "test": "test"});
    let (async_server, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "ApplicationCheckUpdate"
      }))
      .returns(response.clone())
      .build_for(RadarrEvent::StartTask(
        RadarrTaskName::ApplicationCheckUpdate,
      ))
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::Value(value) = network
      .handle_radarr_event(RadarrEvent::StartTask(
        RadarrTaskName::ApplicationCheckUpdate,
      ))
      .await
      .unwrap()
    else {
      panic!("Expected Value")
    };
    async_server.assert_async().await;
    assert_eq!(value, response);
  }
}
