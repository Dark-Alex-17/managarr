#[cfg(test)]
mod tests {
  use std::fmt::Debug;
  use std::string::ToString;
  use std::sync::Arc;

  use mockito::{Mock, Server, ServerGuard};
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;
  use serde::{Deserialize, Serialize};
  use tokio::sync::{mpsc, Mutex};
  use tokio_util::sync::CancellationToken;

  use crate::app::{App, AppConfig, RadarrConfig};
  use crate::models::HorizontallyScrollableText;
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::{Network, NetworkEvent, RequestMethod, RequestProps};

  #[tokio::test]
  async fn test_handle_network_event_radarr_event() {
    let mut server = Server::new_async().await;
    let radarr_server = server
      .mock("GET", "/api/v3/health")
      .with_status(200)
      .create_async()
      .await;
    let host = server.host_with_port().split(':').collect::<Vec<&str>>()[0].to_owned();
    let port = Some(
      server.host_with_port().split(':').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap(),
    );
    let mut app = App::default();
    app.is_loading = true;
    let radarr_config = RadarrConfig {
      host,
      api_token: String::new(),
      port,
    };
    app.config.radarr = radarr_config;
    let app_arc = Arc::new(Mutex::new(app));
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
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
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
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
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
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
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_request_with_response_body_ignore_error_code(
    #[values(RequestMethod::Get, RequestMethod::Post)] request_method: RequestMethod,
  ) {
    let (async_server, app_arc, server) = mock_api(request_method, 400, true).await;
    let mut network = Network::new(&app_arc, CancellationToken::new());
    let mut test_result = String::default();

    network
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
    let mut network = Network::new(&app_arc, cancellation_token);
    network.cancellation_token.cancel();

    network
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
    assert!(!network.cancellation_token.is_cancelled());
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
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
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
  }

  #[tokio::test]
  async fn test_handle_request_failure_to_send_request() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_request::<(), Test>(
        RequestProps {
          uri: String::default(),
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
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
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
  }

  #[tokio::test]
  async fn test_handle_request_non_success_code_empty_response_body() {
    let (async_server, app_arc, server) = mock_api(RequestMethod::Post, 404, false).await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
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
    let network = Network::new(&app_arc, CancellationToken::new());

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

  #[derive(Serialize, Deserialize, Debug, Default)]
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
