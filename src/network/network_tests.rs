#[cfg(test)]
mod tests {
  use std::fmt::Debug;
  use std::string::ToString;
  use std::sync::Arc;

  use mockito::{Mock, Server, ServerGuard};
  use pretty_assertions::assert_str_eq;
  use reqwest::Client;
  use rstest::rstest;
  use serde::{Deserialize, Serialize};
  use tokio::sync::{mpsc, Mutex};
  use tokio_util::sync::CancellationToken;

  use crate::app::{App, AppConfig, ServarrConfig};
  use crate::models::HorizontallyScrollableText;
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::NetworkResource;
  use crate::network::{Network, NetworkEvent, NetworkTrait, RequestMethod, RequestProps};

  #[tokio::test]
  async fn test_handle_network_event_radarr_event() {
    let mut server = Server::new_async().await;
    let radarr_server = server
      .mock("GET", "/api/v3/health")
      .with_status(200)
      .with_body("{}")
      .create_async()
      .await;
    let host = Some(server.host_with_port().split(':').collect::<Vec<&str>>()[0].to_owned());
    let port = Some(
      server.host_with_port().split(':').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap(),
    );
    let mut app = App::default();
    app.is_loading = true;
    let radarr_config = ServarrConfig {
      host,
      api_token: String::new(),
      port,
      ssl_cert_path: None,
      ..ServarrConfig::default()
    };
    app.config.radarr = Some(radarr_config);
    let app_arc = Arc::new(Mutex::new(app));
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let _ = network
      .handle_network_event(RadarrEvent::HealthCheck.into())
      .await;

    radarr_server.assert_async().await;
    assert!(!app_arc.lock().await.is_loading);
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_request_no_response_body(
    #[values(RequestMethod::Post, RequestMethod::Put, RequestMethod::Delete)]
    request_method: RequestMethod,
  ) {
    let mut server = Server::new_async().await;
    let async_server = server
      .mock(&request_method.to_string().to_uppercase(), "/test")
      .match_header("X-Api-Key", "test1234")
      .with_status(200)
      .create_async()
      .await;
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let _ = network
      .handle_request::<Test, ()>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: Some(Test {
            value: "Test".to_owned(),
          }),
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
        },
        |_, _| (),
      )
      .await;

    async_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_request_with_response_body(
    #[values(RequestMethod::Get, RequestMethod::Post)] request_method: RequestMethod,
  ) {
    let (async_server, app_arc, server) = mock_api(request_method, 200, true).await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    async_server.assert_async().await;
    assert_str_eq!(app_arc.lock().await.error.text, "Test");
    assert!(resp.is_ok());
    assert_eq!(
      resp.unwrap(),
      Test {
        value: "Test".to_owned()
      }
    );
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_request_with_response_body_ignore_error_code(
    #[values(RequestMethod::Get, RequestMethod::Post)] request_method: RequestMethod,
  ) {
    let (async_server, app_arc, server) = mock_api(request_method, 400, true).await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let mut test_result = String::new();

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: true,
        },
        |response, _app| test_result = response.value,
      )
      .await;

    async_server.assert_async().await;
    assert!(app_arc.lock().await.error.text.is_empty());
    assert!(resp.is_ok());
    assert_eq!(
      resp.unwrap(),
      Test {
        value: "Test".to_owned()
      }
    );
  }

  #[tokio::test]
  async fn test_handle_request_request_is_cancelled() {
    let (async_server, _, server) = mock_api(RequestMethod::Get, 200, true).await;
    let cancellation_token = CancellationToken::new();
    let (tx, _) = mpsc::channel::<NetworkEvent>(500);
    let app_arc = Arc::new(Mutex::new(App::new(
      tx,
      AppConfig::default(),
      cancellation_token.clone(),
    )));
    app_arc.lock().await.is_loading = true;
    let mut network = Network::new(&app_arc, cancellation_token, Client::new());
    network.cancellation_token.cancel();

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: RequestMethod::Get,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
        },
        |_, _| (),
      )
      .await;

    assert!(!async_server.matched_async().await);
    assert!(app_arc.lock().await.error.text.is_empty());
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap(), Test::default());
  }

  #[tokio::test]
  async fn test_reset_cancellation_token() {
    let cancellation_token = CancellationToken::new();
    let (tx, _) = mpsc::channel::<NetworkEvent>(500);
    let app_arc = Arc::new(Mutex::new(App::new(
      tx,
      AppConfig::default(),
      cancellation_token.clone(),
    )));
    app_arc.lock().await.should_refresh = false;
    app_arc.lock().await.is_loading = true;
    let mut network = Network::new(&app_arc, cancellation_token, Client::new());
    network.cancellation_token.cancel();

    network.reset_cancellation_token().await;

    assert!(!network.cancellation_token.is_cancelled());
    assert!(app_arc.lock().await.should_refresh);
    assert!(!app_arc.lock().await.is_loading);
  }

  #[tokio::test]
  async fn test_handle_request_get_invalid_body() {
    let mut server = Server::new_async().await;
    let async_server = server
      .mock("GET", "/test")
      .match_header("X-Api-Key", "test1234")
      .with_status(200)
      .with_body(r#"{ "invalid": "INVALID" }"#)
      .create_async()
      .await;
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: RequestMethod::Get,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .error
      .text
      .starts_with("Failed to parse response!"));
    assert!(resp.is_err());
    assert!(resp
      .unwrap_err()
      .to_string()
      .starts_with("Failed to parse response!"));
  }

  #[tokio::test]
  async fn test_handle_request_failure_to_send_request() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: String::new(),
          method: RequestMethod::Get,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    assert!(app_arc
      .lock()
      .await
      .error
      .text
      .starts_with("Failed to send request."));
    assert!(resp.is_err());
    assert!(resp
      .unwrap_err()
      .to_string()
      .starts_with("Failed to send request."));
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_request_non_success_code(
    #[values(
      RequestMethod::Get,
      RequestMethod::Post,
      RequestMethod::Put,
      RequestMethod::Delete
    )]
    request_method: RequestMethod,
  ) {
    let (async_server, app_arc, server) = mock_api(request_method, 404, true).await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    async_server.assert_async().await;
    assert_str_eq!(
      app_arc.lock().await.error.text,
      r#"Request failed. Received 404 Not Found response code with body: { "value": "Test" }"#
    );
    assert!(resp.is_err());
    assert_str_eq!(
      resp.unwrap_err().to_string(),
      r#"Request failed. Received 404 Not Found response code with body: { "value": "Test" }"#
    );
  }

  #[tokio::test]
  async fn test_handle_request_non_success_code_empty_response_body() {
    let (async_server, app_arc, server) = mock_api(RequestMethod::Post, 404, false).await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: RequestMethod::Post,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    async_server.assert_async().await;
    assert_str_eq!(
      app_arc.lock().await.error.text,
      r#"Request failed. Received 404 Not Found response code with body: "#
    );
    assert!(resp.is_err());
    assert_str_eq!(
      resp.unwrap_err().to_string(),
      r#"Request failed. Received 404 Not Found response code with body: "#
    );
  }

  #[rstest]
  #[tokio::test]
  async fn test_call_api(
    #[values(
      RequestMethod::Get,
      RequestMethod::Post,
      RequestMethod::Put,
      RequestMethod::Delete
    )]
    request_method: RequestMethod,
  ) {
    let mut server = Server::new_async().await;
    let mut async_server = server
      .mock(&request_method.to_string().to_uppercase(), "/test")
      .match_header("X-Api-Key", "test1234")
      .with_status(200);
    let mut body = None::<Test>;

    if request_method == RequestMethod::Post {
      async_server = async_server.with_body(
        r#"{ 
        "value": "Test" 
      }"#,
      );
      body = Some(Test {
        value: "Test".to_owned(),
      });
    }

    async_server = async_server.create_async().await;
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    network
      .call_api(RequestProps {
        uri: format!("{}/test", server.url()),
        method: request_method,
        body,
        api_token: "test1234".to_owned(),
        ignore_status_code: false,
      })
      .await
      .send()
      .await
      .unwrap();

    async_server.assert_async().await;
  }

  #[tokio::test]
  #[should_panic(expected = "Radarr config undefined")]
  async fn test_request_props_from_requires_radarr_config_to_be_present_for_radarr_events() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    network
      .request_props_from(
        RadarrEvent::GetMovies,
        RequestMethod::Get,
        None::<()>,
        None,
        None,
      )
      .await;
  }

  #[tokio::test]
  #[should_panic(expected = "Sonarr config undefined")]
  async fn test_request_props_from_requires_sonarr_config_to_be_present_for_sonarr_events() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    network
      .request_props_from(
        SonarrEvent::ListSeries,
        RequestMethod::Get,
        None::<()>,
        None,
        None,
      )
      .await;
  }

  #[rstest]
  #[case(RadarrEvent::GetMovies, 7878)]
  #[case(SonarrEvent::ListSeries, 8989)]
  #[tokio::test]
  async fn test_request_props_from_default_config(
    #[case] network_event: impl Into<NetworkEvent> + NetworkResource,
    #[case] default_port: u16,
  ) {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let resource = network_event.resource();
    app_arc.lock().await.config = AppConfig {
      radarr: Some(ServarrConfig::default()),
      sonarr: Some(ServarrConfig::default()),
    };

    let request_props = network
      .request_props_from(network_event, RequestMethod::Get, None::<()>, None, None)
      .await;

    assert_str_eq!(
      request_props.uri,
      format!("http://localhost:{default_port}/api/v3{resource}")
    );
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert!(request_props.api_token.is_empty());
  }

  #[rstest]
  #[tokio::test]
  async fn test_request_props_from_custom_config(
    #[values(RadarrEvent::GetMovies, SonarrEvent::ListSeries)] network_event: impl Into<NetworkEvent>
      + NetworkResource,
  ) {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::default()));
    let resource = network_event.resource();
    let servarr_config = ServarrConfig {
      host: Some("192.168.0.123".to_owned()),
      port: Some(8080),
      api_token: api_token.clone(),
      ssl_cert_path: Some("/test/cert.crt".to_owned()),
      ..ServarrConfig::default()
    };
    {
      let mut app = app_arc.lock().await;
      app.config.radarr = Some(servarr_config.clone());
      app.config.sonarr = Some(servarr_config);
    }
    let network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let request_props = network
      .request_props_from(network_event, RequestMethod::Get, None::<()>, None, None)
      .await;

    assert_str_eq!(
      request_props.uri,
      format!("https://192.168.0.123:8080/api/v3{resource}")
    );
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert_str_eq!(request_props.api_token, api_token);
  }

  #[rstest]
  #[tokio::test]
  async fn test_request_props_from_custom_config_using_uri_instead_of_host_and_port(
    #[values(RadarrEvent::GetMovies, SonarrEvent::ListSeries)] network_event: impl Into<NetworkEvent>
      + NetworkResource,
  ) {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::default()));
    let resource = network_event.resource();
    let servarr_config = ServarrConfig {
      uri: Some("https://192.168.0.123:8080".to_owned()),
      api_token: api_token.clone(),
      ..ServarrConfig::default()
    };
    {
      let mut app = app_arc.lock().await;
      app.config.radarr = Some(servarr_config.clone());
      app.config.sonarr = Some(servarr_config);
    }
    let network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let request_props = network
      .request_props_from(network_event, RequestMethod::Get, None::<()>, None, None)
      .await;

    assert_str_eq!(
      request_props.uri,
      format!("https://192.168.0.123:8080/api/v3{resource}")
    );
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert_str_eq!(request_props.api_token, api_token);
  }

  #[rstest]
  #[case(RadarrEvent::GetMovies, 7878)]
  #[case(SonarrEvent::ListSeries, 8989)]
  #[tokio::test]
  async fn test_request_props_from_default_config_with_path_and_query_params(
    #[case] network_event: impl Into<NetworkEvent> + NetworkResource,
    #[case] default_port: u16,
  ) {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let resource = network_event.resource();
    app_arc.lock().await.config = AppConfig {
      radarr: Some(ServarrConfig::default()),
      sonarr: Some(ServarrConfig::default()),
    };

    let request_props = network
      .request_props_from(
        network_event,
        RequestMethod::Get,
        None::<()>,
        Some("/test".to_owned()),
        Some("id=1".to_owned()),
      )
      .await;

    assert_str_eq!(
      request_props.uri,
      format!("http://localhost:{default_port}/api/v3{resource}/test?id=1")
    );
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert!(request_props.api_token.is_empty());
  }

  #[rstest]
  #[tokio::test]
  async fn test_request_props_from_custom_config_with_path_and_query_params(
    #[values(RadarrEvent::GetMovies, SonarrEvent::ListSeries)] network_event: impl Into<NetworkEvent>
      + NetworkResource,
  ) {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::default()));
    let resource = network_event.resource();
    let servarr_config = ServarrConfig {
      host: Some("192.168.0.123".to_owned()),
      port: Some(8080),
      api_token: api_token.clone(),
      ssl_cert_path: Some("/test/cert.crt".to_owned()),
      ..ServarrConfig::default()
    };
    {
      let mut app = app_arc.lock().await;
      app.config.radarr = Some(servarr_config.clone());
      app.config.sonarr = Some(servarr_config);
    }
    let network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let request_props = network
      .request_props_from(
        network_event,
        RequestMethod::Get,
        None::<()>,
        Some("/test".to_owned()),
        Some("id=1".to_owned()),
      )
      .await;

    assert_str_eq!(
      request_props.uri,
      format!("https://192.168.0.123:8080/api/v3{resource}/test?id=1")
    );
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert_str_eq!(request_props.api_token, api_token);
  }

  #[rstest]
  #[tokio::test]
  async fn test_request_props_from_custom_config_using_uri_instead_of_host_and_port_with_path_and_query_params(
    #[values(RadarrEvent::GetMovies, SonarrEvent::ListSeries)] network_event: impl Into<NetworkEvent>
      + NetworkResource,
  ) {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::default()));
    let resource = network_event.resource();
    let servarr_config = ServarrConfig {
      uri: Some("https://192.168.0.123:8080".to_owned()),
      api_token: api_token.clone(),
      ..ServarrConfig::default()
    };
    {
      let mut app = app_arc.lock().await;
      app.config.radarr = Some(servarr_config.clone());
      app.config.sonarr = Some(servarr_config);
    }
    let network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let request_props = network
      .request_props_from(
        network_event,
        RequestMethod::Get,
        None::<()>,
        Some("/test".to_owned()),
        Some("id=1".to_owned()),
      )
      .await;

    assert_str_eq!(
      request_props.uri,
      format!("https://192.168.0.123:8080/api/v3{resource}/test?id=1")
    );
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert_str_eq!(request_props.api_token, api_token);
  }

  #[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
  struct Test {
    pub value: String,
  }

  async fn mock_api<'a>(
    method: RequestMethod,
    response_status: usize,
    has_response_body: bool,
  ) -> (Mock, Arc<Mutex<App<'a>>>, ServerGuard) {
    let mut server = Server::new_async().await;
    let mut async_server = server
      .mock(&method.to_string().to_uppercase(), "/test")
      .match_header("X-Api-Key", "test1234")
      .with_status(response_status);

    if has_response_body {
      async_server = async_server.with_body(
        r#"{ 
        "value": "Test" 
      }"#,
      );
    }

    async_server = async_server.create_async().await;
    let app_arc = Arc::new(Mutex::new(App::default()));

    (async_server, app_arc, server)
  }
}

#[cfg(test)]
pub(in crate::network) mod test_utils {
  use std::sync::Arc;

  use mockito::{Matcher, Mock, Server, ServerGuard};
  use serde_json::Value;
  use tokio::sync::Mutex;

  use crate::{
    app::{App, ServarrConfig},
    network::{NetworkEvent, NetworkResource, RequestMethod},
  };

  pub async fn mock_servarr_api<'a>(
    method: RequestMethod,
    request_body: Option<Value>,
    response_body: Option<Value>,
    response_status: Option<usize>,
    network_event: impl Into<NetworkEvent> + NetworkResource,
    path: Option<&str>,
    query_params: Option<&str>,
  ) -> (Mock, Arc<Mutex<App<'a>>>, ServerGuard) {
    let status = response_status.unwrap_or(200);
    let resource = network_event.resource();
    let mut server = Server::new_async().await;
    let mut uri = format!("/api/v3{resource}");

    if let Some(path) = path {
      uri = format!("{uri}{path}");
    }

    if let Some(params) = query_params {
      uri = format!("{uri}?{params}");
    }

    let mut async_server = server
      .mock(&method.to_string().to_uppercase(), uri.as_str())
      .match_header("X-Api-Key", "test1234")
      .with_status(status);

    if let Some(body) = request_body {
      async_server = async_server.match_body(Matcher::Json(body));
    }

    if let Some(body) = response_body {
      async_server = async_server.with_body(body.to_string());
    }

    async_server = async_server.create_async().await;

    let host = Some(server.host_with_port().split(':').collect::<Vec<&str>>()[0].to_owned());
    let port = Some(
      server.host_with_port().split(':').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap(),
    );
    let mut app = App::default();
    let servarr_config = ServarrConfig {
      host,
      port,
      api_token: "test1234".to_owned(),
      ..ServarrConfig::default()
    };

    match network_event.into() {
      NetworkEvent::Radarr(_) => app.config.radarr = Some(servarr_config),
      NetworkEvent::Sonarr(_) => app.config.sonarr = Some(servarr_config),
    }

    let app_arc = Arc::new(Mutex::new(app));

    (async_server, app_arc, server)
  }
}
