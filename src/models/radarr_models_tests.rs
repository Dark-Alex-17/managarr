#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;

  use crate::models::radarr_models::{MinimumAvailability, Monitor};

  #[test]
  fn test_minimum_availability_display() {
    assert_str_eq!(MinimumAvailability::Tba.to_string(), "tba");
    assert_str_eq!(MinimumAvailability::Announced.to_string(), "announced");
    assert_str_eq!(MinimumAvailability::InCinemas.to_string(), "inCinemas");
    assert_str_eq!(MinimumAvailability::Released.to_string(), "released");
  }

  #[test]
  fn test_minimum_availability_to_display_str() {
    assert_str_eq!(MinimumAvailability::Tba.to_display_str(), "TBA");
    assert_str_eq!(MinimumAvailability::Announced.to_display_str(), "Announced");
    assert_str_eq!(
      MinimumAvailability::InCinemas.to_display_str(),
      "In Cinemas"
    );
    assert_str_eq!(MinimumAvailability::Released.to_display_str(), "Released");
  }

  #[test]
  fn test_monitor_display() {
    assert_str_eq!(Monitor::MovieOnly.to_string(), "movieOnly");
    assert_str_eq!(
      Monitor::MovieAndCollection.to_string(),
      "movieAndCollection"
    );
    assert_str_eq!(Monitor::None.to_string(), "none");
  }

  #[test]
  fn test_monitor_to_display_str() {
    assert_str_eq!(Monitor::MovieOnly.to_display_str(), "Movie only");
    assert_str_eq!(
      Monitor::MovieAndCollection.to_display_str(),
      "Movie and Collection"
    );
    assert_str_eq!(Monitor::None.to_display_str(), "None");
  }
}
