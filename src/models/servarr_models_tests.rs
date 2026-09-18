#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde::de::DeserializeOwned;
  use serde_json::{Value, json};

  use crate::models::servarr_models::{
    AuthenticationMethod, AuthenticationRequired, CertificateValidation, DownloadStatus, Indexer,
    QualityProfile, SecurityConfig, Update, UpdateChanges,
  };

  #[test]
  fn test_indexer_default() {
    let indexer = Indexer::default();

    assert_eq!(indexer.id, 0);
    assert_none!(indexer.name);
    assert_none!(indexer.implementation);
    assert_none!(indexer.implementation_name);
    assert_none!(indexer.config_contract);
    assert!(!indexer.supports_rss);
    assert!(!indexer.supports_search);
    assert_none!(indexer.fields);
    assert!(!indexer.enable_rss);
    assert!(!indexer.enable_automatic_search);
    assert!(!indexer.enable_interactive_search);
    assert_is_empty!(indexer.protocol);
    assert_eq!(indexer.priority, 1);
    assert_eq!(indexer.download_client_id, 0);
    assert_is_empty!(indexer.tags);
  }

  #[test]
  fn test_authentication_method_display() {
    assert_str_eq!(AuthenticationMethod::Basic.to_string(), "basic");
    assert_str_eq!(AuthenticationMethod::Forms.to_string(), "forms");
    assert_str_eq!(AuthenticationMethod::None.to_string(), "none");
  }

  #[test]
  fn test_authentication_required_display() {
    assert_str_eq!(AuthenticationRequired::Enabled.to_string(), "enabled");
    assert_str_eq!(
      AuthenticationRequired::DisabledForLocalAddresses.to_string(),
      "disabledForLocalAddresses"
    );
  }

  #[test]
  fn test_certificate_validation_display() {
    assert_str_eq!(CertificateValidation::Enabled.to_string(), "enabled");
    assert_str_eq!(
      CertificateValidation::DisabledForLocalAddresses.to_string(),
      "disabledForLocalAddresses"
    );
    assert_str_eq!(CertificateValidation::Disabled.to_string(), "disabled");
  }

  #[test]
  fn test_download_status_display() {
    assert_str_eq!(DownloadStatus::Unknown.to_string(), "unknown");
    assert_str_eq!(DownloadStatus::Queued.to_string(), "queued");
    assert_str_eq!(DownloadStatus::Paused.to_string(), "paused");
    assert_str_eq!(DownloadStatus::Downloading.to_string(), "downloading");
    assert_str_eq!(DownloadStatus::Completed.to_string(), "completed");
    assert_str_eq!(DownloadStatus::Failed.to_string(), "failed");
    assert_str_eq!(DownloadStatus::Warning.to_string(), "warning");
    assert_str_eq!(DownloadStatus::Delay.to_string(), "delay");
    assert_str_eq!(
      DownloadStatus::DownloadClientUnavailable.to_string(),
      "downloadClientUnavailable"
    );
    assert_str_eq!(DownloadStatus::Fallback.to_string(), "fallback");
  }

  #[test]
  fn test_download_status_to_display_str() {
    assert_str_eq!(DownloadStatus::Unknown.to_display_str(), "Unknown");
    assert_str_eq!(DownloadStatus::Queued.to_display_str(), "Queued");
    assert_str_eq!(DownloadStatus::Paused.to_display_str(), "Paused");
    assert_str_eq!(DownloadStatus::Downloading.to_display_str(), "Downloading");
    assert_str_eq!(DownloadStatus::Completed.to_display_str(), "Completed");
    assert_str_eq!(DownloadStatus::Failed.to_display_str(), "Failed");
    assert_str_eq!(DownloadStatus::Warning.to_display_str(), "Warning");
    assert_str_eq!(DownloadStatus::Delay.to_display_str(), "Delay");
    assert_str_eq!(
      DownloadStatus::DownloadClientUnavailable.to_display_str(),
      "Download Client Unavailable"
    );
    assert_str_eq!(DownloadStatus::Fallback.to_display_str(), "Fallback");
  }

  #[test]
  fn test_quality_profile_from_tuple_ref() {
    let id = 2;
    let name = "Test".to_owned();
    let quality_profile_tuple = (&id, &name);
    let expected_quality_profile = QualityProfile {
      id: 2,
      name: "Test".to_owned(),
    };

    let quality_profile = QualityProfile::from(quality_profile_tuple);

    assert_eq!(expected_quality_profile, quality_profile);
  }

  #[test]
  fn test_update_deserializes_when_changes_is_absent() {
    let update_json = r#"{
      "version": "0.4.18.2805",
      "releaseDate": "2025-06-15T06:22:22Z",
      "installed": false,
      "latest": true,
      "installedOn": "2025-06-22T05:09:11Z"
    }"#;

    let update = serde_json::from_str::<Update>(update_json);

    assert_ok!(&update);
    assert_eq!(
      update.unwrap().changes,
      UpdateChanges {
        new: None,
        fixed: None,
      }
    );
  }

  #[test]
  fn test_security_config_authentication_method_deserialization_falls_back_to_default() {
    let security_config_json = serde_json::to_string(&security_config()).unwrap();
    let expected = SecurityConfig {
      authentication_method: AuthenticationMethod::default(),
      ..security_config()
    };

    assert_eq!(
      deserialize_with_field::<SecurityConfig>(
        &security_config_json,
        "authenticationMethod",
        json!(2)
      ),
      expected
    );
    assert_eq!(
      deserialize_with_field::<SecurityConfig>(
        &security_config_json,
        "authenticationMethod",
        json!("notAThing")
      ),
      expected
    );
  }

  #[test]
  fn test_security_config_authentication_required_deserialization_falls_back_to_none() {
    let security_config_json = serde_json::to_string(&security_config()).unwrap();
    let expected = SecurityConfig {
      authentication_required: None,
      ..security_config()
    };

    assert_eq!(
      deserialize_with_field::<SecurityConfig>(
        &security_config_json,
        "authenticationRequired",
        json!(2)
      ),
      expected
    );
    assert_eq!(
      deserialize_with_field::<SecurityConfig>(
        &security_config_json,
        "authenticationRequired",
        json!("notAThing")
      ),
      expected
    );
  }

  #[test]
  fn test_security_config_deserializes_when_authentication_required_is_absent() {
    let mut security_config_json = serde_json::to_value(security_config()).unwrap();
    security_config_json
      .as_object_mut()
      .unwrap()
      .remove("authenticationRequired");
    let expected = SecurityConfig {
      authentication_required: None,
      ..security_config()
    };

    let deserialized = serde_json::from_value::<SecurityConfig>(security_config_json);

    assert_ok!(&deserialized);
    assert_eq!(deserialized.unwrap(), expected);
  }

  #[test]
  fn test_security_config_certificate_validation_deserialization_falls_back_to_default() {
    let security_config_json = serde_json::to_string(&security_config()).unwrap();
    let expected = SecurityConfig {
      certificate_validation: CertificateValidation::default(),
      ..security_config()
    };

    assert_eq!(
      deserialize_with_field::<SecurityConfig>(
        &security_config_json,
        "certificateValidation",
        json!(2)
      ),
      expected
    );
    assert_eq!(
      deserialize_with_field::<SecurityConfig>(
        &security_config_json,
        "certificateValidation",
        json!("notAThing")
      ),
      expected
    );
  }

  fn deserialize_with_field<T: DeserializeOwned>(json: &str, field: &str, value: Value) -> T {
    let mut fixture_json: Value = serde_json::from_str(json).unwrap();
    fixture_json[field] = value;

    serde_json::from_value(fixture_json).unwrap()
  }

  fn security_config() -> SecurityConfig {
    SecurityConfig {
      authentication_method: AuthenticationMethod::Forms,
      authentication_required: Some(AuthenticationRequired::Enabled),
      username: Some("admin".to_owned()),
      password: Some("password".to_owned()),
      api_key: "test-api-key".to_owned(),
      certificate_validation: CertificateValidation::Disabled,
    }
  }
}
