use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::models::Scrollable;
use crate::app::radarr::ActiveRadarrBlock;
use crate::handlers::handle_clear_errors;
use crate::{App, Key};

pub async fn handle_radarr_key_events(
  key: Key,
  app: &mut App,
  active_radarr_block: ActiveRadarrBlock,
) {
  match key {
    _ if key == DEFAULT_KEYBINDINGS.up.key => handle_scroll_up(app, active_radarr_block).await,
    _ if key == DEFAULT_KEYBINDINGS.down.key => handle_scroll_down(app, active_radarr_block).await,
    _ if key == DEFAULT_KEYBINDINGS.left.key || key == DEFAULT_KEYBINDINGS.right.key => {
      handle_tab_action(key, app, active_radarr_block).await
    }
    _ if key == DEFAULT_KEYBINDINGS.submit.key => handle_submit(app, active_radarr_block).await,
    _ if key == DEFAULT_KEYBINDINGS.esc.key => handle_esc(app, active_radarr_block).await,
    _ => (),
  }
}

async fn handle_tab_action(key: Key, app: &mut App, active_radarr_block: ActiveRadarrBlock) {
  match active_radarr_block {
    ActiveRadarrBlock::Movies | ActiveRadarrBlock::Downloads | ActiveRadarrBlock::Collections => {
      match key {
        _ if key == DEFAULT_KEYBINDINGS.left.key => {
          app.data.radarr_data.main_tabs.previous();
          app.pop_and_push_navigation_stack(
            app.data.radarr_data.main_tabs.get_active_route().clone(),
          );
        }
        _ if key == DEFAULT_KEYBINDINGS.right.key => {
          app.data.radarr_data.main_tabs.next();
          app.pop_and_push_navigation_stack(
            app.data.radarr_data.main_tabs.get_active_route().clone(),
          );
        }
        _ => (),
      }
    }
    ActiveRadarrBlock::MovieDetails
    | ActiveRadarrBlock::MovieHistory
    | ActiveRadarrBlock::FileInfo
    | ActiveRadarrBlock::Cast
    | ActiveRadarrBlock::Crew => match key {
      _ if key == DEFAULT_KEYBINDINGS.left.key => {
        app.data.radarr_data.movie_info_tabs.previous();
        app.pop_and_push_navigation_stack(
          app
            .data
            .radarr_data
            .movie_info_tabs
            .get_active_route()
            .clone(),
        );
      }
      _ if key == DEFAULT_KEYBINDINGS.right.key => {
        app.data.radarr_data.movie_info_tabs.next();
        app.pop_and_push_navigation_stack(
          app
            .data
            .radarr_data
            .movie_info_tabs
            .get_active_route()
            .clone(),
        );
      }
      _ => (),
    },
    _ => (),
  }
}

async fn handle_scroll_up(app: &mut App, active_radarr_block: ActiveRadarrBlock) {
  match active_radarr_block {
    ActiveRadarrBlock::Collections => app.data.radarr_data.collections.scroll_up(),
    ActiveRadarrBlock::CollectionDetails => app.data.radarr_data.collection_movies.scroll_up(),
    ActiveRadarrBlock::Movies => app.data.radarr_data.movies.scroll_up(),
    ActiveRadarrBlock::MovieDetails => app.data.radarr_data.movie_details.scroll_up(),
    ActiveRadarrBlock::MovieHistory => app.data.radarr_data.movie_history.scroll_up(),
    ActiveRadarrBlock::Cast => app.data.radarr_data.movie_cast.scroll_up(),
    ActiveRadarrBlock::Crew => app.data.radarr_data.movie_crew.scroll_up(),
    ActiveRadarrBlock::Downloads => app.data.radarr_data.downloads.scroll_up(),
    _ => (),
  }
}

async fn handle_scroll_down(app: &mut App, active_radarr_block: ActiveRadarrBlock) {
  match active_radarr_block {
    ActiveRadarrBlock::Collections => app.data.radarr_data.collections.scroll_down(),
    ActiveRadarrBlock::CollectionDetails => app.data.radarr_data.collection_movies.scroll_down(),
    ActiveRadarrBlock::Movies => app.data.radarr_data.movies.scroll_down(),
    ActiveRadarrBlock::MovieDetails => app.data.radarr_data.movie_details.scroll_down(),
    ActiveRadarrBlock::MovieHistory => app.data.radarr_data.movie_history.scroll_down(),
    ActiveRadarrBlock::Cast => app.data.radarr_data.movie_cast.scroll_down(),
    ActiveRadarrBlock::Crew => app.data.radarr_data.movie_crew.scroll_down(),
    ActiveRadarrBlock::Downloads => app.data.radarr_data.downloads.scroll_down(),
    _ => (),
  }
}

async fn handle_submit(app: &mut App, active_radarr_block: ActiveRadarrBlock) {
  match active_radarr_block {
    ActiveRadarrBlock::Movies => app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into()),
    ActiveRadarrBlock::Collections => {
      app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into())
    }
    ActiveRadarrBlock::CollectionDetails => {
      app.push_navigation_stack(ActiveRadarrBlock::ViewMovieOverview.into())
    }
    _ => (),
  }
}

async fn handle_esc(app: &mut App, active_radarr_block: ActiveRadarrBlock) {
  match active_radarr_block {
    ActiveRadarrBlock::MovieDetails
    | ActiveRadarrBlock::MovieHistory
    | ActiveRadarrBlock::FileInfo
    | ActiveRadarrBlock::Cast
    | ActiveRadarrBlock::Crew => {
      app.pop_navigation_stack();
      app.data.radarr_data.reset_movie_info_tabs();
    }
    ActiveRadarrBlock::CollectionDetails => {
      app.pop_navigation_stack();
      app.data.radarr_data.reset_movie_collection_table();
    }
    ActiveRadarrBlock::ViewMovieOverview => app.pop_navigation_stack(),
    _ => handle_clear_errors(app).await,
  }
}
