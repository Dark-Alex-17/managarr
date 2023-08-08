use reqwest::Response;
use serde::de::DeserializeOwned;
use serde_json::Number;

use crate::models::radarr_models::DownloadRecord;

pub async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, reqwest::Error> {
  response.json::<T>().await
}
