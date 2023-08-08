use anyhow::Result;
use log::{debug, error};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::app::radarr::RadarrData;
use crate::app::RadarrConfig;
use crate::network::{Network, RadarrEvent};

impl RadarrEvent {
    const fn resource(self) -> &'static str {
        match self {
            RadarrEvent::HealthCheck => "/health",
            RadarrEvent::GetOverview => "/diskspace"
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpace {
    pub path: String,
    pub label: String,
    pub free_space: Number,
    pub total_space: Number
}

impl<'a> Network<'a> {
    pub async fn handle_radarr_event(&self, radarr_event: RadarrEvent) {
        match radarr_event {
            RadarrEvent::HealthCheck => {
                self.healthcheck(RadarrEvent::HealthCheck.resource()).await;
            }
            RadarrEvent::GetOverview => {
                match self.diskspace(RadarrEvent::GetOverview.resource()).await {
                    Ok(disk_space_vec) => {
                        let mut app = self.app.lock().await;
                        app.data.radarr_data = RadarrData::from(&disk_space_vec[0]);
                    }
                    Err(e) => {
                        error!("Failed to fetch disk space. {:?}", e);
                    }
                }
            }
        }

        let mut app = self.app.lock().await;
        app.reset();
    }

    async fn healthcheck(&self, resource: &str) {
        if let Err(e) = self.call_radarr_api(resource).await.send().await {
            error!("Healthcheck failed. {:?}", e)
        }
    }

    async fn diskspace(&self, resource: &str) -> Result<Vec<DiskSpace>> {
        debug!("Handling diskspace event: {:?}", resource);

        Ok(
            self.call_radarr_api(resource)
                .await
                .send()
                .await?
                .json::<Vec<DiskSpace>>()
                .await?
        )
    }

    async fn call_radarr_api(&self, resource: &str) -> RequestBuilder {
        debug!("Creating RequestBuilder for resource: {:?}", resource);
        let app = self.app.lock().await;
        let RadarrConfig {  host, port, api_token } = &app.config.radarr;

        app.client.get(format!("http://{}:{}/api/v3{}", host, port.unwrap_or(7878), resource))
            .header("X-Api-Key", api_token)
    }
}