use history::HistoryHandler;
use indexers::IndexersHandler;
use library::LibraryHandler;
use root_folders::RootFoldersHandler;
use system::SystemHandler;

use crate::{
	app::App, event::Key, matches_key, models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock,
};

use super::KeyEventHandler;

mod history;
mod indexers;
mod library;
mod root_folders;
mod system;

// #[cfg(test)]
// #[path = "lidarr_handler_tests.rs"]
// mod lidarr_handler_tests;

// #[cfg(test)]
// #[path = "lidarr_handler_test_utils.rs"]
// mod lidarr_handler_test_utils;

pub(super) struct LidarrHandler<'a, 'b> {
	key: Key,
	app: &'a mut App<'b>,
	active_lidarr_block: ActiveLidarrBlock,
	context: Option<ActiveLidarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for LidarrHandler<'a, 'b> {
	fn handle(&mut self) {
		match self.active_lidarr_block {
			_ if LibraryHandler::accepts(self.active_lidarr_block) => {
				LibraryHandler::new(self.key, self.app, self.active_lidarr_block, self.context).handle();
			}
			_ if HistoryHandler::accepts(self.active_lidarr_block) => {
				HistoryHandler::new(self.key, self.app, self.active_lidarr_block, self.context).handle()
			}
			_ if RootFoldersHandler::accepts(self.active_lidarr_block) => {
				RootFoldersHandler::new(self.key, self.app, self.active_lidarr_block, self.context).handle()
			}
			_ if IndexersHandler::accepts(self.active_lidarr_block) => {
				IndexersHandler::new(self.key, self.app, self.active_lidarr_block, self.context).handle()
			}
			_ if SystemHandler::accepts(self.active_lidarr_block) => {
				SystemHandler::new(self.key, self.app, self.active_lidarr_block, self.context).handle()
			}
			_ => self.handle_key_event(),
		}
	}

	fn accepts(_active_block: ActiveLidarrBlock) -> bool {
		true
	}

	fn ignore_special_keys(&self) -> bool {
		self.app.ignore_special_keys_for_textbox_input
	}

	fn new(
		key: Key,
		app: &'a mut App<'b>,
		active_block: ActiveLidarrBlock,
		context: Option<ActiveLidarrBlock>,
	) -> LidarrHandler<'a, 'b> {
		LidarrHandler {
			key,
			app,
			active_lidarr_block: active_block,
			context,
		}
	}

	fn get_key(&self) -> Key {
		self.key
	}

	fn is_ready(&self) -> bool {
		true
	}

	fn handle_scroll_up(&mut self) {}

	fn handle_scroll_down(&mut self) {}

	fn handle_home(&mut self) {}

	fn handle_end(&mut self) {}

	fn handle_delete(&mut self) {}

	fn handle_left_right_action(&mut self) {}

	fn handle_submit(&mut self) {}

	fn handle_esc(&mut self) {}

	fn handle_char_key_event(&mut self) {}
}

pub fn handle_change_tab_left_right_keys(app: &mut App<'_>, key: Key) {
	let key_ref = key;
	match key_ref {
		_ if matches_key!(left, key, app.ignore_special_keys_for_textbox_input) => {
			app.data.lidarr_data.main_tabs.previous();
			app.pop_and_push_navigation_stack(app.data.lidarr_data.main_tabs.get_active_route());
		}
		_ if matches_key!(right, key, app.ignore_special_keys_for_textbox_input) => {
			app.data.lidarr_data.main_tabs.next();
			app.pop_and_push_navigation_stack(app.data.lidarr_data.main_tabs.get_active_route());
		}
		_ => (),
	}
}
