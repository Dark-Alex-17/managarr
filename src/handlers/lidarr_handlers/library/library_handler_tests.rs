#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  use pretty_assertions::assert_str_eq;
  use serde_json::Number;
  use strum::IntoEnumIterator;

  use crate::handlers::lidarr_handlers::library::{LibraryHandler, artists_sorting_options};
  use crate::handlers::KeyEventHandler;
  use crate::models::lidarr_models::{Artist, ArtistStatistics, ArtistStatus};
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, LIBRARY_BLOCKS};

  #[test]
  fn test_library_handler_accepts() {
    for lidarr_block in ActiveLidarrBlock::iter() {
      if LIBRARY_BLOCKS.contains(&lidarr_block) {
        assert!(LibraryHandler::accepts(lidarr_block));
      } else {
        assert!(!LibraryHandler::accepts(lidarr_block));
      }
    }
  }

  #[test]
  fn test_artists_sorting_options_name() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.artist_name
        .text
        .to_lowercase()
        .cmp(&b.artist_name.text.to_lowercase())
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[0].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Name");
  }

  #[test]
  fn test_artists_sorting_options_type() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.artist_type
        .as_ref()
        .unwrap_or(&String::new())
        .to_lowercase()
        .cmp(
          &b.artist_type
            .as_ref()
            .unwrap_or(&String::new())
            .to_lowercase(),
        )
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[1].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Type");
  }

  #[test]
  fn test_artists_sorting_options_status() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.status
        .to_string()
        .to_lowercase()
        .cmp(&b.status.to_string().to_lowercase())
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[2].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Status");
  }

  #[test]
  fn test_artists_sorting_options_quality_profile() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering =
      |a, b| a.quality_profile_id.cmp(&b.quality_profile_id);
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[3].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Quality Profile");
  }

  #[test]
  fn test_artists_sorting_options_metadata_profile() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering =
      |a, b| a.metadata_profile_id.cmp(&b.metadata_profile_id);
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[4].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Metadata Profile");
  }

  #[test]
  fn test_artists_sorting_options_albums() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.statistics
        .as_ref()
        .map_or(0, |stats| stats.album_count)
        .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.album_count))
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[5].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Albums");
  }

  #[test]
  fn test_artists_sorting_options_tracks() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.statistics
        .as_ref()
        .map_or(0, |stats| stats.track_count)
        .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.track_count))
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[6].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Tracks");
  }

  #[test]
  fn test_artists_sorting_options_size() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.statistics
        .as_ref()
        .map_or(0, |stats| stats.size_on_disk)
        .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.size_on_disk))
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[7].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Size");
  }

  #[test]
  fn test_artists_sorting_options_monitored() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| a.monitored.cmp(&b.monitored);
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[8].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Monitored");
  }

  #[test]
  fn test_artists_sorting_options_tags() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      let a_str = a
        .tags
        .iter()
        .map(|tag| tag.as_i64().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
      let b_str = b
        .tags
        .iter()
        .map(|tag| tag.as_i64().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
      a_str.cmp(&b_str)
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[9].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Tags");
  }

  fn artists_vec() -> Vec<Artist> {
    vec![
      Artist {
        id: 3,
        artist_name: "Test Artist 1".into(),
        artist_type: Some("Group".to_owned()),
        status: ArtistStatus::Ended,
        quality_profile_id: 1,
        metadata_profile_id: 1,
        monitored: false,
        tags: vec![Number::from(1), Number::from(2)],
        statistics: Some(ArtistStatistics {
          album_count: 5,
          track_count: 50,
          size_on_disk: 789,
          ..ArtistStatistics::default()
        }),
        ..Artist::default()
      },
      Artist {
        id: 2,
        artist_name: "Test Artist 2".into(),
        artist_type: Some("Solo".to_owned()),
        status: ArtistStatus::Continuing,
        quality_profile_id: 2,
        metadata_profile_id: 2,
        monitored: false,
        tags: vec![Number::from(1), Number::from(3)],
        statistics: Some(ArtistStatistics {
          album_count: 10,
          track_count: 100,
          size_on_disk: 456,
          ..ArtistStatistics::default()
        }),
        ..Artist::default()
      },
      Artist {
        id: 1,
        artist_name: "Test Artist 3".into(),
        artist_type: None,
        status: ArtistStatus::Deleted,
        quality_profile_id: 3,
        metadata_profile_id: 3,
        monitored: true,
        tags: vec![Number::from(2), Number::from(3)],
        statistics: Some(ArtistStatistics {
          album_count: 3,
          track_count: 30,
          size_on_disk: 123,
          ..ArtistStatistics::default()
        }),
        ..Artist::default()
      },
    ]
  }
}
