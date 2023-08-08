use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::models::Scrollable;
use crate::app::radarr::ActiveRadarrBlock;
use crate::{App, Key};

pub async fn handle_radarr_key_events(
  key: Key,
  app: &mut App,
  active_radarr_block: ActiveRadarrBlock,
) {
  match key {
    _ if key == DEFAULT_KEYBINDINGS.up.key => handle_scroll_up(app, active_radarr_block).await,
    _ if key == DEFAULT_KEYBINDINGS.down.key => handle_scroll_down(app, active_radarr_block).await,
    _ if key == DEFAULT_KEYBINDINGS.submit.key => handle_submit(app, active_radarr_block).await,
    _ if key == DEFAULT_KEYBINDINGS.esc.key => handle_esc(app, active_radarr_block).await,
    _ => (),
  }
}

async fn handle_scroll_up(app: &mut App, active_radarr_block: ActiveRadarrBlock) {
  match active_radarr_block {
    ActiveRadarrBlock::Movies => app.data.radarr_data.movies.scroll_up(),
    ActiveRadarrBlock::MovieDetails => app.data.radarr_data.movie_details.scroll_up(),
    _ => (),
  }
}

async fn handle_scroll_down(app: &mut App, active_radarr_block: ActiveRadarrBlock) {
  match active_radarr_block {
    ActiveRadarrBlock::Movies => app.data.radarr_data.movies.scroll_down(),
    ActiveRadarrBlock::MovieDetails => app.data.radarr_data.movie_details.scroll_down(),
    _ => (),
  }
}

async fn handle_submit(app: &mut App, active_radarr_block: ActiveRadarrBlock) {
  match active_radarr_block {
    ActiveRadarrBlock::Movies => app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into()),
    _ => (),
  }
}

async fn handle_esc(app: &mut App, active_radarr_block: ActiveRadarrBlock) {
  match active_radarr_block {
    ActiveRadarrBlock::MovieDetails => app.pop_navigation_stack(),
    _ => (),
  }
}
