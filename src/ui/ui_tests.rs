#[cfg(test)]
mod snapshot_tests {
  use crate::app::App;
  use crate::handlers::populate_keymapping_table;
	use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
	use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::ui;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

  #[test]
  fn test_radarr_ui_renders_library_tab() {
    let mut app = App::test_default_fully_populated();
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
      ui(f, app);
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_radarr_ui_renders_library_tab_with_error() {
    let mut app = App::test_default_fully_populated();
    app.error = "Some error".into();
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
      ui(f, app);
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_radarr_ui_renders_library_tab_error_popup() {
    let mut app = App::test_default_fully_populated();
    populate_keymapping_table(&mut app);
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
      ui(f, app);
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_sonarr_ui_renders_library_tab() {
  	let mut app = App::test_default_fully_populated();
  	app.push_navigation_stack(ActiveSonarrBlock::default().into());

  	let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
  		ui(f, app);
  	});

  	insta::assert_snapshot!(output);
  }

  #[test]
  fn test_sonarr_ui_renders_library_tab_with_error() {
  	let mut app = App::test_default_fully_populated();
  	app.push_navigation_stack(ActiveSonarrBlock::default().into());
   app.error = "Some error".into();

  	let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
  		ui(f, app);
  	});

  	insta::assert_snapshot!(output);
  }

  #[test]
  fn test_sonarr_ui_renders_library_tab_error_popup() {
  	let mut app = App::test_default_fully_populated();
  	populate_keymapping_table(&mut app);
  	app.push_navigation_stack(ActiveSonarrBlock::default().into());

  	let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
  		ui(f, app);
  	});

  	insta::assert_snapshot!(output);
  }

	#[test]
	fn test_lidarr_ui_renders_library_tab() {
		let mut app = App::test_default_fully_populated();
		app.push_navigation_stack(ActiveLidarrBlock::default().into());

		let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
			ui(f, app);
		});

		insta::assert_snapshot!(output);
	}

	#[test]
	fn test_lidarr_ui_renders_library_tab_with_error() {
		let mut app = App::test_default_fully_populated();
		app.push_navigation_stack(ActiveLidarrBlock::default().into());
		app.error = "Some error".into();

		let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
			ui(f, app);
		});

		insta::assert_snapshot!(output);
	}

	#[test]
	fn test_lidarr_ui_renders_library_tab_error_popup() {
		let mut app = App::test_default_fully_populated();
		populate_keymapping_table(&mut app);
		app.push_navigation_stack(ActiveLidarrBlock::default().into());

		let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
			ui(f, app);
		});

		insta::assert_snapshot!(output);
	}
}
