#[cfg(test)]
mod tests {
  use anyhow::anyhow;
  use pretty_assertions::assert_eq;
  use tokio::sync::mpsc;

  use crate::app::context_clues::{build_context_clue_string, SERVARR_CONTEXT_CLUES};
  use crate::app::{App, AppConfig, Data, ServarrConfig, DEFAULT_ROUTE};
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, RadarrData};
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SonarrData};
  use crate::models::{HorizontallyScrollableText, TabRoute};
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::NetworkEvent;

  #[test]
  fn test_app_default() {
    let app = App::default();

    assert_eq!(app.navigation_stack, vec![DEFAULT_ROUTE]);
    assert!(app.network_tx.is_none());
    assert!(!app.cancellation_token.is_cancelled());
    assert!(app.is_first_render);
    assert_eq!(app.error, HorizontallyScrollableText::default());
    assert_eq!(app.server_tabs.index, 0);
    assert_eq!(
      app.server_tabs.tabs,
      vec![
        TabRoute {
          title: "Radarr",
          route: ActiveRadarrBlock::Movies.into(),
          help: format!(
            "<↑↓> scroll | ←→ change tab | {}  ",
            build_context_clue_string(&SERVARR_CONTEXT_CLUES)
          ),
          contextual_help: None,
        },
        TabRoute {
          title: "Sonarr",
          route: ActiveSonarrBlock::Series.into(),
          help: format!("{}  ", build_context_clue_string(&SERVARR_CONTEXT_CLUES)),
          contextual_help: None,
        },
      ]
    );
    assert_eq!(app.tick_until_poll, 400);
    assert_eq!(app.ticks_until_scroll, 4);
    assert_eq!(app.tick_count, 0);
    assert!(!app.is_loading);
    assert!(!app.is_routing);
    assert!(!app.should_refresh);
    assert!(!app.should_ignore_quit_key);
    assert!(!app.cli_mode);
  }

  #[test]
  fn test_navigation_stack_methods() {
    let mut app = App::default();

    assert_eq!(app.get_current_route(), DEFAULT_ROUTE);

    app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());

    assert_eq!(app.get_current_route(), ActiveRadarrBlock::Downloads.into());
    assert!(app.is_routing);

    app.is_routing = false;
    app.pop_and_push_navigation_stack(ActiveRadarrBlock::Collections.into());

    assert_eq!(
      app.get_current_route(),
      ActiveRadarrBlock::Collections.into()
    );
    assert!(app.is_routing);

    app.is_routing = false;
    app.pop_navigation_stack();

    assert_eq!(app.get_current_route(), DEFAULT_ROUTE);
    assert!(app.is_routing);

    app.is_routing = false;
    app.pop_navigation_stack();

    assert_eq!(app.get_current_route(), DEFAULT_ROUTE);
    assert!(app.is_routing);
  }

  #[test]
  fn test_reset_cancellation_token() {
    let mut app = App {
      is_loading: true,
      should_refresh: false,
      ..App::default()
    };
    app.cancellation_token.cancel();

    assert!(app.cancellation_token.is_cancelled());

    let new_token = app.reset_cancellation_token();

    assert!(!app.cancellation_token.is_cancelled());
    assert!(!new_token.is_cancelled());
    assert!(!app.is_loading);
    assert!(app.should_refresh);
  }

  #[test]
  fn test_reset_tick_count() {
    let mut app = App {
      tick_count: 2,
      ..App::default()
    };

    app.reset_tick_count();

    assert_eq!(app.tick_count, 0);
  }

  #[test]
  fn test_reset() {
    let radarr_data = RadarrData {
      version: "test".into(),
      ..RadarrData::default()
    };
    let sonarr_data = SonarrData {
      version: "test".into(),
      ..SonarrData::default()
    };
    let data = Data {
      radarr_data,
      sonarr_data,
    };
    let mut app = App {
      tick_count: 2,
      error: "Test error".to_owned().into(),
      is_first_render: false,
      data,
      ..App::default()
    };

    app.reset();

    assert_eq!(app.tick_count, 0);
    assert_eq!(app.error, HorizontallyScrollableText::default());
    assert!(app.is_first_render);
    assert!(app.data.radarr_data.version.is_empty());
    assert!(app.data.sonarr_data.version.is_empty());
  }

  #[test]
  fn test_handle_error() {
    let mut app = App::default();
    let test_string = "Testing";

    app.handle_error(anyhow!(test_string));

    assert_eq!(app.error.text, test_string);

    app.handle_error(anyhow!("Testing a different error"));

    assert_eq!(app.error.text, test_string);
  }

  #[tokio::test]
  async fn test_dispatch_network_event() {
    let (sync_network_tx, mut sync_network_rx) = mpsc::channel::<NetworkEvent>(500);

    let mut app = App {
      tick_until_poll: 2,
      network_tx: Some(sync_network_tx),
      ..App::default()
    };

    assert_eq!(app.tick_count, 0);

    app
      .dispatch_network_event(RadarrEvent::GetStatus.into())
      .await;

    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetStatus.into()
    );
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_on_tick_first_render() {
    let (sync_network_tx, mut sync_network_rx) = mpsc::channel::<NetworkEvent>(500);

    let mut app = App {
      tick_until_poll: 2,
      network_tx: Some(sync_network_tx),
      is_first_render: true,
      ..App::default()
    };

    assert_eq!(app.tick_count, 0);

    app.on_tick().await;

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
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDiskSpace.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetStatus.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetMovies.into()
    );
    assert!(!app.is_routing);
    assert!(!app.should_refresh);
    assert_eq!(app.tick_count, 1);
  }

  #[tokio::test]
  async fn test_on_tick_routing() {
    let mut app = App {
      tick_until_poll: 2,
      tick_count: 2,
      is_routing: true,
      ..App::default()
    };

    app.on_tick().await;
    assert!(!app.is_routing);
  }

  #[tokio::test]
  async fn test_on_tick_should_refresh() {
    let mut app = App {
      tick_until_poll: 2,
      tick_count: 2,
      should_refresh: true,
      ..App::default()
    };

    app.on_tick().await;
    assert!(!app.should_refresh);
  }

  #[test]
  fn test_app_config_default() {
    let app_config = AppConfig::default();

    assert!(app_config.radarr.is_none());
    assert!(app_config.sonarr.is_none());
  }

  #[test]
  fn test_servarr_config_default() {
    let servarr_config = ServarrConfig::default();

    assert_eq!(servarr_config.host, Some("localhost".to_string()));
    assert_eq!(servarr_config.port, None);
    assert_eq!(servarr_config.uri, None);
    assert!(servarr_config.api_token.is_empty());
    assert_eq!(servarr_config.ssl_cert_path, None);
  }
}
