use anyhow::Result;
use log::info;

use super::{NetworkEvent, NetworkResource};
use crate::models::lidarr_models::{
  DeleteArtistParams, EditArtistParams, LidarrSerdeable, MetadataProfile,
};
use crate::models::servarr_models::{QualityProfile, Tag};
use crate::network::{Network, RequestMethod};

mod downloads;
mod library;
mod root_folders;
mod system;

#[cfg(test)]
#[path = "lidarr_network_tests.rs"]
mod lidarr_network_tests;

#[cfg(test)]
#[path = "lidarr_network_test_utils.rs"]
pub mod lidarr_network_test_utils;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LidarrEvent {
  AddTag(String),
  DeleteArtist(DeleteArtistParams),
  DeleteTag(i64),
  EditArtist(EditArtistParams),
  GetArtistDetails(i64),
  GetDiskSpace,
  GetDownloads(u64),
  GetHostConfig,
  GetMetadataProfiles,
  GetQualityProfiles,
  GetRootFolders,
  GetSecurityConfig,
  GetStatus,
  GetTags,
  HealthCheck,
  ListArtists,
  ToggleArtistMonitoring(i64),
  UpdateAllArtists,
}

impl NetworkResource for LidarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      LidarrEvent::AddTag(_) | LidarrEvent::DeleteTag(_) | LidarrEvent::GetTags => "/tag",
      LidarrEvent::DeleteArtist(_)
      | LidarrEvent::EditArtist(_)
      | LidarrEvent::GetArtistDetails(_)
      | LidarrEvent::ListArtists
      | LidarrEvent::ToggleArtistMonitoring(_) => "/artist",
      LidarrEvent::GetDiskSpace => "/diskspace",
      LidarrEvent::GetDownloads(_) => "/queue",
      LidarrEvent::GetHostConfig | LidarrEvent::GetSecurityConfig => "/config/host",
      LidarrEvent::UpdateAllArtists => "/command",
      LidarrEvent::GetMetadataProfiles => "/metadataprofile",
      LidarrEvent::GetQualityProfiles => "/qualityprofile",
      LidarrEvent::GetRootFolders => "/rootfolder",
      LidarrEvent::GetStatus => "/system/status",
      LidarrEvent::HealthCheck => "/health",
    }
  }
}

impl From<LidarrEvent> for NetworkEvent {
  fn from(lidarr_event: LidarrEvent) -> Self {
    NetworkEvent::Lidarr(lidarr_event)
  }
}

impl Network<'_, '_> {
  pub async fn handle_lidarr_event(
    &mut self,
    lidarr_event: LidarrEvent,
  ) -> Result<LidarrSerdeable> {
    match lidarr_event {
      LidarrEvent::AddTag(tag) => self.add_lidarr_tag(tag).await.map(LidarrSerdeable::from),
      LidarrEvent::DeleteArtist(params) => {
        self.delete_artist(params).await.map(LidarrSerdeable::from)
      }
      LidarrEvent::DeleteTag(tag_id) => self
        .delete_lidarr_tag(tag_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetArtistDetails(artist_id) => self
        .get_artist_details(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetDiskSpace => self.get_lidarr_diskspace().await.map(LidarrSerdeable::from),
      LidarrEvent::GetDownloads(count) => self
        .get_lidarr_downloads(count)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetHostConfig => self
        .get_lidarr_host_config()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetMetadataProfiles => self
        .get_lidarr_metadata_profiles()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetQualityProfiles => self
        .get_lidarr_quality_profiles()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetRootFolders => self
        .get_lidarr_root_folders()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetSecurityConfig => self
        .get_lidarr_security_config()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetStatus => self.get_lidarr_status().await.map(LidarrSerdeable::from),
      LidarrEvent::GetTags => self.get_lidarr_tags().await.map(LidarrSerdeable::from),
      LidarrEvent::HealthCheck => self
        .get_lidarr_healthcheck()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ListArtists => self.list_artists().await.map(LidarrSerdeable::from),
      LidarrEvent::ToggleArtistMonitoring(artist_id) => self
        .toggle_artist_monitoring(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::UpdateAllArtists => self.update_all_artists().await.map(LidarrSerdeable::from),
      LidarrEvent::EditArtist(params) => self.edit_artist(params).await.map(LidarrSerdeable::from),
    }
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_healthcheck(&mut self) -> Result<()> {
    info!("Performing Lidarr health check");
    let event = LidarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn get_lidarr_metadata_profiles(&mut self) -> Result<Vec<MetadataProfile>> {
    info!("Fetching Lidarr metadata profiles");
    let event = LidarrEvent::GetMetadataProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<MetadataProfile>>(request_props, |metadata_profiles, mut app| {
        app.data.lidarr_data.metadata_profile_map = metadata_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  async fn get_lidarr_quality_profiles(&mut self) -> Result<Vec<QualityProfile>> {
    info!("Fetching Lidarr quality profiles");
    let event = LidarrEvent::GetQualityProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QualityProfile>>(request_props, |quality_profiles, mut app| {
        app.data.lidarr_data.quality_profile_map = quality_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  async fn get_lidarr_tags(&mut self) -> Result<Vec<Tag>> {
    info!("Fetching Lidarr tags");
    let event = LidarrEvent::GetTags;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Tag>>(request_props, |tags_vec, mut app| {
        app.data.lidarr_data.tags_map = tags_vec
          .into_iter()
          .map(|tag| (tag.id, tag.label))
          .collect();
      })
      .await
  }

  async fn add_lidarr_tag(&mut self, tag: String) -> Result<Tag> {
    info!("Adding a new Lidarr tag");
    let event = LidarrEvent::AddTag(String::new());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(serde_json::json!({ "label": tag })),
        None,
        None,
      )
      .await;

    self
      .handle_request::<serde_json::Value, Tag>(request_props, |tag, mut app| {
        app.data.lidarr_data.tags_map.insert(tag.id, tag.label);
      })
      .await
  }

  async fn delete_lidarr_tag(&mut self, id: i64) -> Result<()> {
    info!("Deleting Lidarr tag with ID: {id}");
    let event = LidarrEvent::DeleteTag(id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn extract_and_add_lidarr_tag_ids_vec(
    &mut self,
    edit_tags: &str,
  ) -> Vec<i64> {
    let missing_tags_vec = {
      let tags_map = &self.app.lock().await.data.lidarr_data.tags_map;
      edit_tags
        .split(',')
        .filter(|&tag| {
          !tag.is_empty() && tags_map.get_by_right(tag.to_lowercase().trim()).is_none()
        })
        .collect::<Vec<&str>>()
    };

    for tag in missing_tags_vec {
      self
        .add_lidarr_tag(tag.trim().to_owned())
        .await
        .expect("Unable to add tag");
    }

    let app = self.app.lock().await;
    edit_tags
      .split(',')
      .filter(|tag| !tag.is_empty())
      .map(|tag| {
        *app
          .data
          .lidarr_data
          .tags_map
          .get_by_right(tag.to_lowercase().trim())
          .unwrap()
      })
      .collect()
  }
}
