use reqwest::Response;
use serde::de::DeserializeOwned;
use serde_json::Number;

use crate::network::radarr_network::DownloadRecord;

pub async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, reqwest::Error> {
  response.json::<T>().await
}

pub fn get_movie_status(
  has_file: bool,
  downloads_vec: &[DownloadRecord],
  movie_id: Number,
) -> String {
  if !has_file {
    if let Some(download) = downloads_vec
      .iter()
      .find(|&download| download.movie_id.as_u64().unwrap() == movie_id.as_u64().unwrap())
    {
      if download.status == "downloading" {
        return "Downloading".to_owned();
      }
    }

    return "Missing".to_owned();
  }

  "Downloaded".to_owned()
}
