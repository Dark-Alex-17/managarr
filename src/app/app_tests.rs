#[cfg(test)]
mod tests {
  use anyhow::anyhow;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
  use serde_json::Value;
  use serial_test::serial;
  use tokio::sync::mpsc;

  use crate::app::{App, AppConfig, Data, ServarrConfig, interpolate_env_vars};
  use crate::models::servarr_data::lidarr::lidarr_data::LidarrData;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, RadarrData};
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SonarrData};
  use crate::models::{HorizontallyScrollableText, TabRoute};
  use crate::network::NetworkEvent;
  use crate::network::radarr_network::RadarrEvent;
  use tokio_util::sync::CancellationToken;

  #[test]
  fn test_app_new() {
    let radarr_config_1 = ServarrConfig {
      name: Some("Radarr Test".to_owned()),
      ..ServarrConfig::default()
    };
    let radarr_config_2 = ServarrConfig {
      weight: Some(3),
      ..ServarrConfig::default()
    };
    let sonarr_config_1 = ServarrConfig {
      name: Some("Sonarr Test".to_owned()),
      weight: Some(1),
      ..ServarrConfig::default()
    };
    let sonarr_config_2 = ServarrConfig::default();
    let config = AppConfig {
      theme: None,
      radarr: Some(vec![radarr_config_1.clone(), radarr_config_2.clone()]),
      sonarr: Some(vec![sonarr_config_1.clone(), sonarr_config_2.clone()]),
      lidarr: None,
    };
    let expected_tab_routes = vec![
      TabRoute {
        title: "Sonarr Test".to_owned(),
        route: ActiveSonarrBlock::default().into(),
        contextual_help: None,
        config: Some(sonarr_config_1),
      },
      TabRoute {
        title: "Radarr 1".to_owned(),
        route: ActiveRadarrBlock::default().into(),
        contextual_help: None,
        config: Some(radarr_config_2),
      },
      TabRoute {
        title: "Radarr Test".to_owned(),
        route: ActiveRadarrBlock::default().into(),
        contextual_help: None,
        config: Some(radarr_config_1),
      },
      TabRoute {
        title: "Sonarr 1".to_owned(),
        route: ActiveSonarrBlock::default().into(),
        contextual_help: None,
        config: Some(sonarr_config_2),
      },
    ];

    let app = App::new(
      mpsc::channel::<NetworkEvent>(500).0,
      config,
      CancellationToken::new(),
    );

    assert_is_empty!(app.navigation_stack);
    assert_eq!(app.get_current_route(), ActiveSonarrBlock::default().into());
    assert_some!(app.network_tx);
    assert!(!app.cancellation_token.is_cancelled());
    assert!(app.is_first_render);
    assert_eq!(app.error, HorizontallyScrollableText::default());
    assert_eq!(app.server_tabs.index, 0);
    assert_eq!(app.server_tabs.tabs, expected_tab_routes);
    assert_eq!(app.tick_until_poll, 400);
    assert_eq!(app.ticks_until_scroll, 4);
    assert_eq!(app.tick_count, 0);
    assert_eq!(app.ui_scroll_tick_count, 0);
    assert!(!app.is_loading);
    assert!(!app.is_routing);
    assert!(!app.should_refresh);
    assert!(!app.ignore_special_keys_for_textbox_input);
    assert!(!app.cli_mode);
  }

  #[test]
  fn test_app_default() {
    let app = App::default();

    assert_is_empty!(app.navigation_stack);
    assert_none!(app.network_tx);
    assert!(!app.cancellation_token.is_cancelled());
    assert!(app.is_first_render);
    assert_eq!(app.error, HorizontallyScrollableText::default());
    assert_eq!(app.server_tabs.index, 0);
    assert_eq!(app.tick_until_poll, 400);
    assert_eq!(app.ticks_until_scroll, 4);
    assert_eq!(app.tick_count, 0);
    assert!(!app.is_loading);
    assert!(!app.is_routing);
    assert!(!app.should_refresh);
    assert!(!app.ignore_special_keys_for_textbox_input);
    assert!(!app.cli_mode);
  }

  #[test]
  fn test_navigation_stack_methods() {
    let mut app = App::test_default();
    let default_route = app.server_tabs.tabs.first().unwrap().route;

    assert_eq!(app.get_current_route(), default_route);

    app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());

    assert_eq!(app.get_current_route(), ActiveRadarrBlock::Downloads.into());
    assert!(app.is_routing);

    app.is_routing = false;
    app.pop_and_push_navigation_stack(ActiveRadarrBlock::Collections.into());

    assert_eq!(
      app.get_current_route(),
      ActiveRadarrBlock::Collections.into()
    );
    assert!(app.is_routing);

    app.is_routing = false;
    app.pop_navigation_stack();

    assert_eq!(app.get_current_route(), default_route);
    assert!(app.is_routing);

    app.is_routing = false;
    app.pop_navigation_stack();

    assert_eq!(app.get_current_route(), default_route);
    assert!(app.is_routing);
  }

  #[test]
  fn test_reset_cancellation_token() {
    let mut app = App {
      is_loading: true,
      should_refresh: false,
      ..App::test_default()
    };
    app.cancellation_token.cancel();

    assert!(app.cancellation_token.is_cancelled());

    let new_token = app.reset_cancellation_token();

    assert!(!app.cancellation_token.is_cancelled());
    assert!(!new_token.is_cancelled());
    assert!(!app.is_loading);
    assert!(app.should_refresh);
  }

  #[test]
  fn test_reset_tick_count() {
    let mut app = App {
      tick_count: 2,
      ..App::test_default()
    };

    app.reset_tick_count();

    assert_eq!(app.tick_count, 0);
  }

  #[test]
  fn test_reset() {
    let radarr_data = RadarrData {
      version: "test".into(),
      ..RadarrData::default()
    };
    let sonarr_data = SonarrData {
      version: "test".into(),
      ..SonarrData::default()
    };
    let data = Data {
      lidarr_data: LidarrData::default(),
      radarr_data,
      sonarr_data,
    };
    let mut app = App {
      tick_count: 2,
      error: "Test error".to_owned().into(),
      is_first_render: false,
      data,
      ..App::test_default()
    };

    app.reset();

    assert_eq!(app.tick_count, 0);
    assert_eq!(app.error, HorizontallyScrollableText::default());
    assert!(app.is_first_render);
    assert!(app.data.radarr_data.version.is_empty());
    assert!(app.data.sonarr_data.version.is_empty());
  }

  #[test]
  fn test_handle_error() {
    let mut app = App::test_default();
    let test_string = "Testing";

    app.handle_error(anyhow!(test_string));

    assert_eq!(app.error.text, test_string);

    app.handle_error(anyhow!("Testing a different error"));

    assert_eq!(app.error.text, test_string);
  }

  #[tokio::test]
  async fn test_dispatch_network_event() {
    let (sync_network_tx, mut sync_network_rx) = mpsc::channel::<NetworkEvent>(500);

    let mut app = App {
      tick_until_poll: 2,
      network_tx: Some(sync_network_tx),
      ..App::test_default()
    };

    assert_eq!(app.tick_count, 0);

    app
      .dispatch_network_event(RadarrEvent::GetStatus.into())
      .await;

    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetStatus.into()
    );
    assert_eq!(app.tick_count, 0);
  }

  #[test]
  fn test_on_ui_scroll_tick() {
    let mut app = App {
      ticks_until_scroll: 1,
      ..App::default()
    };

    assert_eq!(app.ui_scroll_tick_count, 0);
    assert_eq!(app.tick_count, 0);

    app.on_ui_scroll_tick();

    assert_eq!(app.ui_scroll_tick_count, 1);
    assert_eq!(app.tick_count, 0);

    app.on_ui_scroll_tick();

    assert_eq!(app.ui_scroll_tick_count, 0);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_on_tick_first_render() {
    let (sync_network_tx, mut sync_network_rx) = mpsc::channel::<NetworkEvent>(500);

    let mut app = App {
      tick_until_poll: 2,
      network_tx: Some(sync_network_tx),
      is_first_render: true,
      ..App::test_default()
    };

    assert_eq!(app.tick_count, 0);

    app.on_tick().await;

    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTags.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetRootFolders.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads(500).into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDiskSpace.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetStatus.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTags.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetMovies.into()
    );
    assert!(!app.is_routing);
    assert!(!app.should_refresh);
    assert_eq!(app.tick_count, 1);
  }

  #[tokio::test]
  async fn test_on_tick_routing() {
    let mut app = App {
      tick_until_poll: 2,
      tick_count: 2,
      is_routing: true,
      ..App::test_default()
    };

    app.on_tick().await;
    assert!(!app.is_routing);
  }

  #[tokio::test]
  async fn test_on_tick_should_refresh() {
    let mut app = App {
      tick_until_poll: 2,
      tick_count: 2,
      should_refresh: true,
      ..App::test_default()
    };

    app.on_tick().await;
    assert!(!app.should_refresh);
  }

  #[test]
  fn test_app_config_default() {
    let app_config = AppConfig::default();

    assert_none!(app_config.radarr);
    assert_none!(app_config.sonarr);
  }

  #[test]
  fn test_servarr_config_default() {
    let servarr_config = ServarrConfig::default();

    assert_none!(servarr_config.name);
    assert_some_eq_x!(&servarr_config.host, "localhost");
    assert_none!(servarr_config.port);
    assert_none!(servarr_config.uri);
    assert_none!(servarr_config.weight);
    assert_some_eq_x!(&servarr_config.api_token, "");
    assert_none!(servarr_config.api_token_file);
    assert_none!(servarr_config.ssl_cert_path);
    assert_none!(servarr_config.custom_headers);
  }

  #[test]
  fn serialize_header_map_basic() {
    let mut header_map = HeaderMap::new();
    header_map.insert(
      HeaderName::from_static("x-api-key"),
      HeaderValue::from_static("abc123"),
    );
    header_map.insert(
      HeaderName::from_static("header-1"),
      HeaderValue::from_static("test"),
    );

    let config = ServarrConfig {
      custom_headers: Some(header_map),
      ..ServarrConfig::default()
    };

    let v: Value = serde_json::to_value(&config).expect("serialize ok");
    let custom = v.get("custom_headers").unwrap();
    assert!(custom.is_object());
    let obj = custom.as_object().unwrap();

    assert_some_eq_x!(obj.get("x-api-key"), "abc123");
    assert_some_eq_x!(obj.get("header-1"), "test");

    assert_none!(obj.get("X-Api-Key"));
    assert_none!(obj.get("HEADER-1"));
  }

  #[test]
  fn serialize_header_map_none_is_null() {
    let config = ServarrConfig::default();
    let v: Value = serde_json::to_value(&config).expect("serialize ok");
    assert!(v.get("custom_headers").unwrap().is_null());
  }

  #[test]
  #[serial]
  fn test_deserialize_optional_env_var_is_present() {
    unsafe { std::env::set_var("TEST_VAR_DESERIALIZE_OPTION", "localhost") };
    let yaml_data = r#"
      host: ${TEST_VAR_DESERIALIZE_OPTION}
      api_token: "test123"
    "#;

    let config: ServarrConfig = serde_yaml::from_str(yaml_data).unwrap();

    assert_some_eq_x!(&config.host, "localhost");
    unsafe { std::env::remove_var("TEST_VAR_DESERIALIZE_OPTION") };
  }

  #[test]
  #[serial]
  fn test_deserialize_optional_env_var_does_not_overwrite_non_env_value() {
    unsafe { std::env::set_var("TEST_VAR_DESERIALIZE_OPTION_NO_OVERWRITE", "localhost") };
    let yaml_data = r#"
      host: www.example.com
      api_token: "test123"
    "#;

    let config: ServarrConfig = serde_yaml::from_str(yaml_data).unwrap();

    assert_some_eq_x!(&config.host, "www.example.com");
    unsafe { std::env::remove_var("TEST_VAR_DESERIALIZE_OPTION_NO_OVERWRITE") };
  }

  #[test]
  fn test_deserialize_optional_env_var_empty() {
    let yaml_data = r#"
      api_token: "test123"
    "#;

    let config: ServarrConfig = serde_yaml::from_str(yaml_data).unwrap();

    assert_none!(config.port);
  }

  #[test]
  #[serial]
  fn test_deserialize_optional_env_var_header_map_is_present() {
    unsafe { std::env::set_var("TEST_VAR_DESERIALIZE_HEADER_OPTION", "localhost") };
    let expected_custom_headers = {
      let mut headers = HeaderMap::new();
      headers.insert("X-Api-Host", "localhost".parse().unwrap());
      headers.insert("api-token", "test123".parse().unwrap());
      headers
    };
    let yaml_data = r#"
      custom_headers:
        X-Api-Host: ${TEST_VAR_DESERIALIZE_HEADER_OPTION}
        api-token: "test123"
    "#;

    let config: ServarrConfig = serde_yaml::from_str(yaml_data).unwrap();

    assert_some_eq_x!(&config.custom_headers, &expected_custom_headers);
    unsafe { std::env::remove_var("TEST_VAR_DESERIALIZE_HEADER_OPTION") };
  }

  #[test]
  #[serial]
  fn test_deserialize_optional_env_var_header_map_does_not_overwrite_non_env_value() {
    unsafe {
      std::env::set_var(
        "TEST_VAR_DESERIALIZE_HEADER_OPTION_NO_OVERWRITE",
        "localhost",
      )
    };
    let expected_custom_headers = {
      let mut headers = HeaderMap::new();
      headers.insert("X-Api-Host", "www.example.com".parse().unwrap());
      headers.insert("api-token", "test123".parse().unwrap());
      headers
    };
    let yaml_data = r#"
      custom_headers:
        X-Api-Host: www.example.com
        api-token: "test123"
    "#;

    let config: ServarrConfig = serde_yaml::from_str(yaml_data).unwrap();

    assert_some_eq_x!(&config.custom_headers, &expected_custom_headers);
    unsafe { std::env::remove_var("TEST_VAR_DESERIALIZE_HEADER_OPTION_NO_OVERWRITE") };
  }

  #[test]
  fn test_deserialize_optional_env_var_header_map_empty() {
    let yaml_data = r#"
      api_token: "test123"
    "#;

    let config: ServarrConfig = serde_yaml::from_str(yaml_data).unwrap();

    assert_none!(config.custom_headers);
  }

  #[test]
  #[serial]
  fn test_deserialize_optional_u16_env_var_is_present() {
    unsafe { std::env::set_var("TEST_VAR_DESERIALIZE_OPTION_U16", "1") };
    let yaml_data = r#"
      port: ${TEST_VAR_DESERIALIZE_OPTION_U16}
      api_token: "test123"
    "#;

    let config: ServarrConfig = serde_yaml::from_str(yaml_data).unwrap();

    assert_some_eq_x!(config.port, 1);
    unsafe { std::env::remove_var("TEST_VAR_DESERIALIZE_OPTION_U16") };
  }

  #[test]
  #[serial]
  fn test_deserialize_optional_u16_env_var_does_not_overwrite_non_env_value() {
    unsafe { std::env::set_var("TEST_VAR_DESERIALIZE_OPTION_U16_UNUSED", "1") };
    let yaml_data = r#"
      port: 1234
      api_token: "test123"
    "#;

    let config: ServarrConfig = serde_yaml::from_str(yaml_data).unwrap();

    assert_some_eq_x!(config.port, 1234);
    unsafe { std::env::remove_var("TEST_VAR_DESERIALIZE_OPTION_U16_UNUSED") };
  }

  #[test]
  fn test_deserialize_optional_u16_env_var_invalid_number() {
    let yaml_data = r#"
      port: "hi"
      api_token: "test123"
    "#;
    let result: Result<ServarrConfig, _> = serde_yaml::from_str(yaml_data);

    assert_err!(&result);
    let err = result.unwrap_err().to_string();
    assert_contains!(err, "invalid digit found in string");
  }

  #[test]
  fn test_deserialize_optional_u16_env_var_empty() {
    let yaml_data = r#"
      api_token: "test123"
    "#;

    let config: ServarrConfig = serde_yaml::from_str(yaml_data).unwrap();

    assert_none!(config.port);
  }

  #[test]
  #[serial]
  fn test_interpolate_env_vars() {
    unsafe { std::env::set_var("TEST_VAR_INTERPOLATION", "testing") };

    let var = interpolate_env_vars("${TEST_VAR_INTERPOLATION}");

    assert_str_eq!(var, "testing");
    unsafe { std::env::remove_var("TEST_VAR_INTERPOLATION") };
  }

  #[test]
  fn test_interpolate_env_vars_defaults_to_original_string_if_not_in_yaml_interpolation_format() {
    let var = interpolate_env_vars("TEST_VAR_INTERPOLATION_NON_YAML");

    assert_str_eq!(var, "TEST_VAR_INTERPOLATION_NON_YAML");
  }

  #[test]
  #[serial]
  fn test_interpolate_env_vars_scrubs_all_unnecessary_characters() {
    unsafe {
      std::env::set_var(
        "TEST_VAR_INTERPOLATION_UNNECESSARY_CHARACTERS",
        r#"""
	        `"'https://dontdo:this@testing.com/query?test=%20query#results'"` {([\|$!])}
      """#,
      )
    };

    let var = interpolate_env_vars("${TEST_VAR_INTERPOLATION_UNNECESSARY_CHARACTERS}");

    assert_str_eq!(
      var,
      "https://dontdo:this@testing.com/query?test=%20query#results"
    );
    unsafe { std::env::remove_var("TEST_VAR_INTERPOLATION_UNNECESSARY_CHARACTERS") };
  }

  #[test]
  fn test_interpolate_env_vars_scrubs_all_unnecessary_characters_from_non_environment_variable() {
    let var = interpolate_env_vars("https://dontdo:this@testing.com/query?test=%20query#results");

    assert_str_eq!(
      var,
      "https://dontdo:this@testing.com/query?test=%20query#results"
    );
  }

  #[test]
  fn test_servarr_config_redacted_debug() {
    let name = "Servarr".to_owned();
    let host = "localhost".to_owned();
    let port = 1234;
    let uri = "http://localhost:1234".to_owned();
    let weight = 100;
    let api_token = "thisisatest".to_owned();
    let api_token_file = "/root/.config/api_token".to_owned();
    let ssl_cert_path = "/some/path".to_owned();
    let mut custom_headers = HeaderMap::new();
    custom_headers.insert("X-Custom-Header", "value".parse().unwrap());
    let expected_str = format!(
      "ServarrConfig {{ name: Some(\"{name}\"), host: Some(\"{host}\"), port: Some({port}), uri: Some(\"{uri}\"), weight: Some({weight}), api_token: Some(\"***********\"), api_token_file: Some(\"{api_token_file}\"), ssl_cert_path: Some(\"{ssl_cert_path}\"), custom_headers: Some({{\"x-custom-header\": \"value\"}}) }}"
    );
    let servarr_config = ServarrConfig {
      name: Some(name),
      host: Some(host),
      port: Some(port),
      uri: Some(uri),
      weight: Some(weight),
      api_token: Some(api_token),
      api_token_file: Some(api_token_file),
      ssl_cert_path: Some(ssl_cert_path),
      custom_headers: Some(custom_headers),
    };

    assert_str_eq!(format!("{servarr_config:?}"), expected_str);
  }
}
