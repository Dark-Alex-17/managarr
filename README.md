# managarr - A TUI to manage your Servarrs
Managarr is a TUI to help you manage your HTPC (Home Theater PC). Built with 🤎 in Rust!

![library](screenshots/library.png)

## NOTE: Managarr is not yet stable (Pre-Alpha)
I'm regularly making changes to get Managarr to an alpha release. As such, I'm regularly refactoring the code to be cleaner
and more easily extensible. Until I get Managarr across the alpha-release finish line, this regular refactoring will make
contributions difficult. Thus, stability is not guaranteed (yet!).

This means that while all tests will pass, there may be certain menus or keymappings that are no-ops, or produce empty 
screens, or things of this sort.

Part of the alpha release plan is to add contribution guidelines, CI/CD, release pipelines, etc. so that
all future maintenance and additions can be handled easily. So unfortunately, until that happens, Managarr may contain
breaking changes and be slow to react to any PR's.

Progress for the alpha release can be followed on my [Wekan Board](https://wekan.alexjclarke.com/b/dHoGjBb44MHM9HSv4/managarr)
with all items tagged `Alpha`.

Thanks for your patience as I work to get this into a place ready for contributions and to make developer experiences as
pleasant as possible!

## What Servarrs are supported?

- ![radarr_logo](logos/radarr.png) [Radarr](https://wiki.servarr.com/radarr)
- ![sonarr_logo](logos/sonarr.png) [Sonarr](https://wiki.servarr.com/en/sonarr)
- ![readarr_logo](logos/readarr.png) [Readarr](https://wiki.servarr.com/en/readarr)
- ![lidarr_logo](logos/lidarr.png) [Lidarr](https://wiki.servarr.com/en/lidarr)
- ![prowlarr_logo](logos/prowlarr.png) [Prowlarr](https://wiki.servarr.com/en/prowlarr)
- ![whisparr_logo](logos/whisparr.png) [Whisparr](https://wiki.servarr.com/whisparr)
- ![bazarr_logo](logos/bazarr.png) [Bazarr](https://www.bazarr.media/)
- ![tautulli_logo](logos/tautulli.png) [Tautulli](https://tautulli.com/)

## Features

### Radarr

- [x] View your library, downloads, collections, and blocklist
- [x] View details of a specific movie including description, history, downloaded file info, or the credits
- [x] View details of any collection and the movies in them
- [x] Search your library or collections
- [x] Add movies to your library
- [x] Delete movies, downloads, and indexers
- [x] Trigger automatic searches for movies
- [x] Trigger refresh and disk scan for movies, downloads, and collections
- [x] Manually search for movies
- [x] Edit your movies, collections, and indexers
- [x] Manage your tags
- [x] Manage your root folders
- [x] Manage your blocklist
- [x] View and browse logs, tasks, events queues, and updates
- [x] Manually trigger scheduled tasks

### Sonarr
- [ ] Support for Sonarr

### Readarr

- [ ] Support for Readarr

### Lidarr

- [ ] Support for Lidarr

### Whisparr

- [ ] Support for Whisparr

### Bazarr

- [ ] Support for Bazarr

### Prowlarr

- [ ] Support for Prowlarr

### Tautulli

- [ ] Support for Tautulli

# Configuration
Managarr assumes reasonable defaults to connect to each service (i.e. Radarr is on localhost:7878),
but all servers will require you to input the API token.

The configuration file is located somewhere different for each OS

### Linux
```
$HOME/.config/managarr/config.yml
```

### Mac
```
$HOME/Library/Application Support/managarr/config.yml
```

### Windows
```
%APPDATA%/Roaming/managarr/config.yml
```

### Example Configuration:
```yaml
radarr:
  host: 127.0.0.1
  port: 7878
  api_token: someApiToken1234567890
sonarr:
  host: 127.0.0.1
  port: 8989
  api_token: someApiToken1234567890
readarr:
  host: 127.0.0.1
  port: 8787
  api_token: someApiToken1234567890
lidarr:
  host: 127.0.0.1
  port: 8686
  api_token: someApiToken1234567890
whisparr:
  host: 127.0.0.1
  port: 6969
  api_token: someApiToken1234567890
bazarr:
  host: 127.0.0.1
  port: 6767
  api_token: someApiToken1234567890
prowlarr:
  host: 127.0.0.1
  port: 9696
  api_token: someApiToken1234567890
tautulli:
  host: 127.0.0.1
  port: 8181
  api_token: someApiToken1234567890
```

## Screenshots

![library](screenshots/library.png)
![manual_search](screenshots/manual_search.png)
![logs](screenshots/logs.png)
![new_movie_search](screenshots/new_movie_search.png)
![add_new_movie](screenshots/add_new_movie.png)
![collection_details](screenshots/collection_details.png)
![indexers](screenshots/indexers.png)

## Dependencies
* [ratatui](https://github.com/tui-rs-revival/ratatui)
* [crossterm](https://github.com/crossterm-rs/crossterm)
* [clap](https://github.com/clap-rs/clap)
* [tokio](https://github.com/tokio-rs/tokio)
* [serde](https://github.com/serde-rs/serde)
* [reqwest](https://github.com/seanmonstar/reqwest)

## Servarr Requirements
* [Radarr >= 5.3.6.8612](https://radarr.video/docs/api/)
* [Sonarr >= v3](https://sonarr.tv/docs/api/)
* [Readarr v1](https://readarr.com/docs/api/)
* [Lidarr v1](https://lidarr.audio/docs/api/)
* [Whisparr >= v3](https://whisparr.com/docs/api/)
* [Prowlarr v1](https://prowlarr.com/docs/api/)
* [Bazarr v1.1.4](http://localhost:6767/api)
* [Tautulli >= v2](https://github.com/Tautulli/Tautulli/wiki/Tautulli-API-Reference)

## Creator
* [Alex Clarke](https://github.com/Dark-Alex-17)