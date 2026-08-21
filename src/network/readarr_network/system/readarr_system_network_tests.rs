#[cfg(test)]
mod tests {
  use crate::models::HorizontallyScrollableText;
  use crate::models::readarr_models::ReadarrSerdeable;
  use crate::models::servarr_models::{
    AuthenticationMethod, AuthenticationRequired, CertificateValidation, DiskSpace, HostConfig,
    SecurityConfig, SystemStatus, Update,
  };
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use chrono::DateTime;
  use indoc::formatdoc;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

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
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetDiskSpace)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetDiskSpace)
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_is_empty!(app.lock().await.data.readarr_data.disk_space_vec);
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
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetStatus).await;

    mock.assert_async().await;
    assert_err!(result);
    assert_is_empty!(app.lock().await.data.readarr_data.version);
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
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetUpdates).await;

    mock.assert_async().await;
    assert_err!(result);
    assert_is_empty!(app.lock().await.data.readarr_data.updates.get_text());
  }
}
