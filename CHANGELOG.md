# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
