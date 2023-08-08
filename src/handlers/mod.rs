use crate::app::App;
use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::event::Key;

pub async fn handle_key_events(key: Key, app: &mut App) {
    match key {
        _ if key == DEFAULT_KEYBINDINGS.up.key => handle_scroll_up(app).await,
        _ if key == DEFAULT_KEYBINDINGS.down.key => handle_scroll_down(app).await,
        _ => ()
    }
}

async fn handle_scroll_up(app: &mut App) {
    app.data.radarr_data.movies.scroll_up();
}

async fn handle_scroll_down(app: &mut App) {
    app.data.radarr_data.movies.scroll_down();
}