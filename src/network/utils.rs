use reqwest::Response;
use serde::de::DeserializeOwned;

pub async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, reqwest::Error> {
  response.json::<T>().await
}
