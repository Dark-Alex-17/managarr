use std::fmt::Debug;
use std::sync::Arc;

use anyhow::anyhow;
use log::{debug, error, warn};
use regex::Regex;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use strum_macros::Display;
use tokio::select;
use tokio::sync::{Mutex, MutexGuard};
use tokio_util::sync::CancellationToken;

use crate::app::App;
use crate::network::radarr_network::RadarrEvent;

pub mod radarr_network;
mod utils;

#[cfg(test)]
#[path = "network_tests.rs"]
mod network_tests;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum NetworkEvent {
  Radarr(RadarrEvent),
}

pub struct Network<'a, 'b> {
  client: Client,
  cancellation_token: CancellationToken,
  pub app: &'a Arc<Mutex<App<'b>>>,
}

impl<'a, 'b> Network<'a, 'b> {
  pub fn new(app: &'a Arc<Mutex<App<'b>>>, cancellation_token: CancellationToken) -> Self {
    Network {
      client: Client::new(),
      app,
      cancellation_token,
    }
  }

  pub async fn handle_network_event(&mut self, network_event: NetworkEvent) {
    match network_event {
      NetworkEvent::Radarr(radarr_event) => self.handle_radarr_event(radarr_event).await,
    }

    let mut app = self.app.lock().await;
    app.is_loading = false;
  }

  pub async fn handle_request<B, R>(
    &mut self,
    request_props: RequestProps<B>,
    mut app_update_fn: impl FnMut(R, MutexGuard<'_, App<'_>>),
  ) where
    B: Serialize + Default + Debug,
    R: DeserializeOwned,
  {
    let ignore_status_code = request_props.ignore_status_code;
    let method = request_props.method;
    let request_uri = request_props.uri.clone();
    select! {
    _ = self.cancellation_token.cancelled() => {
        warn!("Received Cancel request. Cancelling request to: {request_uri}");
        let mut app = self.app.lock().await;
        self.cancellation_token = app.reset_cancellation_token();
        app.is_loading = false;
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
                      app_update_fn(value, app);
                    }
                    Err(e) => {
                      error!("Failed to parse response! {e:?}");
                      self
                        .app
                        .lock()
                        .await
                        .handle_error(anyhow!("Failed to parse response! {e:?}"));
                    }
                  }
                }
                RequestMethod::Delete | RequestMethod::Put => (),
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
            }
          }
          Err(e) => {
            error!("Failed to send request. {e:?}");
            self
              .app
              .lock()
              .await
              .handle_error(anyhow!("Failed to send request. {e} "));
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
      RequestMethod::Delete => self.client.delete(uri).header("X-Api-Key", api_token),
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
