#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

  use crate::models::{
    Serdeable,
    lidarr_models::{Artist, ArtistStatistics, ArtistStatus, LidarrSerdeable, Ratings},
  };

  #[test]
  fn test_artist_status_default() {
    assert_eq!(ArtistStatus::default(), ArtistStatus::Continuing);
  }

  #[test]
  fn test_lidarr_serdeable_from() {
    let lidarr_serdeable = LidarrSerdeable::Value(json!({}));

    let serdeable: Serdeable = Serdeable::from(lidarr_serdeable.clone());

    assert_eq!(serdeable, Serdeable::Lidarr(lidarr_serdeable));
  }

  #[test]
  fn test_lidarr_serdeable_from_unit() {
    let lidarr_serdeable = LidarrSerdeable::from(());

    assert_eq!(lidarr_serdeable, LidarrSerdeable::Value(json!({})));
  }

  #[test]
  fn test_lidarr_serdeable_from_value() {
    let value = json!({"test": "test"});

    let lidarr_serdeable: LidarrSerdeable = value.clone().into();

    assert_eq!(lidarr_serdeable, LidarrSerdeable::Value(value));
  }

  #[test]
  fn test_lidarr_serdeable_from_artists() {
    let artists = vec![Artist {
      id: 1,
      ..Artist::default()
    }];

    let lidarr_serdeable: LidarrSerdeable = artists.clone().into();

    assert_eq!(lidarr_serdeable, LidarrSerdeable::Artists(artists));
  }

  #[test]
  fn test_artist_deserialization() {
    let artist_json = json!({
      "id": 1,
      "mbId": "test-mb-id",
      "artistName": "Test Artist",
      "foreignArtistId": "test-foreign-id",
      "status": "continuing",
      "overview": "Test overview",
      "artistType": "Group",
      "disambiguation": "UK Band",
      "path": "/music/test-artist",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "monitored": true,
      "genres": ["Rock", "Alternative"],
      "tags": [1, 2],
      "added": "2023-01-01T00:00:00Z",
      "ratings": {
        "votes": 100,
        "value": 4.5
      },
      "statistics": {
        "albumCount": 5,
        "trackFileCount": 50,
        "trackCount": 60,
        "totalTrackCount": 70,
        "sizeOnDisk": 1000000000,
        "percentOfTracks": 83.33
      }
    });

    let artist: Artist = serde_json::from_value(artist_json).unwrap();

    assert_eq!(artist.id, 1);
    assert_str_eq!(artist.artist_name.text, "Test Artist");
    assert_str_eq!(artist.foreign_artist_id, "test-foreign-id");
    assert_eq!(artist.status, ArtistStatus::Continuing);
    assert_some_eq_x!(&artist.overview, "Test overview");
    assert_some_eq_x!(&artist.artist_type, "Group");
    assert_some_eq_x!(&artist.disambiguation, "UK Band");
    assert_str_eq!(artist.path, "/music/test-artist");
    assert_eq!(artist.quality_profile_id, 1);
    assert_eq!(artist.metadata_profile_id, 1);
    assert!(artist.monitored);
    assert_eq!(artist.genres, vec!["Rock", "Alternative"]);
    assert_eq!(artist.tags.len(), 2);
    assert_some!(&artist.ratings);
    assert_some!(&artist.statistics);

    let ratings = artist.ratings.unwrap();
    assert_eq!(ratings.votes, 100);
    assert_eq!(ratings.value, 4.5);

    let stats = artist.statistics.unwrap();
    assert_eq!(stats.album_count, 5);
    assert_eq!(stats.track_file_count, 50);
    assert_eq!(stats.track_count, 60);
    assert_eq!(stats.total_track_count, 70);
    assert_eq!(stats.size_on_disk, 1000000000);
    assert_eq!(stats.percent_of_tracks, 83.33);
  }

  #[test]
  fn test_artist_status_deserialization() {
    assert_eq!(
      serde_json::from_str::<ArtistStatus>("\"continuing\"").unwrap(),
      ArtistStatus::Continuing
    );
    assert_eq!(
      serde_json::from_str::<ArtistStatus>("\"ended\"").unwrap(),
      ArtistStatus::Ended
    );
    assert_eq!(
      serde_json::from_str::<ArtistStatus>("\"deleted\"").unwrap(),
      ArtistStatus::Deleted
    );
  }

  #[test]
  fn test_ratings_equality() {
    let ratings1 = Ratings {
      votes: 100,
      value: 4.5,
    };
    let ratings2 = Ratings {
      votes: 100,
      value: 4.5,
    };
    let ratings3 = Ratings {
      votes: 50,
      value: 3.0,
    };

    assert_eq!(ratings1, ratings2);
    assert_ne!(ratings1, ratings3);
  }

  #[test]
  fn test_artist_statistics_equality() {
    let stats1 = ArtistStatistics {
      album_count: 5,
      track_file_count: 50,
      track_count: 60,
      total_track_count: 70,
      size_on_disk: 1000000000,
      percent_of_tracks: 83.33,
    };
    let stats2 = ArtistStatistics {
      album_count: 5,
      track_file_count: 50,
      track_count: 60,
      total_track_count: 70,
      size_on_disk: 1000000000,
      percent_of_tracks: 83.33,
    };
    let stats3 = ArtistStatistics::default();

    assert_eq!(stats1, stats2);
    assert_ne!(stats1, stats3);
  }

  #[test]
  fn test_artist_with_optional_fields_none() {
    let artist_json = json!({
      "id": 1,
      "mbId": "",
      "artistName": "Test Artist",
      "foreignArtistId": "",
      "status": "continuing",
      "path": "",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "monitored": false,
      "genres": [],
      "tags": [],
      "added": "2023-01-01T00:00:00Z"
    });

    let artist: Artist = serde_json::from_value(artist_json).unwrap();

    assert_none!(&artist.overview);
    assert_none!(&artist.artist_type);
    assert_none!(&artist.disambiguation);
    assert_none!(&artist.ratings);
    assert_none!(&artist.statistics);
  }
}
