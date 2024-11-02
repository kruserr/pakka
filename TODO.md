- [ ] 

# 0.1
- [x] meta package manager
- [x] make it a cross platform package manager, that just uses what is available, but has a minimum set of requirements, e.g. should at least have a compatible package manger scoop on windows nad homebrew on mac then has the same interface for every os
- [x] record all package install and uninstall in log structured database with event sourcing, we can then say exactly which packages were installed at a given point in time
- [x] export and import single file sync db
- [x] basic iced gui
- [ ] update
- [ ] upgrade
- [ ] full-upgrade
- [ ] sys-list
- [ ] store event_log.jsonl in e.g. ~/.config/pakka/event_log.jsonl
- [ ] template method pattern for PackageManager trait
- [ ] use an underlying cow filesystem to implement transactional package management, if not available then it is just not transactional

# 0.2
- [ ] pakka-gui feature parity with pakka cli
- [ ] windows scoop bootstrap command that is prompted to be ran the first time the user opens pakka-gui on windows and scoop is not in path also maybe prompt to restart the app
- [ ] make a ~/.config/pakka/.env config file
  - [ ] set default package manager
  - [ ] disable background service for pakka-gui
  - [ ] disable background update checking for pakka-gui
  - [ ] disable intrusive update notifications for pakka-gui
  - [ ] settings menu in pakka-gui that modifies the .env file directly and changes the setting in memory
- [ ] pakka-gui windows scoop interactive popup when upgrading to close runninng applications that are about to be upgraded, if no then only non running apps are upgraded
- [ ] pakka-gui daemon or background service to check for updates in the background and notify when new updates are available, with an intrusive option such that you can not hide it after having hidden it for 3 times, detect presentation mode, or if an application is fullscreen or if a powerpoint is running, then only show it 5 min after presentation is done or when tv is unplugged etc.
- [ ] pakka-gui cleaner material 3 based ui desgin

# 0.3
- [ ] map apt names to scoop names with binary linked static hashmap, this also makes it possible to e.g. write install build-essential on pacman and scoop
- [ ] the map should be implemented like this: base package name: package manager key: value

# v0.4
- [ ] sync to database, e.g. calc diff between which packages should be on the system and install the ones that are missing, we will wait with removing packages since that can be a lot harder, only allow removing packages manually not automatically sync tr db
- [ ] as long as the package is installed through here, it will be recorded, also record the command that was executed e.g. if you were on windows it will record scoop install. then we create a package availability database for each major distribution, and when we do a sync operation we can check the package availability db and map the name to a distro name and also check if the packages are available on the backing package manager, and if not we can throw a nice error that displays which packages are not available and then we can have a force options to allow the user to sync anyways

# 0.5
- [ ] system to scan installed packages for cves and notify the end user to prompt updates, but only when a critical cve is discovered
- [ ] option to limit strictly to open source software or white list packages
- [ ] maintain an official package whitelist

# Future
- [ ] allow multiple versions of the same package to run in parallel, and then have applications use the version that they have requested, but use e.g. the latest version for the path, or set a path version manually
- [ ] sync share functionality only export latest packages, or have modules / applications that only install a limited number of packages for a given application
- [ ] in later versions have an optional declarative toml file to install packages with multiple package locations, e.g. system package manager, git revision, local package and use name alias and version locking like cargo, basically cargo for the system
- [ ] package reviews and package audits with needed permissions, used syscalls, capabilities optimally provided  by the devs and then checked by software auditors
- [ ] flag to set default package manager e.g. apt such that if there are conflicts in package name mapping it will default to apt or your setting
- [ ] in later version automatically install nix and run only on nix, install nix in wsl on windows and write tooling to auto install wsl
