use anyhow::Result;

use super::{NetworkEvent, NetworkResource};
use crate::models::readarr_models::ReadarrSerdeable;
use crate::network::Network;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ReadarrEvent {}

impl NetworkResource for ReadarrEvent {
  fn resource(&self) -> &'static str {
    match *self {}
  }
}

impl From<ReadarrEvent> for NetworkEvent {
  fn from(readarr_event: ReadarrEvent) -> Self {
    NetworkEvent::Readarr(readarr_event)
  }
}

impl Network<'_, '_> {
  pub async fn handle_readarr_event(
    &mut self,
    readarr_event: ReadarrEvent,
  ) -> Result<ReadarrSerdeable> {
    match readarr_event {}
  }
}
