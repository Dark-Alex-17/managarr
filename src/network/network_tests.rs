#[cfg(test)]
mod tests {
  use std::fmt::Debug;
  use std::string::ToString;
  use std::sync::Arc;

  use mockito::{Mock, Server, ServerGuard};
  use pretty_assertions::assert_str_eq;
  use reqwest::Client;
  use reqwest::header::HeaderMap;
  use rstest::rstest;
  use serde::{Deserialize, Serialize};
  use serde_json::json;
  use tokio::sync::{Mutex, mpsc};
  use tokio_util::sync::CancellationToken;

  use super::test_utils;
  use crate::app::{App, AppConfig, ServarrConfig};
  use crate::models::HorizontallyScrollableText;
  use crate::network::NetworkResource;
  use crate::network::network_tests::test_utils::test_network;
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::{Network, NetworkEvent, NetworkTrait, RequestMethod, RequestProps};

  #[tokio::test]
  async fn test_handle_network_event_radarr_event() {
    use test_utils::{MockServarrApi, test_network};

    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .build_for(RadarrEvent::HealthCheck)
      .await;

    app.lock().await.is_loading = true;
    let mut network = test_network(&app);

    let _ = network
      .handle_network_event(RadarrEvent::HealthCheck.into())
      .await;

    mock.assert_async().await;
    assert!(!app.lock().await.is_loading);
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
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let mut network = test_network(&app_arc);

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
          custom_headers: HeaderMap::new(),
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
    let mut network = test_network(&app_arc);

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
          custom_headers: HeaderMap::new(),
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
    let mut network = test_network(&app_arc);
    let mut test_result = String::new();

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: true,
          custom_headers: HeaderMap::new(),
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
          custom_headers: HeaderMap::new(),
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
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let mut network = test_network(&app_arc);

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: RequestMethod::Get,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
          custom_headers: HeaderMap::new(),
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    async_server.assert_async().await;
    assert!(
      app_arc
        .lock()
        .await
        .error
        .text
        .starts_with("Failed to parse response!")
    );
    assert!(resp.is_err());
    assert!(
      resp
        .unwrap_err()
        .to_string()
        .starts_with("Failed to parse response!")
    );
  }

  #[tokio::test]
  async fn test_handle_request_failure_to_send_request() {
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let mut network = test_network(&app_arc);

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: String::new(),
          method: RequestMethod::Get,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
          custom_headers: HeaderMap::new(),
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    assert!(
      app_arc
        .lock()
        .await
        .error
        .text
        .starts_with("Failed to send request.")
    );
    assert!(resp.is_err());
    assert!(
      resp
        .unwrap_err()
        .to_string()
        .starts_with("Failed to send request.")
    );
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
    let mut network = test_network(&app_arc);

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
          custom_headers: HeaderMap::new(),
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
    let mut network = test_network(&app_arc);

    let resp = network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: RequestMethod::Post,
          body: None,
          api_token: "test1234".to_owned(),
          ignore_status_code: false,
          custom_headers: HeaderMap::new(),
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
      .match_header("X-Custom-Header", "CustomValue")
      .with_status(200);
    let mut body = None::<Test>;
    let mut custom_headers = HeaderMap::new();
    custom_headers.insert("X-Custom-Header", "CustomValue".parse().unwrap());

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
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let network = test_network(&app_arc);

    network
      .call_api(RequestProps {
        uri: format!("{}/test", server.url()),
        method: request_method,
        body,
        api_token: "test1234".to_owned(),
        ignore_status_code: false,
        custom_headers,
      })
      .await
      .send()
      .await
      .unwrap();

    async_server.assert_async().await;
  }

  #[tokio::test]
  #[should_panic(expected = "Servarr config is undefined")]
  #[rstest]
  async fn test_request_props_from_requires_radarr_config_to_be_present_for_all_network_events(
    #[values(RadarrEvent::HealthCheck, SonarrEvent::HealthCheck)] network_event: impl Into<NetworkEvent>
    + NetworkResource,
  ) {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = test_network(&app_arc);

    network
      .request_props_from(network_event, RequestMethod::Get, None::<()>, None, None)
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
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let network = test_network(&app_arc);
    let resource = network_event.resource();
    {
      let mut app = app_arc.lock().await;
      app.server_tabs.tabs[0].config = Some(ServarrConfig::default());
      app.server_tabs.tabs[1].config = Some(ServarrConfig::default());
    }

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
    assert!(request_props.custom_headers.is_empty());
  }

  #[rstest]
  #[tokio::test]
  async fn test_request_props_from_custom_config(
    #[values(RadarrEvent::GetMovies, SonarrEvent::ListSeries)] network_event: impl Into<NetworkEvent>
    + NetworkResource,
  ) {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let resource = network_event.resource();
    let servarr_config = ServarrConfig {
      host: Some("192.168.0.123".to_owned()),
      port: Some(8080),
      api_token: Some(api_token.clone()),
      ssl_cert_path: Some("/test/cert.crt".to_owned()),
      ..ServarrConfig::default()
    };
    {
      let mut app = app_arc.lock().await;
      app.server_tabs.tabs[0].config = Some(servarr_config.clone());
      app.server_tabs.tabs[1].config = Some(servarr_config);
    }
    let network = test_network(&app_arc);

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
    assert!(request_props.custom_headers.is_empty());
  }

  #[rstest]
  #[tokio::test]
  async fn test_request_props_from_custom_config_custom_headers(
    #[values(RadarrEvent::GetMovies, SonarrEvent::ListSeries)] network_event: impl Into<NetworkEvent>
    + NetworkResource,
  ) {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let resource = network_event.resource();
    let mut header_map = HeaderMap::new();
    header_map.insert("X-Custom-Header", "CustomValue".parse().unwrap());
    let servarr_config = ServarrConfig {
      host: Some("192.168.0.123".to_owned()),
      port: Some(8080),
      api_token: Some(api_token.clone()),
      ssl_cert_path: Some("/test/cert.crt".to_owned()),
      custom_headers: Some(header_map.clone()),
      ..ServarrConfig::default()
    };
    {
      let mut app = app_arc.lock().await;
      app.server_tabs.tabs[0].config = Some(servarr_config.clone());
      app.server_tabs.tabs[1].config = Some(servarr_config);
    }
    let network = test_network(&app_arc);

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
    assert_eq!(request_props.custom_headers, header_map);
  }

  #[rstest]
  #[tokio::test]
  async fn test_request_props_from_custom_config_using_uri_instead_of_host_and_port(
    #[values(RadarrEvent::GetMovies, SonarrEvent::ListSeries)] network_event: impl Into<NetworkEvent>
    + NetworkResource,
  ) {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let resource = network_event.resource();
    let servarr_config = ServarrConfig {
      uri: Some("https://192.168.0.123:8080".to_owned()),
      api_token: Some(api_token.clone()),
      ..ServarrConfig::default()
    };
    {
      let mut app = app_arc.lock().await;
      app.server_tabs.tabs[0].config = Some(servarr_config.clone());
      app.server_tabs.tabs[1].config = Some(servarr_config);
    }
    let network = test_network(&app_arc);

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
    assert!(request_props.custom_headers.is_empty());
  }

  #[rstest]
  #[case(RadarrEvent::GetMovies, 7878)]
  #[case(SonarrEvent::ListSeries, 8989)]
  #[tokio::test]
  async fn test_request_props_from_default_config_with_path_and_query_params(
    #[case] network_event: impl Into<NetworkEvent> + NetworkResource,
    #[case] default_port: u16,
  ) {
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let network = test_network(&app_arc);
    let resource = network_event.resource();
    {
      let mut app = app_arc.lock().await;
      app.server_tabs.tabs[0].config = Some(ServarrConfig::default());
      app.server_tabs.tabs[1].config = Some(ServarrConfig::default());
    }

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
    assert!(request_props.custom_headers.is_empty());
  }

  #[rstest]
  #[tokio::test]
  async fn test_request_props_from_custom_config_with_path_and_query_params(
    #[values(RadarrEvent::GetMovies, SonarrEvent::ListSeries)] network_event: impl Into<NetworkEvent>
    + NetworkResource,
  ) {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let resource = network_event.resource();
    let servarr_config = ServarrConfig {
      host: Some("192.168.0.123".to_owned()),
      port: Some(8080),
      api_token: Some(api_token.clone()),
      ssl_cert_path: Some("/test/cert.crt".to_owned()),
      ..ServarrConfig::default()
    };
    {
      let mut app = app_arc.lock().await;
      app.server_tabs.tabs[0].config = Some(servarr_config.clone());
      app.server_tabs.tabs[1].config = Some(servarr_config);
    }
    let network = test_network(&app_arc);

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
    assert!(request_props.custom_headers.is_empty());
  }

  #[rstest]
  #[tokio::test]
  async fn test_request_props_from_custom_config_using_uri_instead_of_host_and_port_with_path_and_query_params(
    #[values(RadarrEvent::GetMovies, SonarrEvent::ListSeries)] network_event: impl Into<NetworkEvent>
    + NetworkResource,
  ) {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let resource = network_event.resource();
    let servarr_config = ServarrConfig {
      uri: Some("https://192.168.0.123:8080".to_owned()),
      api_token: Some(api_token.clone()),
      ..ServarrConfig::default()
    };
    {
      let mut app = app_arc.lock().await;
      app.server_tabs.tabs[0].config = Some(servarr_config.clone());
      app.server_tabs.tabs[1].config = Some(servarr_config);
    }
    let network = test_network(&app_arc);

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
    assert!(request_props.custom_headers.is_empty());
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
    let app_arc = Arc::new(Mutex::new(App::test_default()));

    (async_server, app_arc, server)
  }
}

#[cfg(test)]
pub(in crate::network) mod test_utils {
  use std::sync::Arc;

  use mockito::{Matcher, Mock, Server, ServerGuard};
  use reqwest::Client;
  use serde_json::Value;
  use tokio::sync::Mutex;
  use tokio_util::sync::CancellationToken;

  use crate::{
    app::{App, ServarrConfig},
    network::{Network, NetworkEvent, NetworkResource, RequestMethod},
  };

  pub fn test_client() -> Client {
    Client::new()
  }

  pub fn test_cancellation_token() -> CancellationToken {
    CancellationToken::new()
  }

  pub fn test_network<'a, 'b>(app: &'a Arc<Mutex<App<'b>>>) -> Network<'a, 'b> {
    Network::new(app, test_cancellation_token(), test_client())
  }

  pub struct MockServarrApi {
    method: RequestMethod,
    request_body: Option<Value>,
    response_body: Option<Value>,
    response_status: usize,
    path: Option<String>,
    query_params: Option<String>,
  }

  impl MockServarrApi {
    pub fn get() -> Self {
      Self::new(RequestMethod::Get)
    }

    pub fn post() -> Self {
      Self::new(RequestMethod::Post)
    }

    #[allow(dead_code)]
    pub fn put() -> Self {
      Self::new(RequestMethod::Put)
    }

    pub fn delete() -> Self {
      Self::new(RequestMethod::Delete)
    }

    pub fn new(method: RequestMethod) -> Self {
      Self {
        method,
        request_body: None,
        response_body: None,
        response_status: 200,
        path: None,
        query_params: None,
      }
    }

    pub fn with_request_body(mut self, body: Value) -> Self {
      self.request_body = Some(body);
      self
    }

    pub fn returns(mut self, body: Value) -> Self {
      self.response_body = Some(body);
      self
    }

    pub fn status(mut self, status: usize) -> Self {
      self.response_status = status;
      self
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
      self.path = Some(path.into());
      self
    }

    pub fn query(mut self, params: impl Into<String>) -> Self {
      self.query_params = Some(params.into());
      self
    }

    pub async fn build_for<E>(
      self,
      network_event: E,
    ) -> (Mock, Arc<Mutex<App<'static>>>, ServerGuard)
    where
      E: Into<NetworkEvent> + NetworkResource + Clone,
    {
      let resource = network_event.resource();
      let network_event_clone: NetworkEvent = network_event.clone().into();
      let api_version = match &network_event_clone {
        NetworkEvent::Lidarr(_) => "v1",
        _ => "v3",
      };
      let mut server = Server::new_async().await;
      let mut uri = format!("/api/{api_version}{resource}");

      if let Some(path) = &self.path {
        uri = format!("{uri}{path}");
      }

      if let Some(params) = &self.query_params {
        uri = format!("{uri}?{params}");
      }

      let mut mock_builder = server
        .mock(&self.method.to_string().to_uppercase(), uri.as_str())
        .match_header("X-Api-Key", "test1234")
        .with_status(self.response_status);

      if let Some(body) = &self.request_body {
        mock_builder = mock_builder.match_body(Matcher::Json(body.clone()));
      }

      if let Some(body) = &self.response_body {
        mock_builder = mock_builder.with_body(body.to_string());
      }

      let mock = mock_builder.create_async().await;

      let host = Some(server.host_with_port().split(':').collect::<Vec<&str>>()[0].to_owned());
      let port = Some(
        server.host_with_port().split(':').collect::<Vec<&str>>()[1]
          .parse()
          .unwrap(),
      );
      let mut app = App::test_default();
      let servarr_config = ServarrConfig {
        host,
        port,
        api_token: Some("test1234".to_owned()),
        ..ServarrConfig::default()
      };

      match network_event_clone {
        NetworkEvent::Radarr(_) => app.server_tabs.tabs[0].config = Some(servarr_config),
        NetworkEvent::Sonarr(_) => app.server_tabs.tabs[1].config = Some(servarr_config),
        NetworkEvent::Lidarr(_) => app.server_tabs.tabs[2].config = Some(servarr_config),
      }

      let app_arc = Arc::new(Mutex::new(app));

      (mock, app_arc, server)
    }
  }
}
