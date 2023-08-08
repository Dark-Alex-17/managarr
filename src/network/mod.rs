use std::fmt::Debug;
use std::sync::Arc;

use anyhow::anyhow;
use log::{debug, error};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::{Mutex, MutexGuard};

use crate::app::App;
use crate::network::radarr_network::RadarrEvent;

pub(crate) mod radarr_network;
mod utils;

#[derive(PartialEq, Eq, Debug)]
pub enum NetworkEvent {
  Radarr(RadarrEvent),
}

pub struct Network<'a> {
  pub client: Client,

  pub app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
  pub fn new(client: Client, app: &'a Arc<Mutex<App>>) -> Self {
    Network { client, app }
  }

  pub async fn handle_network_event(&self, network_event: NetworkEvent) {
    match network_event {
      NetworkEvent::Radarr(radarr_event) => self.handle_radarr_event(radarr_event).await,
    }

    let mut app = self.app.lock().await;
    app.is_loading = false;
  }

  pub async fn handle_request<T, R>(
    &self,
    request_props: RequestProps<T>,
    mut app_update_fn: impl FnMut(R, MutexGuard<App>),
  ) where
    T: Serialize + Default + Debug,
    R: DeserializeOwned,
  {
    let method = request_props.method.clone();
    match self.call_api(request_props).await.send().await {
      Ok(response) => {
        if response.status().is_success() {
          match method {
            RequestMethod::Get => match utils::parse_response::<R>(response).await {
              Ok(value) => {
                let app = self.app.lock().await;
                app_update_fn(value, app);
              }
              Err(e) => {
                error!("Failed to parse response! {:?}", e);
                self.app.lock().await.handle_error(anyhow!(e));
              }
            },
            RequestMethod::Delete | RequestMethod::Post => (),
          }
        } else {
          error!(
            "Request failed. Received {} response code",
            response.status()
          );
          self.app.lock().await.handle_error(anyhow!(
            "Request failed. Received {} response code",
            response.status()
          ));
        }
      }
      Err(e) => {
        error!("Failed to send request. {:?}", e);
        self.app.lock().await.handle_error(anyhow!(e));
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
    } = request_props;
    debug!("Creating RequestBuilder for resource: {:?}", uri);
    let app = self.app.lock().await;
    debug!(
      "Sending {:?} request to {} with body {:?}",
      method, uri, body
    );

    match method {
      RequestMethod::Get => app.client.get(uri).header("X-Api-Key", api_token),
      RequestMethod::Post => app
        .client
        .post(uri)
        .json(&body.unwrap_or_default())
        .header("X-Api-Key", api_token),
      RequestMethod::Delete => app.client.delete(uri).header("X-Api-Key", api_token),
    }
  }
}

#[derive(Clone, Debug)]
pub enum RequestMethod {
  Get,
  Post,
  Delete,
}

#[derive(Debug)]
pub struct RequestProps<T: Serialize + Debug> {
  pub uri: String,
  pub method: RequestMethod,
  pub body: Option<T>,
  pub api_token: String,
}
