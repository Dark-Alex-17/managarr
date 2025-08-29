# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.6.0 (2025-08-29)

### Feat

- Support for custom headers to be added to every request to each server to support alternative authentication mechanisms [#47]
- Refactor all keybinding tips into a dynamically changing menu that can be invoked via '?' [#32]
- Display total disk usage for series in the Library view to mirror Radarr functionality [#44]
- Pagination support for jumping 20 items at a time in all table views [#45]
- Support toggling Movie monitoring directly from the library view [#43]
- Support toggling Movie monitoring from the CLI
- Support toggling Series monitoring directly from the Sonarr library view [#43]
- Support toggling Series monitoring from the CLI
- Fixed the Radarr downloads tab to display more than 10 downloads at a time and added a new --count flag to the CLI for specifying the number of downloads to return
- Fetch more than 10 downloads when listing Sonarr downloads, and add a --count flag to the CLI to specify how many downloads to fetch
- Support alternative keymappings for all keys, featuring hjkl movements
- Added the Eldritch theme and updated documentation
- Write built in themes to the themes file on first run so users can define custom themes
- Created a theme validation macro to verify theme configurations before allowing the TUI to start
- Initial support for custom user-defined themes

### Fix

- Marked Radarr studios as nullable to prevent crashes
- Fixed a bug where the Sonarr API was returning empty values for seeders when searching for season releases
- Improve fault tolerance for tag associations in Radarr and Sonarr
- Upgraded to the most recent version of Tokio to mitigate CWE-664
- Updated all dependencies and updated openssl to the most recent version to mitigate CWE-416
- Updated the name of the should_ignore_quit_key to ignore_special_keys_for_textbox_input to give a better idea of what the flag is used for; also added alt keybinding for backspace
- Marked videoCodecs as Option to resolve #38
- Marked the `Season.statistics` field as `Option` so that a panic does not happen for outdated Sonarr data. This resolves #35
- When adding a film from the Collection Details modal, the render order was wrong: Radarr Library -> Collection Table -> Add Movie Prompt (missing the Collection details prompt too). Correct order is: Collection Table -> Collection Details Modal -> Add Movie Modal
- Fixed a bug that was rendering encompassing blocks after other widgets were rendered, thus overwriting the custom styles on each previously rendered widget
- change the name of the theme configuration file to 'themes'
- Ensure key events are only processed on key press to avoid duplicates
- Updated ring dependency to mitigate CWE-770
- Modified the Sonarr DownloadRecord so that the episode_id is optional to prevent crashes for weird downloads

### Refactor

- Network module is now broken out into similar directory structures for each servarr to mimic the rest of the project to make it easier to develop and maintain
- Refactored the IndexerTestResut model into the general Servarr models
- Renamed 'ctrl-*' keyboard shortcuts to 'C-*' to simplify and shrink the on-screen help
- Formatted files using rustfmt
- Reformatted code to make the format checks pass
- Created a derive macro for defining the display style of Enum models and removed the use of the EnumDisplayStyle trait
- Expanded the serde_enum_from macro to further reduce code duplication

## v0.5.1 (2025-03-01)

### Feat

- CLI Support for multiple Servarr instances
- Support for multiple servarr definitions - no tests [skip ci]
- Support for loading Servarr API tokens from a file
- Tweaked the implementation for environment variables in the config a bit
- var interpolation

### Fix

- Updated openssl to 0.10.70 to mitigate CVE-2025-24898
- Addressed rustfmt complaints
- Corrected typo in the managarr.nuspec.template

### Refactor

- Updated dependencies
- Addressed Cargo fmt complaints
- Added a debug line for logging to output the config used when starting Managarr
- Updated the 2018 idiom lint to the 2021_compatibility lint
- Removed unnecessary clones in the networking module to speed up network request handling
- Corrected some clone instead of copy behaviors in the command line handlers
- Removed unnecessary clone from stateful table
- Removed unnecessary clone call from extract_and_add_tag_ids_vec method
- Reduced the number of clones necessary when building modal structs
- Refactored a handful of Option calls to use take instead
- Renamed KeyEventHandler::with to KeyEventHandler::new to keep with Rust best practices and conventions

## v0.4.2 (2024-12-21)

### Fix

- Revert failed release [skip ci]
- **sonarr**: Pass the series ID alongside all UpdateAndScan events when publishing to the networking channel
- **sonarr**: pass the series ID alongside all TriggerAutomaticSeriesSearch events when publishing to the networking channel
- **sonarr**: Pass the series ID and season number alongside all TriggerAutomaticSeasonSearch events when publishing to the networking channel
- **sonarr**: Pass the episode ID alongside all TriggerAutomaticEpisodeSearch events when publishing to the networking channel
- **sonarr**: Pass the episode ID alongside all ToggleEpisodeMonitoring events when publishing to the networking channel
- **sonarr**: Pass the series ID and season number alongside all toggle season monitoring events when publishing to the networking channel
- **sonarr**: Pass the indexer ID directly alongside all TestIndexer events when publishing to the networking channel
- **sonarr**: Provide the task name directly alongside all StartTask events when publishing to the networking channel
- **sonarr**: Pass the search query directly to the networking channel when searching for a new series
- **sonarr**: Pass the series ID alongside all GetSeriesHistory events when publishing to the networking channel
- **sonarr**: Pass the series ID alongside all GetSeriesDetails events when publishing to the networking channel
- **sonarr**: Pass series ID and season number alongside all ManualSeasonSearch events when publishing to the networking channel
- **sonarr**: Provide the series ID and season number alongside all GetSeasonHistory events when publishing to the networking channel
- **sonarr**: Pass the episode ID alongside all ManualEpisodeSearch events when publishing to the networking channel
- **sonarr**: Pass events alongside all GetLogs events when publishing to the networking channel
- **sonarr**: Pass the episode ID alongside all GetEpisodeHistory events when publishing to the networking channel
- **sonarr**: Pass series ID alongside all GetEpisodeFiles events when publishing to the networking channel
- **sonarr**: Pass series ID alognside all GetEpisodes events when publishing to the networking channel
- **sonarr**: Pass the episode ID alongside all GetEpisodeDetails events when publishing to the networking channel
- **sonarr**: Pass history events alongside all GetHistory events when publishing to the networking channel
- **sonarr**: Construct and pass edit series parameters alongside all EditSeries events when publishing to the networking channel
- **sonarr**: Construct and pass edit indexer parameters alongside all EditIndexer events when publishing to the networking channel
- **sonarr**: Construct and pass edit all indexer settings alongside all EditAllIndexerSettings events when publishing to the networking channel
- **sonarr**: Construct and pass delete series params alongside all DeleteSeries events when publishing to the networking channel
- **sonarr**: Corrected a bug that would cause a crash if a user spams the ESC key while searching for a new series and the search results are still loading
- **sonarr**: Pass the root folder ID alongside all DeleteRootFolder events when publishing to the networking channel
- **sonarr**: Pass the indexer ID alongside all DeleteIndexer events when publishing to the networking channel
- **sonarr**: Pass the episode file ID alongside all DeleteEpisodeFile events when publishing to the networking channel
- **sonarr**: Pass the download ID alongside all DeleteDownload events published to the networking channel
- **sonarr**: Pass the blocklist item ID alongside the DeleteBlocklistItem event when publishing to the networking channel
- **sonarr**: Construct and pass the add series body alongside AddSeries events when publishing to the networking channel
- **sonarr**: Construct and pass the AddRootFolderBody alongside all AddRootFolder events when publishing to the networking channel
- **radarr**: Pass the movie ID alongside all UpdateAndScan events published to the networking channel
- **radarr**: Provide the movie ID alongside all TriggerAutomaticMovieSearch events when publishing to the networking channel
- **radarr**: Pass in the indexer id with all TestIndexer events when publishing to the networking channel
- **radarr**: Pass in the task name alongside the StartTask event when publishing to the networking channel
- **radarr**: Pass in the search query for the SearchNewMovie event when publishing to the networking channel
- **radarr**: Pass in the movie ID alongside the GetReleases event when publishing to the networking channel
- **radarr**: Pass in the movie ID alongside the GetMovieHistory event when publishing to the networking channel
- **radarr**: Pass the movie ID in alongside the GetMovieDetaisl event when publishing to the networking channel
- **radarr**: Provide the movie id alongside the GetMovieCredits event when publishing to the networking channel
- **radarr**: Pass the number of log events to fetch in with the GetLogs event when publishing to the networking channel
- **radarr**: Construct and pass the edit movie parameters alongside the EditMovie event when publishing to the networking channel
- **radarr**: Construct and pass params when publishing the EditIndexer event to the networking channel
- **radarr**: Construct and pass edit collection parameters alongside the EditCollection event when publishing to the networking channel
- **radarr**: Build and pass the edit indexer settings body with the EditAllIndexerSettings event when publishing to the networking channel
- **radarr**: Send the parameters alongside the DownloadRelease event when publishing to the networking channel
- **radarr**: Pass the root folder ID in with the DeleteRootFolder event when publishing to the networking channel
- Pass the delete movie params in with the DeleteMovie event when publishing to the networking channel
- Pass the indexer ID in with the DeleteIndexer event when sending to the networking channel
- Pass the download ID directly in the DeleteDownload event when publishing into the networking channel
- Blocklist Item ID passed in the DeleteBlocklistItem event when sent to the networking channel
- AddRootFolderBody now constructed prior to AddRootFolder event being sent down the network channel
- Cancel all requests when switching Servarr tabs to both improve performance and fix issue #15
- **add_movie_handler_tests**: Added in a forgotten test for the build_add_movie_body function
- Missing tagged version of docker builds in release flow
- AddMovie Radarr event is now populated in the dispatch thread before being sent to the network thread
- dynamically load servarrs in UI based on what configs are provided

## v0.4.1 (2024-12-14)

### Feat

- **docs**: Updated the README with new screeshots for the Sonarr release
- **handler**: Support for toggling the monitoring status of a specified episode in the Sonarr UI
- **handlers**: Support for toggling the monitoring status of a season in the Sonarr UI
- **keybindings**: Added a new keybinding for toggling the monitoring of a highlighted table item
- **cli**: Support for toggling monitoring on a specific episode in Sonarr
- **network**: Support for toggling the monitoring status of an episode in Sonarr
- **cli**: Support for toggling monitoring for a specific season in Sonarr
- **network**: Support for toggling monitoring/unmonitoring a season
- **handlers**: Support for the episode details popup
- **ui**: Support for the episode details UI
- **handler**: Full handler support for the Season details UI in Sonarr
- **ui**: Sonarr support for viewing season details
- **cli**: Sonarr support for fetching a list of all episode files for a given series ID
- **app**: Dispatch support for Season Details to fetch both the current downloads as well as the episode files to match qualities to them
- **network**: Support for fetching all episode files for a given series
- **app**: Model and modal support for the season and episode details popups
- **cli**: Sonarr support for fetching season history events
- **network**: Sonarr support for fetching season history
- **ui**: Sonarr support for the series details popup
- **ui**: Sonarr support for editing a series from within the series details popup
- **ui**: Sonarr Series details UI is now available
- **ui**: Full Sonarr system tab support
- **handler**: System handler support for Sonarr
- **ui**: Full Sonarr support for the indexer tab
- **ui**: Support for modifying the indexer priority in Radarr
- **handler**: Full indexer tab handler support
- **ui**: Root folder tab support
- **handlers**: Support for root folder actions
- **ui**: History tab support
- **handler**: History tab support
- **ui**: Blocklist UI support
- **handler**: Wired in the blocklist handler to the main handlers
- **handler**: Blocklist handler support
- **ui**: Downloads tab support
- **handler**: Download tab support
- **ui**: Edit series support
- **handler**: Edit series support
- **ui**: Add series support Sonarr
- **handler**: Add series support for Sonarr
- **ui**: Delete a series
- **handler**: Support for deleting a series in Sonarr
- **ui**: Support for the Series table
- **handlers**: Sonarr key support for the Series table
- **models**: Added the necessary contextual help and tabs for the Sonarr UI
- **ui**: Initial UI support for switching to Sonarr tabs
- **app**: Dispatch support for all relevant Sonarr blocks

### Fix

- **blocklist_handler**: Fixed a breaking change between Sonarr v3 and v4
- **style**: Addressed linter complaints on formatting
- Implemented a handful of fixes that are breaking changes between Sonarr v3 and v4
- **handler_tests**: Fixed all delegation tests to have initial conditions set properly
- **ui**: Fixed a bug that requires a minimum height for all popups so all error messages and other simple popups appear
- **handler**: Fixed a bug in the history handler that wouldn't reset the filter or search if a user hit 'esc' on the History tab
- **ui**: Fix the System Details Tasks popup to be navigable in both Sonarr and Radarr
- **ui**: Fixed a potential rare bug in the UI where the application would panic if the height of the downloads window is 0.

### Refactor

- **network**: Changed the toggle episode monitoring handler to simply return empty since the response is always empty from Sonarr
- **ui**: Tweaked some of the color schemes in the series table
- Fixed a couple of typos in some test function names
- **handlers**: Refactored the handlers to all use the handle_table_events macro when appropriate and created tests for the macro so tests don't have to be duplicated across each handler
- **ui**: Simplified the popup delegation so all future UI is easier to implement
- **indexers_handler**: Use the new handle_table_events macro
- **root_folders_handler**: Use the new handle_table_events macro
- **blocklist_handler**: Use the new handle_table_events macro
- **downloads_handler**: Use the new handle_table_events macro
- **collection_details_handler**: use the new handle_table_events macro
- **collections_handler**: Use the new handle_table_events macro
- **movie_details_handler**: Use the new handle_table_events macro
- **library_handler**: Radarr use the new handle_table_events macro
- **indexers_handler**: Use the new handle_table_events macro
- **indexers_handler**: Use the new handle_table_events macro
- **root_folder_handler**: Use the new handle_table_events macro
- **history_handler**: Use the new handle_table_event macro
- **blocklist_handler**: Use the new handle_table_events macro
- **downloads_handler**: Use the new handle_table_events macro
- **series_details_handler**: Use the new handle_table_events macro
- **handler**: Created a macro to handle all table key events to reduce code duplication and make future implementations faster; Only refactored the Sonarr library to use it thus far
- **ui**: all table search and filter functionality is now available directly through the ManagarrTable widget to make life easier moving forward
- **keys**: Created a auto search key instead of reusing the existing search key to make things easier
- **BlockSelectionState**: Refactored so selection of blocks in 2x2 grids is more intuitive and added left() and right() methods to aid this effort.

### Perf

- Improved performance by optimizing API calls to only refresh when the tick prompts a refresh. All UI is now significantly faster

## v0.3.7 (2024-11-26)

### Fix

- **ci**: Forgot to also pull in the most recent changes [skip ci]

## v0.3.6 (2024-11-26)

### Fix

- **ci**: Ensure the Release Crate job fetches the most recent commit before publishing the crate [skip ci]

## v0.3.4 (2024-11-26)

## v0.3.3 (2024-11-26)

### Fix

- **ci**: Properly prefix version tags with 'v' [skip ci]
- **ci**: Bump the version in the Cargo.lock file and commit it as well when releasing [skip ci]

## v0.3.2 (2024-11-26)

### Fix

- **ci**: Updated the Cargo.lock file [skip ci]
- **ci**: Use a different GitHub action to release the crate to Crates.io [skip ci]
- **ci**: Don't manually push the tags and let Commitizen do it [skip ci]

## v0.3.1 (2024-11-26)

### Fix

- **ci**: Don't manually push the tags and let Commitizen do it [skip ci]
- **ci**: Fixed a typo in the version creation on GitHub [skip ci]

## v0.3.0 (2024-11-26)

### Feat

- **cli**: Support for editing a sonarr series
- **models**: Added the ActiveSonarrBlocks for editing a series
- **network**: Support for editing a series in Sonarr
- **models**: Created the EditSeriesModal
- **cli**: Support for editing Sonarr indexers
- **network**: Support for editing a sonarr indexer
- **cli**: Support for deleting an episode file from disk
- **network**: Support for deleting an episode file from disk in Sonarr
- **cli**: Support for editing all indexer settings in Sonarr
- **models**: Added the ActiveSonarrBlocks for editing all indexer settings
- **network**: Support for editing all sonarr indexer settings
- **cli**: Support for searching for new series to add to Sonarr
- **network**: Support for searching for new series
- **cli**: Support for adding a series to Sonarr
- **cli**: Support for adding a series to Sonarr
- **network**: Support for adding a new series to Sonarr
- **cli**: Support for fetching all sonarr language profiles
- **network**: Support for fetching all Sonarr language profiles
- **cli**: Support for deleting a series from Sonarr
- **network**: Support for deleting a series from Sonarr
- **cli**: Support for downloading an episode release in Sonarr
- **cli**: Support for downloading a season release in Sonarr
- **cli**: Support for downloading a Series release in Sonarr
- **network**: Support for downloading releases from Sonarr
- **cli**: Support for refreshing Sonarr downloads
- **network**: Support for updating Sonarr downloads
- **cli**: Support for refreshing a specific series in Sonarr
- **network**: Support for updating and scanning a series in Sonarr
- **cli**: Support for refreshing all Sonarr series data
- **network**: Support for updating all series in Sonarr
- **cli**: Support for triggering an automatic episode search in Sonarr
- **cli**: Support for triggering an automatic season search in Sonarr
- **cli**: Support for triggering an automatic series search in Sonarr
- **network**: Support for triggering an automatic episode search in Sonarr
- **network**: Support for triggering an automatic season search in Sonarr
- **network**: Support for triggering an automatic series search in Sonarr
- **cli**: Support for testing all Sonarr indexers at once
- **network**: Support for testing all Sonarr indexers at once
- **cli**: Support for testing an individual Sonarr indexer
- **network**: Added the ability to test an individual indexer in Sonarr
- **cli**: Support for starting a Sonarr task
- **network**: Support for starting a Sonarr task
- **cli**: Support for listing Sonarr updates
- **network**: Support for fetching Sonarr updates
- **cli**: Support for listing all Sonarr tasks
- **network**: Support for fetching all Sonarr tasks
- **cli**: Support for marking a Sonarr history item as 'failed'
- **network**: Support for marking a Sonarr history item as failed
- **cli**: Support for listing the available disk space for all provisioned root folders in both Radarr and Sonarr
- **network**: Support for listing disk space on a Sonarr instance
- **cli**: Support for listing all Sonarr tags
- **cli**: Support for adding a root folder to Sonarr
- **cli**: CLI support for adding a tag to Sonarr
- **network**: Support for fetching and listing all Sonarr tags
- **network**: Support for deleting tags from Sonarr
- **network**: Support for adding tags to Sonarr
- **network**: Support for adding a root folder to Sonarr
- **cli**: Support for deleting a root folder from Sonarr
- **network**: Support for deleting a Sonarr root folder
- **cli**: Support for fetching all Sonarr root folders
- **network**: Support for fetching all Sonarr root folders
- **cli**: Support for deleting a Sonarr indexer
- **network**: Support for deleting an indexer from Sonarr
- **cli**: Support for deleting a download from Sonarr
- **network**: Support for deleting a download from Sonarr
- **cli**: Support for fetching episode history events from Sonarr
- **network**: Support for fetching episode history
- **cli**: Added a spinner to the CLI for long running commands like fetching releases
- **cli**: Support for fetching history for a given series ID
- **network**: Support for fetching Sonarr series history for a given series ID
- **cli**: Support for fetching all Sonarr history events
- **network**: Support to fetch all Sonarr history events
- **models**: Added an additional History tab to the mocked tabs for viewing all Sonarr history at once
- **models**: Stubbed out the necessary ActiveSonarrBlocks for the UI mockup
- **cli**: Added support for manually searching for episode releases in Sonarr
- **network**: Added support for fetching episode releases in Sonarr
- **cli**: Added CLI support for fetching series details in Sonarr
- **network**: Added support for fetching series details for a given series ID in Sonarr
- **cli**: Added support for manually searching for season releases for Sonarr
- **network**: Added support for fetching season releases for Sonarr
- **cli**: Added support for listing Sonarr queued events
- **network**: Added support for fetching Sonarr queued events
- **cli**: Added CLI support for fetching all indexer settings for Sonarr
- **network**: Added netwwork support for fetching all indexer settings for Sonarr
- **cli**: Added Sonarr support for fetching host and security configs
- **network**: Added network support for fetching host and security configs from Sonarr
- **cli**: Added CLI support for listing Sonarr indexers
- **network**: Added the GetIndexers network call for Sonarr
- **cli**: Added sonarr support for listing downloads, listing quality profiles, and fetching detailed information about an episode
- **network**: Added get quality profiles and get episode details events for Sonarr
- **cli**: Sonarr CLI support for fetching all episodes for a given series
- **sonarr_network**: Added support for fetching episodes for a specified series to the network events
- **models**: Added the Episode model to Sonarr models
- **models**: Created the StatefulTree struct for displaying seasons and episodes (and any other structured data) for the UI.
- **sonarr**: Added CLI support for listing Sonarr logs
- **sonarr**: Added the ability to fetch Sonarr logs
- **sonarr**: Added blocklist commands (List, Clear, Delete)
- Added initial Sonarr CLI support and the initial network handler setup for the TUI
- Added a new command to the main managarr CLI: tail-logs, to enable users to tail the Managarr logs without needing to know where the log file itself is located

### Fix

- Reverted to old version to fix release [skip ci]
- **minimal-versions**: Addressed concerns with the minimal-versions CI checks
- **lint**: Addressed linter complaints
- **cli**: Corrected some copy/paste typos
- **network**: Force sonarr to save edits to indexers
- **network**: Made the overview field nullable in the Sonarr series model
- **network**: Added filtering for full seasons specifically in the UI when performing a manual full season search and added a message to the CLI that noes to only try to download a full season if that release includes 'fullSeason: true'
- **network**: Not all Sonarr tasks return the lastDuration field and was causing a crash
- **network**: Fixed an issue with dynamic typing in responses from Sonarr for history items
- **config**: The CLI panics if the servarr you specify has no config defined
- Imported a missing macro in the panic hook

### Refactor

- **cli**: the trigger-automatic-search commands now all have their own dedicated subcommand to keep things cleaner. Now they look like 'trigger-automatic-search episode/series/season' and their corresponding flags
- **cli**: Added an additional delegation test to ensure manual-search commands are delegated to the manual-search command handler
- **cli**: Moved the manual-season-search and manual-episode-search commands into their own dedicated handler so the commands can now be manual-search episode or manual-search season

## v0.2.2 (2024-11-06)

### Fix

- **handler**: Fixed a bug in the movie details handler that would allow key events to be processed before the data was finished loading
- **ui**: Fixed a bug that would freeze all user input while background network requests were running
- **radarr_ui**: Fixed a race condition bug in the movie details UI that would panic if the user changes tabs too quickly

### Perf

- **network**: Improved performance and reactiveness of the UI by speeding up network requests and clearing the channel whenever a request is cancelled/the UI is routing

## v0.2.1 (2024-11-06)

## [0.2.1](https://github.com/Dark-Alex-17/managarr/compare/v0.2.0...v0.2.1) - 2024-11-06

### Other

- Removed the need for use_ssl to indicate SSL usage; instead just use the ssl_cert_path
- Applied bug fix to the downloads tab as well as the context [skip ci]
- Updated the README to not include the GitHub downloads badge since all binary releases are on crates.io [skip ci]
- Set all releases as manually triggered instead of automatic [skip ci]
- Updated dockerfile to no longer use the --disable-terminal-size-checks flag [skip ci]

## [0.1.5](https://github.com/Dark-Alex-17/managarr/compare/v0.1.4...v0.1.5) - 2024-11-03

### Other

- Added HTTPS support for all Servarrs

## [0.1.4](https://github.com/Dark-Alex-17/managarr/compare/v0.1.3...v0.1.4) - 2024-11-01

### Other

- Added the ability to fetch host configs and security configs to the CLI
- Updated README to be more clear about what features are supported [skip ci]

## [0.1.2](https://github.com/Dark-Alex-17/managarr/compare/v0.1.1...v0.1.2) - 2024-10-30

### Other

- Updated README to a more polished format for the alpha release

## [0.1.1](https://github.com/Dark-Alex-17/managarr/compare/v0.1.0...v0.1.1) - 2024-10-30

### Other

- Final dependency update
- Updated serde version to get minimal_versions job to pass
- Updated strum_macros dependency
