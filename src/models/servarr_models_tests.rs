#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::models::servarr_models::{
    AuthenticationMethod, AuthenticationRequired, CertificateValidation, QualityProfile,
  };

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
}
