use crate::app::{App, Route};
use crate::event::Key;
use crate::handlers::radarr_handler::handle_radarr_key_events;

mod radarr_handler;

pub async fn handle_key_events(key: Key, app: &mut App) {
  match app.get_current_route().clone() {
    Route::Radarr(active_radarr_block) => {
      handle_radarr_key_events(key, app, active_radarr_block).await
    }
    _ => (),
  }
}

pub async fn handle_clear_errors(app: &mut App) {
  if !app.error.is_empty() {
    app.error = String::default();
  }
}
