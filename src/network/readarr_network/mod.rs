use anyhow::Result;
use log::info;

use super::{NetworkEvent, NetworkResource};
use crate::models::readarr_models::{ReadarrSerdeable, ReadarrTaskName};
use crate::network::{Network, RequestMethod};

mod system;

#[cfg(test)]
#[path = "readarr_network_tests.rs"]
mod readarr_network_tests;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ReadarrEvent {
  GetDiskSpace,
  GetHostConfig,
  GetLogs(u64),
  GetQueuedEvents,
  GetSecurityConfig,
  GetStatus,
  GetTasks,
  GetUpdates,
  HealthCheck,
  StartTask(ReadarrTaskName),
}

impl NetworkResource for ReadarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      ReadarrEvent::GetQueuedEvents | ReadarrEvent::StartTask(_) => "/command",
      ReadarrEvent::GetHostConfig | ReadarrEvent::GetSecurityConfig => "/config/host",
      ReadarrEvent::GetDiskSpace => "/diskspace",
      ReadarrEvent::HealthCheck => "/health",
      ReadarrEvent::GetLogs(_) => "/log",
      ReadarrEvent::GetStatus => "/system/status",
      ReadarrEvent::GetTasks => "/system/task",
      ReadarrEvent::GetUpdates => "/update",
    }
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
    match readarr_event {
      ReadarrEvent::GetDiskSpace => self
        .get_readarr_diskspace()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetHostConfig => self
        .get_readarr_host_config()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetLogs(events) => self
        .get_readarr_logs(events)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetQueuedEvents => self
        .get_queued_readarr_events()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetSecurityConfig => self
        .get_readarr_security_config()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetStatus => self.get_readarr_status().await.map(ReadarrSerdeable::from),
      ReadarrEvent::GetTasks => self.get_readarr_tasks().await.map(ReadarrSerdeable::from),
      ReadarrEvent::GetUpdates => self.get_readarr_updates().await.map(ReadarrSerdeable::from),
      ReadarrEvent::HealthCheck => self
        .get_readarr_healthcheck()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::StartTask(task_name) => self
        .start_readarr_task(task_name)
        .await
        .map(ReadarrSerdeable::from),
    }
  }

  pub(in crate::network::readarr_network) async fn get_readarr_healthcheck(
    &mut self,
  ) -> Result<()> {
    info!("Performing Readarr health check");
    let event = ReadarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }
}
