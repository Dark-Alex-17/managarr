use crate::models::servarr_models::{
  CommandBody, DiskSpace, HostConfig, LogResponse, QueueEvent, SecurityConfig, Update,
};
use crate::models::sonarr_models::{SonarrTask, SonarrTaskName, SystemStatus};
use crate::models::{HorizontallyScrollableText, Scrollable, ScrollableText};
use crate::network::sonarr_network::SonarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use indoc::formatdoc;
use log::info;
use serde_json::Value;

#[cfg(test)]
#[path = "sonarr_system_network_tests.rs"]
mod sonarr_system_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::sonarr_network) async fn get_sonarr_host_config(
    &mut self,
  ) -> Result<HostConfig> {
    info!("Fetching Sonarr host config");
    let event = SonarrEvent::GetHostConfig;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), HostConfig>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_sonarr_logs(
    &mut self,
    events: u64,
  ) -> Result<LogResponse> {
    info!("Fetching Sonarr logs");
    let event = SonarrEvent::GetLogs(events);

    let params = format!("pageSize={events}&sortDirection=descending&sortKey=time");
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), LogResponse>(request_props, |log_response, mut app| {
        let mut logs = log_response.records;
        logs.reverse();

        let log_lines = logs
          .into_iter()
          .map(|log| {
            if log.exception.is_some() {
              HorizontallyScrollableText::from(format!(
                "{}|{}|{}|{}|{}",
                log.time,
                log.level.to_uppercase(),
                log.logger.as_ref().unwrap(),
                log.exception_type.as_ref().unwrap(),
                log.exception.as_ref().unwrap()
              ))
            } else {
              HorizontallyScrollableText::from(format!(
                "{}|{}|{}|{}",
                log.time,
                log.level.to_uppercase(),
                log.logger.as_ref().unwrap(),
                log.message.as_ref().unwrap()
              ))
            }
          })
          .collect();

        app.data.sonarr_data.logs.set_items(log_lines);
        app.data.sonarr_data.logs.scroll_to_bottom();
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_sonarr_diskspace(
    &mut self,
  ) -> Result<Vec<DiskSpace>> {
    info!("Fetching Sonarr disk space");
    let event = SonarrEvent::GetDiskSpace;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<DiskSpace>>(request_props, |disk_space_vec, mut app| {
        app.data.sonarr_data.disk_space_vec = disk_space_vec;
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_queued_sonarr_events(
    &mut self,
  ) -> Result<Vec<QueueEvent>> {
    info!("Fetching Sonarr queued events");
    let event = SonarrEvent::GetQueuedEvents;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QueueEvent>>(request_props, |queued_events_vec, mut app| {
        app
          .data
          .sonarr_data
          .queued_events
          .set_items(queued_events_vec);
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_sonarr_security_config(
    &mut self,
  ) -> Result<SecurityConfig> {
    info!("Fetching Sonarr security config");
    let event = SonarrEvent::GetSecurityConfig;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SecurityConfig>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_sonarr_status(
    &mut self,
  ) -> Result<SystemStatus> {
    info!("Fetching Sonarr system status");
    let event = SonarrEvent::GetStatus;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.sonarr_data.version = system_status.version;
        app.data.sonarr_data.start_time = system_status.start_time;
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_sonarr_tasks(
    &mut self,
  ) -> Result<Vec<SonarrTask>> {
    info!("Fetching Sonarr tasks");
    let event = SonarrEvent::GetTasks;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<SonarrTask>>(request_props, |tasks_vec, mut app| {
        app.data.sonarr_data.tasks.set_items(tasks_vec);
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_sonarr_updates(
    &mut self,
  ) -> Result<Vec<Update>> {
    info!("Fetching Sonarr updates");
    let event = SonarrEvent::GetUpdates;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Update>>(request_props, |updates_vec, mut app| {
        let latest_installed = if updates_vec
          .iter()
          .any(|update| update.latest && update.installed_on.is_some())
        {
          "already".to_owned()
        } else {
          "not".to_owned()
        };
        let updates = updates_vec
          .into_iter()
          .map(|update| {
            let install_status = if update.installed_on.is_some() {
              if update.installed {
                "(Currently Installed)".to_owned()
              } else {
                "(Previously Installed)".to_owned()
              }
            } else {
              String::new()
            };
            let vec_to_bullet_points = |vec: Vec<String>| {
              vec
                .iter()
                .map(|change| format!("  * {change}"))
                .collect::<Vec<String>>()
                .join("\n")
            };

            let mut update_info = formatdoc!(
              "{} - {} {install_status}
              {}",
              update.version,
              update.release_date,
              "-".repeat(200)
            );

            if let Some(new_changes) = update.changes.new {
              let changes = vec_to_bullet_points(new_changes);
              update_info = formatdoc!(
                "{update_info}
              New:
              {changes}"
              )
            }

            if let Some(fixes) = update.changes.fixed {
              let fixes = vec_to_bullet_points(fixes);
              update_info = formatdoc!(
                "{update_info}
              Fixed:
              {fixes}"
              );
            }

            update_info
          })
          .reduce(|version_1, version_2| format!("{version_1}\n\n\n{version_2}"))
          .unwrap();

        app.data.sonarr_data.updates = ScrollableText::with_string(formatdoc!(
          "The latest version of Sonarr is {latest_installed} installed
          
          {updates}"
        ));
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn start_sonarr_task(
    &mut self,
    task: SonarrTaskName,
  ) -> Result<Value> {
    let event = SonarrEvent::StartTask(task);
    let task_name = task.to_string();
    info!("Starting Sonarr task: {task_name}");

    let body = CommandBody { name: task_name };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }
}
