use crate::models::readarr_models::ReadarrTask;
use crate::models::servarr_models::{
  DiskSpace, HostConfig, LogResponse, SecurityConfig, SystemStatus, Update,
};
use crate::models::{HorizontallyScrollableText, Scrollable, ScrollableText};
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use indoc::formatdoc;
use log::info;

#[cfg(test)]
#[path = "readarr_system_network_tests.rs"]
mod readarr_system_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn get_readarr_diskspace(
    &mut self,
  ) -> Result<Vec<DiskSpace>> {
    info!("Fetching Readarr disk space");
    let event = ReadarrEvent::GetDiskSpace;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<DiskSpace>>(request_props, |disk_space_vec, mut app| {
        app.data.readarr_data.disk_space_vec = disk_space_vec;
      })
      .await
  }

  pub(in crate::network::readarr_network) async fn get_readarr_host_config(
    &mut self,
  ) -> Result<HostConfig> {
    info!("Fetching Readarr host config");
    let event = ReadarrEvent::GetHostConfig;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), HostConfig>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn get_readarr_logs(
    &mut self,
    events: u64,
  ) -> Result<LogResponse> {
    info!("Fetching Readarr logs");
    let event = ReadarrEvent::GetLogs(events);

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
            if let Some(exception) = log.exception {
              HorizontallyScrollableText::from(format!(
                "{}|{}|{}|{}|{}",
                log.time,
                log.level.to_uppercase(),
                log
                  .logger
                  .as_ref()
                  .expect("logger must exist when exception is present"),
                log
                  .exception_type
                  .as_ref()
                  .expect("exception_type must exist when exception is present"),
                exception
              ))
            } else {
              HorizontallyScrollableText::from(format!(
                "{}|{}|{}|{}",
                log.time,
                log.level.to_uppercase(),
                log.logger.as_ref().expect("logger must exist in log entry"),
                log
                  .message
                  .as_ref()
                  .expect("message must exist when exception is not present")
              ))
            }
          })
          .collect();

        app.data.readarr_data.logs.set_items(log_lines);
        app.data.readarr_data.logs.scroll_to_bottom();
      })
      .await
  }

  pub(in crate::network::readarr_network) async fn get_readarr_security_config(
    &mut self,
  ) -> Result<SecurityConfig> {
    info!("Fetching Readarr security config");
    let event = ReadarrEvent::GetSecurityConfig;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SecurityConfig>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn get_readarr_status(
    &mut self,
  ) -> Result<SystemStatus> {
    info!("Fetching Readarr system status");
    let event = ReadarrEvent::GetStatus;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.readarr_data.version = system_status.version;
        app.data.readarr_data.start_time = system_status.start_time;
      })
      .await
  }

  pub(in crate::network::readarr_network) async fn get_readarr_tasks(
    &mut self,
  ) -> Result<Vec<ReadarrTask>> {
    info!("Fetching Readarr tasks");
    let event = ReadarrEvent::GetTasks;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<ReadarrTask>>(request_props, |tasks_vec, mut app| {
        app.data.readarr_data.tasks.set_items(tasks_vec);
      })
      .await
  }

  pub(in crate::network::readarr_network) async fn get_readarr_updates(
    &mut self,
  ) -> Result<Vec<Update>> {
    info!("Fetching Readarr updates");
    let event = ReadarrEvent::GetUpdates;

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
                " (Currently Installed)".to_owned()
              } else {
                " (Previously Installed)".to_owned()
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
              "{} - {}{install_status}
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

        app.data.readarr_data.updates = ScrollableText::with_string(formatdoc!(
          "The latest version of Readarr is {latest_installed} installed
          
          {updates}"
        ));
      })
      .await
  }
}
