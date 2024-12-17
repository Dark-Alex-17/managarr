use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use super::HorizontallyScrollableText;

#[cfg(test)]
#[path = "servarr_models_tests.rs"]
mod servarr_models_tests;

#[derive(Default, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AddRootFolderBody {
  pub path: String,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug, ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum AuthenticationMethod {
  #[default]
  Basic,
  Forms,
  None,
}

impl Display for AuthenticationMethod {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let authentication_method = match self {
      AuthenticationMethod::Basic => "basic",
      AuthenticationMethod::Forms => "forms",
      AuthenticationMethod::None => "none",
    };
    write!(f, "{authentication_method}")
  }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug, ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum AuthenticationRequired {
  Enabled,
  #[default]
  DisabledForLocalAddresses,
}

impl Display for AuthenticationRequired {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let authentication_required = match self {
      AuthenticationRequired::Enabled => "enabled",
      AuthenticationRequired::DisabledForLocalAddresses => "disabledForLocalAddresses",
    };
    write!(f, "{authentication_required}")
  }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug, ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum CertificateValidation {
  #[default]
  Enabled,
  DisabledForLocalAddresses,
  Disabled,
}

impl Display for CertificateValidation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let certificate_validation = match self {
      CertificateValidation::Enabled => "enabled",
      CertificateValidation::DisabledForLocalAddresses => "disabledForLocalAddresses",
      CertificateValidation::Disabled => "disabled",
    };
    write!(f, "{certificate_validation}")
  }
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandBody {
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpace {
  #[serde(deserialize_with = "super::from_i64")]
  pub free_space: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub total_space: i64,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditIndexerParams {
  pub indexer_id: i64,
  pub name: Option<String>,
  pub enable_rss: Option<bool>,
  pub enable_automatic_search: Option<bool>,
  pub enable_interactive_search: Option<bool>,
  pub url: Option<String>,
  pub api_key: Option<String>,
  pub seed_ratio: Option<String>,
  pub tags: Option<Vec<i64>>,
  pub priority: Option<i64>,
  pub clear_tags: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HostConfig {
  pub bind_address: HorizontallyScrollableText,
  #[serde(deserialize_with = "super::from_i64")]
  pub port: i64,
  pub url_base: Option<HorizontallyScrollableText>,
  pub instance_name: Option<HorizontallyScrollableText>,
  pub application_url: Option<HorizontallyScrollableText>,
  pub enable_ssl: bool,
  #[serde(deserialize_with = "super::from_i64")]
  pub ssl_port: i64,
  pub ssl_cert_path: Option<String>,
  pub ssl_cert_password: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Indexer {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub name: Option<String>,
  pub implementation: Option<String>,
  pub implementation_name: Option<String>,
  pub config_contract: Option<String>,
  pub supports_rss: bool,
  pub supports_search: bool,
  pub fields: Option<Vec<IndexerField>>,
  pub enable_rss: bool,
  pub enable_automatic_search: bool,
  pub enable_interactive_search: bool,
  pub protocol: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub priority: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub download_client_id: i64,
  pub tags: Vec<Number>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct IndexerField {
  pub name: Option<String>,
  pub value: Option<Value>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Language {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub name: String,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Log {
  pub time: DateTime<Utc>,
  pub exception: Option<String>,
  pub exception_type: Option<String>,
  pub level: String,
  pub logger: Option<String>,
  pub message: Option<String>,
  pub method: Option<String>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LogResponse {
  pub records: Vec<Log>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Quality {
  pub name: String,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct QualityProfile {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub name: String,
}

impl From<(&i64, &String)> for QualityProfile {
  fn from(value: (&i64, &String)) -> Self {
    QualityProfile {
      id: *value.0,
      name: value.1.clone(),
    }
  }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct QualityWrapper {
  pub quality: Quality,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct QueueEvent {
  pub trigger: String,
  pub name: String,
  pub command_name: String,
  pub status: String,
  pub queued: DateTime<Utc>,
  pub started: Option<DateTime<Utc>>,
  pub ended: Option<DateTime<Utc>>,
  pub duration: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RootFolder {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub path: String,
  pub accessible: bool,
  #[serde(deserialize_with = "super::from_i64")]
  pub free_space: i64,
  pub unmapped_folders: Option<Vec<UnmappedFolder>>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityConfig {
  pub authentication_method: AuthenticationMethod,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub authentication_required: Option<AuthenticationRequired>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,
  pub api_key: String,
  pub certificate_validation: CertificateValidation,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Tag {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub label: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct UnmappedFolder {
  pub name: String,
  pub path: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Update {
  pub version: String,
  pub release_date: DateTime<Utc>,
  pub installed: bool,
  pub latest: bool,
  pub installed_on: Option<DateTime<Utc>>,
  pub changes: UpdateChanges,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChanges {
  pub new: Option<Vec<String>>,
  pub fixed: Option<Vec<String>>,
}
