use std::sync::Arc;

use reqwest::Client;
use tokio::sync::Mutex;

use crate::app::App;

pub(crate) mod radarr;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum RadarrEvent {
  HealthCheck,
  GetOverview,
  GetStatus,
}

pub struct Network<'a> {
  pub client: Client,

  pub app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
  pub fn new(client: Client, app: &'a Arc<Mutex<App>>) -> Self {
    Network { client, app }
  }
}
