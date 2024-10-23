- [ ] 

# v0.1
- [ ] meta package manager
- [ ] make it a cross platform package manager, that just uses what is available, but has a minimum set of requirements, e.g. should atleast have a compatible package manger scoop on windows nad homebrew on mac then has the same interface for every os
- [ ] use a underlying cow filesystem to implement transactional package managment, if not available then it is just not transactional
- [ ] if installing on windows run scoop install bootstrap

# v0.2
- [ ] record all package install and unistall in log structured database with event sourcing, we can then say excatly which packages were installed at a given point in time
- [ ] sync to database, e.g. calc diff between which packages should be on the system and install teh ones that are missing, we will wait with removing packagfes since that can be alot harder, only allow removing packages manually not automatically sync tr db
- [ ] export and import single file sync db

- [ ] iced gui
- [ ] we don't want 100% reproducibility we just want to be able to install same packages as on another machine and we want a simple, clean and cross platform solution that a regular windows user can use with a gui but also a cli for me :)

# v0.3
- [ ] map apt names to scoop names with binary linked static hashmap, this also makes it possible to e.g. write install build-essential on pacman and scoop
- [ ] the map should be implemented like this: base package name: package manager key: value
- [ ] as long as the package is installed through here, it will be recorded, also record the command that was executed e.g. if you were on windows it will record scoop install. then we create a pakcage availability database for each major distribution, and when we do a sync operation we can check the package availability db and map the name to a distro name and also check if the packages are available on the backing package manager, and if not we can throw a nice error that displays which packages are not available and then we can have a force options to allow the user to sync anyways

# Future
- [ ] allow multiple versions of the same package to run in parallel, and then have applications use the version that they have requested, but use e.g. the latest version for the path, or set a path version manually
- [ ] sync share functionallity only export latest packages, or have modules / applications that only install a limited number of packages for a given application
- [ ] in later versions have a optional decleartive toml file to install packages with multiple package locations, e.g. system package manager, git revision, local package and use name alias like and version locking like cargo, basically cargo for the system
- [ ] package reviews and package audits with needed permissions, used syscalls, capabilities optimally provided  by the devs and the n checked by software auditors
- [ ] system to scan installed packages for cves and notify the end user to prompt updates, but only when a critical cve is discovered
- [ ] flag to set deafult package manager e.g. apt such that if there are conflicts in package name mapping it will deault to apt or your setting
- [ ] in later version automatically install nix and run only on nix, install nix in wsl on windows and write tooling to auto install wsl
- [ ] option to limit strictly to open source software
