use std::sync::Arc;

use reqwest::Client;
use tokio::sync::Mutex;

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
}
