use std::fmt::Debug;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::{debug, error, warn};
use regex::Regex;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sonarr_network::SonarrEvent;
use strum_macros::Display;
use tokio::select;
use tokio::sync::{Mutex, MutexGuard};
use tokio_util::sync::CancellationToken;

use crate::app::{App, ServarrConfig};
use crate::models::Serdeable;
use crate::network::radarr_network::RadarrEvent;
#[cfg(test)]
use mockall::automock;

pub mod radarr_network;
pub mod sonarr_network;
mod utils;

#[cfg(test)]
#[path = "network_tests.rs"]
mod network_tests;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait NetworkTrait {
  async fn handle_network_event(&mut self, network_event: NetworkEvent) -> Result<Serdeable>;
}

pub trait NetworkResource {
  fn resource(&self) -> &'static str;
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NetworkEvent {
  Radarr(RadarrEvent),
  Sonarr(SonarrEvent),
}

#[derive(Clone)]
pub struct Network<'a, 'b> {
  client: Client,
  pub cancellation_token: CancellationToken,
  pub app: &'a Arc<Mutex<App<'b>>>,
}

#[async_trait]
impl NetworkTrait for Network<'_, '_> {
  async fn handle_network_event(&mut self, network_event: NetworkEvent) -> Result<Serdeable> {
    let resp = match network_event {
      NetworkEvent::Radarr(radarr_event) => self
        .handle_radarr_event(radarr_event)
        .await
        .map(Serdeable::from),
      NetworkEvent::Sonarr(sonarr_event) => self
        .handle_sonarr_event(sonarr_event)
        .await
        .map(Serdeable::from),
    };

    let mut app = self.app.lock().await;
    app.is_loading = false;

    resp
  }
}

impl<'a, 'b> Network<'a, 'b> {
  pub fn new(
    app: &'a Arc<Mutex<App<'b>>>,
    cancellation_token: CancellationToken,
    client: Client,
  ) -> Self {
    Network {
      client,
      app,
      cancellation_token,
    }
  }

  pub(super) async fn reset_cancellation_token(&mut self) {
    self.cancellation_token = self.app.lock().await.reset_cancellation_token();
  }

  async fn handle_request<B, R>(
    &mut self,
    request_props: RequestProps<B>,
    mut app_update_fn: impl FnMut(R, MutexGuard<'_, App<'_>>),
  ) -> Result<R>
  where
    B: Serialize + Default + Debug,
    R: DeserializeOwned + Default + Clone,
  {
    let ignore_status_code = request_props.ignore_status_code;
    let method = request_props.method;
    let request_uri = request_props.uri.clone();
    select! {
    _ = self.cancellation_token.cancelled() => {
        warn!("Received Cancel request. Cancelling request to: {request_uri}");
        Ok(R::default())
      }
    resp = self.call_api(request_props).await.send() => {
         match resp {
          Ok(response) => {
            if response.status().is_success() || ignore_status_code {
              match method {
                RequestMethod::Get | RequestMethod::Post => {
                  match utils::parse_response::<R>(response).await {
                    Ok(value) => {
                      let app = self.app.lock().await;
                      app_update_fn(value.clone(), app);
                      Ok(value)
                    }
                    Err(e) => {
                      error!("Failed to parse response! {e:?}");
                      self
                        .app
                        .lock()
                        .await
                        .handle_error(anyhow!("Failed to parse response! {e:?}"));
                        Err(anyhow!("Failed to parse response! {e:?}"))
                    }
                  }
                }
                RequestMethod::Delete | RequestMethod::Put => Ok(R::default()),
              }
            } else {
              let status = response.status();
              let whitespace_regex = Regex::new(r"\s+").unwrap();
              let response_body = response.text().await.unwrap_or_default();
              let error_body = whitespace_regex
                .replace_all(&response_body.replace('\n', " "), " ")
                .to_string();

              error!("Request failed. Received {status} response code with body: {response_body}");
              self.app.lock().await.handle_error(anyhow!("Request failed. Received {status} response code with body: {error_body}"));
              Err(anyhow!("Request failed. Received {status} response code with body: {error_body}"))
            }
          }
          Err(e) => {
            error!("Failed to send request. {e:?}");
            self
              .app
              .lock()
              .await
              .handle_error(anyhow!("Failed to send request. {e} "));
              Err(anyhow!("Failed to send request. {e} "))
          }
        }
      }
    }
  }

  async fn call_api<T: Serialize + Default + Debug>(
    &self,
    request_props: RequestProps<T>,
  ) -> RequestBuilder {
    let RequestProps {
      uri,
      method,
      body,
      api_token,
      ..
    } = request_props;
    debug!("Creating RequestBuilder for resource: {uri:?}");
    debug!("Sending {method:?} request to {uri} with body {body:?}");

    match method {
      RequestMethod::Get => self.client.get(uri).header("X-Api-Key", api_token),
      RequestMethod::Post => self
        .client
        .post(uri)
        .json(&body.unwrap_or_default())
        .header("X-Api-Key", api_token),
      RequestMethod::Put => self
        .client
        .put(uri)
        .json(&body.unwrap_or_default())
        .header("X-Api-Key", api_token),
      RequestMethod::Delete => self
        .client
        .delete(uri)
        .json(&body.unwrap_or_default())
        .header("X-Api-Key", api_token),
    }
  }

  async fn request_props_from<T, N>(
    &self,
    network_event: N,
    method: RequestMethod,
    body: Option<T>,
    path: Option<String>,
    query_params: Option<String>,
  ) -> RequestProps<T>
  where
    T: Serialize + Debug,
    N: Into<NetworkEvent> + NetworkResource,
  {
    let app = self.app.lock().await;
    let resource = network_event.resource();
    let (
      ServarrConfig {
        host,
        port,
        uri,
        api_token,
        ssl_cert_path,
      },
      default_port,
    ) = match network_event.into() {
      NetworkEvent::Radarr(_) => (
        &app.config.radarr.as_ref().expect("Radarr config undefined"),
        7878,
      ),
      NetworkEvent::Sonarr(_) => (
        &app.config.sonarr.as_ref().expect("Sonarr config undefined"),
        8989,
      ),
    };
    let mut uri = if let Some(servarr_uri) = uri {
      format!("{servarr_uri}/api/v3{resource}")
    } else {
      let protocol = if ssl_cert_path.is_some() {
        "https"
      } else {
        "http"
      };
      let host = host.as_ref().unwrap();
      format!(
        "{protocol}://{host}:{}/api/v3{resource}",
        port.unwrap_or(default_port)
      )
    };

    if let Some(path) = path {
      uri = format!("{uri}{path}");
    }

    if let Some(params) = query_params {
      uri = format!("{uri}?{params}");
    }

    RequestProps {
      uri,
      method,
      body,
      api_token: api_token.to_owned(),
      ignore_status_code: false,
    }
  }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
pub enum RequestMethod {
  Get,
  Post,
  Put,
  Delete,
}

#[derive(Debug)]
pub struct RequestProps<T: Serialize + Debug> {
  pub uri: String,
  pub method: RequestMethod,
  pub body: Option<T>,
  pub api_token: String,
  pub ignore_status_code: bool,
}
