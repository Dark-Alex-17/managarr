#[cfg(test)]
mod tests {
  use crate::models::readarr_models::{ReadarrSerdeable, ReadarrTask, ReadarrTaskName};
  use crate::models::servarr_models::{
    AuthenticationMethod, AuthenticationRequired, CertificateValidation, DiskSpace, HostConfig,
    LogResponse, QueueEvent, SecurityConfig, SystemStatus, Update,
  };
  use crate::models::{HorizontallyScrollableText, ScrollableText};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use chrono::DateTime;
  use indoc::formatdoc;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_get_queued_readarr_events_event() {
    let queued_events_json = json!([{
        "name": "CheckHealth",
        "commandName": "Check Health",
        "status": "completed",
        "queued": "2023-05-20T21:29:16Z",
        "started": "2023-05-20T21:29:16Z",
        "ended": "2023-05-20T21:29:16Z",
        "duration": "00:00:00.3524068",
        "trigger": "manual",
    }]);
    let response: Vec<QueueEvent> = serde_json::from_value(queued_events_json.clone()).unwrap();
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_event = QueueEvent {
      name: "CheckHealth".to_owned(),
      command_name: "Check Health".to_owned(),
      status: "completed".to_owned(),
      queued: timestamp,
      started: Some(timestamp),
      ended: Some(timestamp),
      duration: Some("00:00:00.3524068".to_owned()),
      trigger: "manual".to_owned(),
    };
    let (mock, app, _server) = MockServarrApi::get()
      .returns(queued_events_json)
      .build_for(ReadarrEvent::GetQueuedEvents)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQueuedEvents)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::QueueEvents(events) = result.unwrap() else {
      panic!("Expected QueueEvents")
    };

    assert_eq!(events, response);
    assert_eq!(
      app.lock().await.data.readarr_data.queued_events.items,
      vec![expected_event]
    );
  }

  #[tokio::test]
  async fn test_handle_get_queued_readarr_events_event_empty_response() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .build_for(ReadarrEvent::GetQueuedEvents)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .queued_events
        .set_items(vec![QueueEvent {
          name: "StaleEvent".to_owned(),
          ..QueueEvent::default()
        }]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQueuedEvents)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::QueueEvents(events) = result.unwrap() else {
      panic!("Expected QueueEvents")
    };

    assert_is_empty!(events);
    assert_is_empty!(app.lock().await.data.readarr_data.queued_events);
  }

  #[tokio::test]
  async fn test_handle_get_queued_readarr_events_event_failure() {
    let seeded_event = QueueEvent {
      name: "StaleEvent".to_owned(),
      ..QueueEvent::default()
    };
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetQueuedEvents)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .queued_events
        .set_items(vec![seeded_event.clone()]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQueuedEvents)
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.queued_events.items,
      vec![seeded_event]
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_diskspace_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([
        {
          "path": "/nfs/books/",
          "freeSpace": 11039526748160i64,
          "totalSpace": 117810875334656i64
        },
        {
          "path": "/config",
          "label": "",
          "freeSpace": 783978655744i64,
          "totalSpace": 1004298338304i64
        }
      ]))
      .build_for(ReadarrEvent::GetDiskSpace)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);
    let disk_space_vec = vec![
      DiskSpace {
        path: Some("/nfs/books/".to_owned()),
        free_space: 11039526748160,
        total_space: 117810875334656,
      },
      DiskSpace {
        path: Some("/config".to_owned()),
        free_space: 783978655744,
        total_space: 1004298338304,
      },
    ];

    let result = network
      .handle_readarr_event(ReadarrEvent::GetDiskSpace)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::DiskSpaces(disk_spaces) = result.unwrap() else {
      panic!("Expected DiskSpaces")
    };

    assert_eq!(disk_spaces, disk_space_vec);
    assert_eq!(
      app.lock().await.data.readarr_data.disk_space_vec,
      disk_space_vec
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_diskspace_event_failure() {
    let seeded_disk_space = vec![DiskSpace {
      path: Some("/stale".to_owned()),
      free_space: 1,
      total_space: 2,
    }];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetDiskSpace)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.disk_space_vec = seeded_disk_space.clone();
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetDiskSpace)
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.disk_space_vec,
      seeded_disk_space
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_host_config_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({
        "bindAddress": "*",
        "port": 8787,
        "urlBase": "",
        "instanceName": "Readarr",
        "applicationUrl": "",
        "enableSsl": false,
        "sslPort": 6868,
        "sslCertPath": "",
        "sslCertPassword": ""
      }))
      .build_for(ReadarrEvent::GetHostConfig)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetHostConfig)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::HostConfig(host_config) = result.unwrap() else {
      panic!("Expected HostConfig")
    };

    assert_eq!(
      host_config,
      HostConfig {
        bind_address: "*".into(),
        port: 8787,
        url_base: Some(HorizontallyScrollableText::default()),
        instance_name: Some("Readarr".into()),
        application_url: Some(HorizontallyScrollableText::default()),
        enable_ssl: false,
        ssl_port: 6868,
        ssl_cert_path: Some(String::new()),
        ssl_cert_password: Some(String::new()),
      }
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_host_config_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetHostConfig)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetHostConfig)
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_logs_event() {
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
          "logger": "ReadarrError",
          "exception": "test exception",
          "exceptionType": "Some.Big.Bad.Exception",
          "id": 2
        }
      ]
    });
    let response: LogResponse = serde_json::from_value(logs_response_json.clone()).unwrap();
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|ReadarrError|Some.Big.Bad.Exception|test exception",
      ),
      HorizontallyScrollableText::from("2023-05-20 21:29:16 UTC|INFO|TestLogger|test message"),
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(logs_response_json)
      .query("pageSize=500&sortDirection=descending&sortKey=time")
      .build_for(ReadarrEvent::GetLogs(500))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetLogs(500))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::LogResponse(logs) = result.unwrap() else {
      panic!("Expected LogResponse")
    };

    assert_eq!(logs, response);
    assert_eq!(app.lock().await.data.readarr_data.logs.items, expected_logs);
    assert_str_eq!(
      app
        .lock()
        .await
        .data
        .readarr_data
        .logs
        .current_selection()
        .text,
      "2023-05-20 21:29:16 UTC|INFO|TestLogger|test message"
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_logs_event_failure() {
    let seeded_logs = vec![HorizontallyScrollableText::from("stale log entry")];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .query("pageSize=500&sortDirection=descending&sortKey=time")
      .build_for(ReadarrEvent::GetLogs(500))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.logs.set_items(seeded_logs.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetLogs(500))
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(app.lock().await.data.readarr_data.logs.items, seeded_logs);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_security_config_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({
        "authenticationMethod": "forms",
        "authenticationRequired": "enabled",
        "username": "test",
        "password": "test",
        "apiKey": "test1234",
        "certificateValidation": "enabled"
      }))
      .build_for(ReadarrEvent::GetSecurityConfig)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetSecurityConfig)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::SecurityConfig(security_config) = result.unwrap() else {
      panic!("Expected SecurityConfig")
    };

    assert_eq!(
      security_config,
      SecurityConfig {
        authentication_method: AuthenticationMethod::Forms,
        authentication_required: Some(AuthenticationRequired::Enabled),
        username: Some("test".to_owned()),
        password: Some("test".to_owned()),
        api_key: "test1234".to_owned(),
        certificate_validation: CertificateValidation::Enabled,
      }
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_security_config_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetSecurityConfig)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetSecurityConfig)
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_status_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({
        "version": "0.4.20.129",
        "startTime": "2026-08-17T03:55:02Z"
      }))
      .build_for(ReadarrEvent::GetStatus)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);
    let date_time = DateTime::from(DateTime::parse_from_rfc3339("2026-08-17T03:55:02Z").unwrap());

    let result = network.handle_readarr_event(ReadarrEvent::GetStatus).await;

    mock.assert_async().await;

    let ReadarrSerdeable::SystemStatus(status) = result.unwrap() else {
      panic!("Expected SystemStatus")
    };

    assert_eq!(
      status,
      SystemStatus {
        version: "0.4.20.129".to_owned(),
        start_time: date_time
      }
    );
    assert_str_eq!(app.lock().await.data.readarr_data.version, "0.4.20.129");
    assert_eq!(app.lock().await.data.readarr_data.start_time, date_time);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_status_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetStatus)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.version = "stale version".to_owned();
    }
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetStatus).await;

    mock.assert_async().await;
    assert_err!(result);
    assert_str_eq!(app.lock().await.data.readarr_data.version, "stale version");
  }

  #[tokio::test]
  async fn test_handle_get_readarr_tasks_event() {
    let tasks_json = json!([{
      "name": "Application Update Check",
      "taskName": "ApplicationUpdateCheck",
      "interval": 360,
      "lastExecution": "2023-05-20T21:29:16Z",
      "lastDuration": "00:00:00.2293467",
      "nextExecution": "2023-05-20T21:29:16Z",
    },
    {
      "name": "Refresh Author",
      "taskName": "RefreshAuthor",
      "interval": 10080,
      "lastExecution": "2023-05-20T21:29:16Z",
      "lastDuration": "00:00:03.7211204",
      "nextExecution": "2023-05-20T21:29:16Z",
    }]);
    let response: Vec<ReadarrTask> = serde_json::from_value(tasks_json.clone()).unwrap();
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_tasks = vec![
      ReadarrTask {
        name: "Application Update Check".to_owned(),
        task_name: ReadarrTaskName::ApplicationUpdateCheck,
        interval: 360,
        last_execution: timestamp,
        last_duration: "00:00:00.2293467".to_owned(),
        next_execution: timestamp,
      },
      ReadarrTask {
        name: "Refresh Author".to_owned(),
        task_name: ReadarrTaskName::RefreshAuthor,
        interval: 10080,
        last_execution: timestamp,
        last_duration: "00:00:03.7211204".to_owned(),
        next_execution: timestamp,
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(tasks_json)
      .build_for(ReadarrEvent::GetTasks)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetTasks).await;

    mock.assert_async().await;

    let ReadarrSerdeable::Tasks(tasks) = result.unwrap() else {
      panic!("Expected Tasks")
    };

    assert_eq!(tasks, response);
    assert_eq!(
      app.lock().await.data.readarr_data.tasks.items,
      expected_tasks
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_tasks_event_failure() {
    let seeded_task = ReadarrTask {
      name: "Stale Task".to_owned(),
      ..ReadarrTask::default()
    };
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetTasks)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .tasks
        .set_items(vec![seeded_task.clone()]);
    }
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetTasks).await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.tasks.items,
      vec![seeded_task]
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_updates_event() {
    let updates_json = json!([{
      "version": "4.3.2.1",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": true,
      "installedOn": "2023-04-15T02:02:53Z",
      "latest": true,
      "changes": { "new": ["Cool new thing"], "fixed": ["Some bugs killed"] },
    },
    {
      "version": "3.2.1.0",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": false,
      "installedOn": "2023-04-15T02:02:53Z",
      "latest": false,
      "changes": { "new": ["Cool new thing (old)", "Other cool new thing (old)"] },
    },
    {
      "version": "2.1.0",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": false,
      "latest": false,
      "changes": { "fixed": ["Killed bug 1", "Fixed bug 2"] },
    },
    {
      "version": "1.0.0",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": false,
      "latest": false,
    }]);
    let response: Vec<Update> = serde_json::from_value(updates_json.clone()).unwrap();
    let line_break = "-".repeat(200);
    let expected_text = formatdoc!(
      "The latest version of Readarr is already installed

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
        * Fixed bug 2


      1.0.0 - 2023-04-15 02:02:53 UTC
      {line_break}"
    );
    let (mock, app, _server) = MockServarrApi::get()
      .returns(updates_json)
      .build_for(ReadarrEvent::GetUpdates)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetUpdates).await;

    mock.assert_async().await;

    let ReadarrSerdeable::Updates(updates) = result.unwrap() else {
      panic!("Expected Updates")
    };

    assert_eq!(updates, response);

    assert_str_eq!(
      app.lock().await.data.readarr_data.updates.get_text(),
      expected_text
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_updates_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetUpdates)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.updates = ScrollableText::with_string("stale updates".to_owned());
    }
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetUpdates).await;

    mock.assert_async().await;
    assert_err!(result);
    assert_str_eq!(
      app.lock().await.data.readarr_data.updates.get_text(),
      "stale updates"
    );
  }

  #[tokio::test]
  async fn test_handle_start_readarr_task_event() {
    let response = json!({ "test": "test" });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "CheckHealth"
      }))
      .returns(response.clone())
      .build_for(ReadarrEvent::StartTask(ReadarrTaskName::CheckHealth))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::StartTask(ReadarrTaskName::CheckHealth))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Value(value) = result.unwrap() else {
      panic!("Expected Value")
    };

    assert_eq!(value, response);
  }

  #[tokio::test]
  async fn test_handle_start_readarr_task_event_failure() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "CheckHealth"
      }))
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::StartTask(ReadarrTaskName::CheckHealth))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::StartTask(ReadarrTaskName::CheckHealth))
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }
}
