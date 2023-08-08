#[cfg(test)]
mod tests {
  mod radarr_data_tests {
    use bimap::BiMap;
    use chrono::{DateTime, Utc};
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::app::radarr::radarr_test_utils::utils::create_test_radarr_data;
    use crate::app::radarr::{ActiveRadarrBlock, RadarrData};
    use crate::models::radarr_models::{
      Collection, MinimumAvailability, Monitor, Movie, RootFolder,
    };
    use crate::models::Route;
    use crate::models::StatefulTable;
    use crate::models::{BlockSelectionState, HorizontallyScrollableText};
    use crate::{
      assert_edit_media_reset, assert_filter_reset, assert_movie_info_tabs_reset,
      assert_preferences_selections_reset, assert_search_reset,
    };

    #[test]
    fn test_from_tuple_to_route_with_context() {
      assert_eq!(
        Route::from((
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::AddMovieSearchResults)
        )),
        Route::Radarr(
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::AddMovieSearchResults)
        )
      );
    }

    #[test]
    fn test_reset_movie_collection_table() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_movie_collection_table();

      assert!(radarr_data.collection_movies.items.is_empty());
    }

    #[test]
    fn test_reset_log_details_list() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_log_details_list();

      assert!(radarr_data.log_details.items.is_empty());
    }

    #[test]
    fn test_reset_delete_movie_preferences() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_delete_movie_preferences();

      assert!(!radarr_data.delete_movie_files);
      assert!(!radarr_data.add_list_exclusion);
    }

    #[test]
    fn test_reset_search() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_search();

      assert_search_reset!(radarr_data);
    }

    #[test]
    fn test_reset_filter() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_filter();

      assert_filter_reset!(radarr_data);
    }

    #[test]
    fn test_reset_movie_info_tabs() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_movie_info_tabs();

      assert_movie_info_tabs_reset!(radarr_data);
    }

    #[test]
    fn test_reset_add_edit_media_fields() {
      let mut radarr_data = RadarrData {
        edit_monitored: Some(true),
        edit_search_on_add: Some(true),
        edit_path: "test path".to_owned().into(),
        edit_tags: "test tag".to_owned().into(),
        ..RadarrData::default()
      };

      radarr_data.reset_add_edit_media_fields();

      assert_edit_media_reset!(radarr_data);
    }

    #[test]
    fn test_reset_preferences_selections() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_preferences_selections();

      assert_preferences_selections_reset!(radarr_data);
    }

    #[test]
    fn test_populate_preferences_lists() {
      let root_folder = RootFolder {
        id: Number::from(1),
        path: "/nfs".to_owned(),
        accessible: true,
        free_space: Number::from(219902325555200u64),
        unmapped_folders: None,
      };
      let mut radarr_data = RadarrData {
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        ..RadarrData::default()
      };
      radarr_data
        .root_folders
        .set_items(vec![root_folder.clone()]);

      radarr_data.populate_preferences_lists();

      assert_eq!(
        radarr_data.monitor_list.items,
        Vec::from_iter(Monitor::iter())
      );
      assert_eq!(
        radarr_data.minimum_availability_list.items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        radarr_data.quality_profile_list.items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_eq!(radarr_data.root_folder_list.items, vec![root_folder]);
    }

    #[rstest]
    fn test_populate_edit_movie_fields(#[values(true, false)] test_filtered_movies: bool) {
      let mut radarr_data = RadarrData {
        edit_path: HorizontallyScrollableText::default(),
        edit_tags: HorizontallyScrollableText::default(),
        edit_monitored: None,
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
        filtered_movies: StatefulTable::default(),
        ..create_test_radarr_data()
      };
      let movie = Movie {
        path: "/nfs/movies/Test".to_owned(),
        monitored: true,
        quality_profile_id: Number::from(2222),
        minimum_availability: MinimumAvailability::Released,
        tags: vec![Number::from(1), Number::from(2)],
        ..Movie::default()
      };

      if test_filtered_movies {
        radarr_data.filtered_movies.set_items(vec![movie]);
      } else {
        radarr_data.movies.set_items(vec![movie]);
      }

      radarr_data.populate_edit_movie_fields();

      assert_eq!(
        radarr_data.minimum_availability_list.items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        radarr_data.minimum_availability_list.current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        radarr_data.quality_profile_list.items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        radarr_data.quality_profile_list.current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(radarr_data.edit_path.text, "/nfs/movies/Test");
      assert_str_eq!(radarr_data.edit_tags.text, "usenet, test");
      assert_eq!(radarr_data.edit_monitored, Some(true));
    }

    #[rstest]
    fn test_populate_edit_collection_fields(
      #[values(true, false)] test_filtered_collections: bool,
    ) {
      let mut radarr_data = RadarrData {
        edit_path: HorizontallyScrollableText::default(),
        edit_monitored: None,
        edit_search_on_add: None,
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        filtered_collections: StatefulTable::default(),
        ..create_test_radarr_data()
      };
      let collection = Collection {
        root_folder_path: Some("/nfs/movies/Test".to_owned()),
        monitored: true,
        search_on_add: true,
        quality_profile_id: Number::from(2222),
        minimum_availability: MinimumAvailability::Released,
        ..Collection::default()
      };

      if test_filtered_collections {
        radarr_data.filtered_collections.set_items(vec![collection]);
      } else {
        radarr_data.collections.set_items(vec![collection]);
      }

      radarr_data.populate_edit_collection_fields();

      assert_eq!(
        radarr_data.minimum_availability_list.items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        radarr_data.minimum_availability_list.current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        radarr_data.quality_profile_list.items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        radarr_data.quality_profile_list.current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(radarr_data.edit_path.text, "/nfs/movies/Test");
      assert_eq!(radarr_data.edit_monitored, Some(true));
      assert_eq!(radarr_data.edit_search_on_add, Some(true));
    }

    #[test]
    fn test_radarr_data_defaults() {
      let radarr_data = RadarrData::default();

      assert!(radarr_data.root_folders.items.is_empty());
      assert_eq!(radarr_data.disk_space_vec, Vec::new());
      assert!(radarr_data.version.is_empty());
      assert_eq!(radarr_data.start_time, <DateTime<Utc>>::default());
      assert!(radarr_data.movies.items.is_empty());
      assert!(radarr_data.add_searched_movies.items.is_empty());
      assert!(radarr_data.monitor_list.items.is_empty());
      assert!(radarr_data.minimum_availability_list.items.is_empty());
      assert!(radarr_data.quality_profile_list.items.is_empty());
      assert!(radarr_data.root_folder_list.items.is_empty());
      assert_eq!(radarr_data.selected_block, BlockSelectionState::default());
      assert!(radarr_data.filtered_movies.items.is_empty());
      assert!(radarr_data.downloads.items.is_empty());
      assert!(radarr_data.indexers.items.is_empty());
      assert!(radarr_data.indexer_settings.is_none());
      assert!(radarr_data.quality_profile_map.is_empty());
      assert!(radarr_data.tags_map.is_empty());
      assert!(radarr_data.file_details.is_empty());
      assert!(radarr_data.audio_details.is_empty());
      assert!(radarr_data.video_details.is_empty());
      assert!(radarr_data.movie_details.get_text().is_empty());
      assert!(radarr_data.movie_history.items.is_empty());
      assert!(radarr_data.movie_cast.items.is_empty());
      assert!(radarr_data.movie_crew.items.is_empty());
      assert!(radarr_data.movie_releases.items.is_empty());
      assert!(radarr_data.movie_releases_sort.items.is_empty());
      assert!(radarr_data.collections.items.is_empty());
      assert!(radarr_data.filtered_collections.items.is_empty());
      assert!(radarr_data.collection_movies.items.is_empty());
      assert!(radarr_data.logs.items.is_empty());
      assert!(radarr_data.log_details.items.is_empty());
      assert!(radarr_data.tasks.items.is_empty());
      assert!(radarr_data.queued_events.items.is_empty());
      assert!(radarr_data.updates.get_text().is_empty());
      assert!(radarr_data.prompt_confirm_action.is_none());
      assert!(radarr_data.search.text.is_empty());
      assert!(radarr_data.filter.text.is_empty());
      assert!(radarr_data.edit_path.text.is_empty());
      assert!(radarr_data.edit_tags.text.is_empty());
      assert!(radarr_data.edit_monitored.is_none());
      assert!(radarr_data.edit_search_on_add.is_none());
      assert!(radarr_data.sort_ascending.is_none());
      assert!(!radarr_data.is_searching);
      assert!(!radarr_data.is_filtering);
      assert!(!radarr_data.prompt_confirm);
      assert!(!radarr_data.delete_movie_files);
      assert!(!radarr_data.add_list_exclusion);

      assert_eq!(radarr_data.main_tabs.tabs.len(), 6);

      assert_str_eq!(radarr_data.main_tabs.tabs[0].title, "Library");
      assert_eq!(
        radarr_data.main_tabs.tabs[0].route,
        ActiveRadarrBlock::Movies.into()
      );
      assert!(radarr_data.main_tabs.tabs[0].help.is_empty());
      assert_eq!(radarr_data.main_tabs.tabs[0].contextual_help,
								 Some("<a> add | <e> edit | <del> delete | <s> search | <f> filter | <r> refresh | <u> update all | <enter> details | <esc> cancel filter"));

      assert_str_eq!(radarr_data.main_tabs.tabs[1].title, "Downloads");
      assert_eq!(
        radarr_data.main_tabs.tabs[1].route,
        ActiveRadarrBlock::Downloads.into()
      );
      assert!(radarr_data.main_tabs.tabs[1].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[1].contextual_help,
        Some("<r> refresh | <del> delete")
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[2].title, "Collections");
      assert_eq!(
        radarr_data.main_tabs.tabs[2].route,
        ActiveRadarrBlock::Collections.into()
      );
      assert!(radarr_data.main_tabs.tabs[2].help.is_empty());
      assert_eq!(radarr_data.main_tabs.tabs[2].contextual_help,
								 Some("<s> search | <e> edit | <f> filter | <r> refresh | <u> update all | <enter> details | <esc> cancel filter"));

      assert_str_eq!(radarr_data.main_tabs.tabs[3].title, "Root Folders");
      assert_eq!(
        radarr_data.main_tabs.tabs[3].route,
        ActiveRadarrBlock::RootFolders.into()
      );
      assert!(radarr_data.main_tabs.tabs[3].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[3].contextual_help,
        Some("<a> add | <del> delete | <r> refresh")
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[4].title, "Indexers");
      assert_eq!(
        radarr_data.main_tabs.tabs[4].route,
        ActiveRadarrBlock::Indexers.into()
      );
      assert!(radarr_data.main_tabs.tabs[4].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[4].contextual_help,
        Some("<enter> edit | <s> settings | <del> delete | <r> refresh")
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[5].title, "System");
      assert_eq!(
        radarr_data.main_tabs.tabs[5].route,
        ActiveRadarrBlock::System.into()
      );
      assert!(radarr_data.main_tabs.tabs[5].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[5].contextual_help,
        Some("<t> open tasks | <e> open events | <l> open logs | <u> open updates | <r> refresh")
      );

      assert_eq!(radarr_data.movie_info_tabs.tabs.len(), 6);

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[0].title, "Details");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[0].route,
        ActiveRadarrBlock::MovieDetails.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[0].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[0]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[1].title, "History");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[1].route,
        ActiveRadarrBlock::MovieHistory.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[1].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[1]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[2].title, "File");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[2].route,
        ActiveRadarrBlock::FileInfo.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[2].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[2]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[3].title, "Cast");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[3].route,
        ActiveRadarrBlock::Cast.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[3].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[3]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[4].title, "Crew");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[4].route,
        ActiveRadarrBlock::Crew.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[4].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[4]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[5].title, "Manual Search");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[5].route,
        ActiveRadarrBlock::ManualSearch.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[5].help,
        "<r> refresh | <u> update | <e> edit | <o> sort | <s> auto search | <esc> close"
      );
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[5].contextual_help,
        Some("<enter> details")
      );
    }
  }

  mod active_radarr_block_tests {
    use pretty_assertions::assert_eq;

    use crate::app::radarr::{
      ActiveRadarrBlock, ADD_MOVIE_SELECTION_BLOCKS, DELETE_MOVIE_SELECTION_BLOCKS,
      EDIT_COLLECTION_SELECTION_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS,
    };

    #[test]
    fn test_add_movie_prompt_block_order() {
      let mut add_movie_block_iter = ADD_MOVIE_SELECTION_BLOCKS.iter();

      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectRootFolder
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectMonitor
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectQualityProfile
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieTagsInput
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieConfirmPrompt
      );
    }

    #[test]
    fn test_edit_movie_prompt_block_order() {
      let mut edit_movie_block_iter = EDIT_MOVIE_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieToggleMonitored
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieSelectMinimumAvailability
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieSelectQualityProfile
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMoviePathInput
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieTagsInput
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieConfirmPrompt
      );
    }

    #[test]
    fn test_edit_collection_prompt_block_order() {
      let mut edit_collection_block_iter = EDIT_COLLECTION_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionToggleMonitored
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionSelectQualityProfile
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionToggleSearchOnAdd
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionConfirmPrompt
      );
    }

    #[test]
    fn test_delete_movie_prompt_block_order() {
      let mut delete_movie_block_iter = DELETE_MOVIE_SELECTION_BLOCKS.iter();
      assert_eq!(
        delete_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::DeleteMovieToggleDeleteFile
      );
      assert_eq!(
        delete_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::DeleteMovieToggleAddListExclusion
      );
      assert_eq!(
        delete_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::DeleteMovieConfirmPrompt
      );
    }
  }

  mod radarr_tests {
    use pretty_assertions::assert_eq;
    use tokio::sync::mpsc;

    use crate::app::radarr::ActiveRadarrBlock;
    use crate::app::App;
    use crate::models::radarr_models::{Collection, CollectionMovie, Credit, Release};
    use crate::models::StatefulTable;
    use crate::network::radarr_network::RadarrEvent;
    use crate::network::NetworkEvent;

    #[tokio::test]
    async fn test_dispatch_by_collections_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Collections)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetCollections.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_collection_details_block() {
      let (mut app, _) = construct_app_unit();

      app.data.radarr_data.collections.set_items(vec![Collection {
        movies: Some(vec![CollectionMovie::default()]),
        ..Collection::default()
      }]);

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::CollectionDetails)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
      assert_eq!(app.tick_count, 0);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_dispatch_by_collection_details_block_with_add_movie() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::AddMovie);

      app.data.radarr_data.collections.set_items(vec![Collection {
        movies: Some(vec![CollectionMovie::default()]),
        ..Collection::default()
      }]);

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::CollectionDetails)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::AddMovie.into()
      );
      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
      assert_eq!(app.tick_count, 0);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_dispatch_by_downloads_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Downloads)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_root_folders_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::RootFolders)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_movies_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Movies)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovies.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_indexers_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Indexers)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetIndexers.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_indexer_settings_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::IndexerSettings)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetIndexerSettings.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_system_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::System)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetTasks.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQueuedEvents.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetLogs.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_system_updates_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::SystemUpdates)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetUpdates.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_add_movie_search_results_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::AddMovieSearchResults)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::SearchNewMovie.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_movie_details_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::MovieDetails)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieDetails.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_file_info_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::FileInfo)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieDetails.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_movie_history_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::MovieHistory)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieHistory.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app.data.radarr_data.movie_cast = StatefulTable::default();
        app.data.radarr_data.movie_crew = StatefulTable::default();

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(app.is_loading);
        assert_eq!(
          sync_network_rx.recv().await.unwrap(),
          RadarrEvent::GetMovieCredits.into()
        );
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks_movie_cast_non_empty() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app
          .data
          .radarr_data
          .movie_cast
          .set_items(vec![Credit::default()]);

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(app.is_loading);
        assert_eq!(
          sync_network_rx.recv().await.unwrap(),
          RadarrEvent::GetMovieCredits.into()
        );
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks_movie_crew_non_empty() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app
          .data
          .radarr_data
          .movie_crew
          .set_items(vec![Credit::default()]);

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(app.is_loading);
        assert_eq!(
          sync_network_rx.recv().await.unwrap(),
          RadarrEvent::GetMovieCredits.into()
        );
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks_cast_and_crew_non_empty() {
      let mut app = App::default();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app
          .data
          .radarr_data
          .movie_cast
          .set_items(vec![Credit::default()]);
        app
          .data
          .radarr_data
          .movie_crew
          .set_items(vec![Credit::default()]);

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(!app.is_loading);
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_search_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetReleases.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_search_block_movie_releases_non_empty() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movie_releases
        .set_items(vec![Release::default()]);

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_search_block_is_loading() {
      let mut app = App {
        is_loading: true,
        ..App::default()
      };

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
        .await;

      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_check_for_prompt_action_no_prompt_confirm() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = false;

      app.check_for_prompt_action().await;

      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.should_refresh);
    }

    #[tokio::test]
    async fn test_check_for_prompt_action() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::GetStatus);

      app.check_for_prompt_action().await;

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetStatus.into()
      );
      assert!(app.should_refresh);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[tokio::test]
    async fn test_radarr_refresh_metadata() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.is_routing = true;

      app.refresh_metadata().await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_first_render() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app.radarr_on_tick(ActiveRadarrBlock::Downloads, true).await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetOverview.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetStatus.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_not_routing() {
      let mut app = App::default();

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert!(!app.is_routing);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_routing() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.is_routing = true;

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_should_refresh() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.should_refresh = true;

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(app.should_refresh);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_network_tick_frequency() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.tick_count = 2;
      app.tick_until_poll = 2;

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_populate_movie_collection_table_unfiltered() {
      let mut app = App::default();
      app.data.radarr_data.collections.set_items(vec![Collection {
        movies: Some(vec![CollectionMovie::default()]),
        ..Collection::default()
      }]);

      app.populate_movie_collection_table().await;

      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
    }

    #[tokio::test]
    async fn test_populate_movie_collection_table_filtered() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_collections
        .set_items(vec![Collection {
          movies: Some(vec![CollectionMovie::default()]),
          ..Collection::default()
        }]);

      app.populate_movie_collection_table().await;

      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
    }

    fn construct_app_unit<'a>() -> (App<'a>, mpsc::Receiver<NetworkEvent>) {
      let (sync_network_tx, sync_network_rx) = mpsc::channel::<NetworkEvent>(500);
      let mut app = App {
        network_tx: Some(sync_network_tx),
        tick_count: 1,
        ..App::default()
      };
      app.data.radarr_data.prompt_confirm = true;

      (app, sync_network_rx)
    }
  }
}
