<p align="center">
  <a href="https://github.com/kruserr/rustic-reader" target="_blank">
    <img width="300" src="https://raw.githubusercontent.com/kruserr/rustic-reader/main/assets/logo/logo.svg">
  </a>
</p>

# pakka
A cross platform meta package manager with auto snapshotting file system based transactions

## Overview
The goal of this project is to improve the package manager user experience, and introduce a new type of audience to using package managers.

We are building a layer of abstraction above the system package manager, and making it such that you only need to learn a single package manager to use them all, although with a reduced feature set where applicable, such as with nix.

This project is especially targeted towards those who are stuck on a ***dows based fleet, but need a streamlined package manager with streamlined critical cve update notifications for installed software with opt out invasive notifications that force the end user to update.

## Features
- CLI client
  - install and uninstall packages

## Quick start guide
### Install the CLI client
```sh
cargo install --locked pakka
pakka --help
```

For further install instructions read the [Getting started page](docs/pages/getting-started.md)

## Documentation
Visit the [Documentation](docs/README.md)

## Roadmap
- [ ] if snapshotting filesystem is available it will automatically create a snapshot pre and post package management
- [ ] upgrade packages
- [ ] search packages
- [ ] list installed packages
- [ ] fully cross platform
- [ ] log structured event sourcing database to track installed packages over time
- [ ] export and import database to install the same packages on another machine
- [ ] package name mapper
- [ ] iced gui
- [ ] scan installed packages for cves
- [ ] option to limit strictly to open source software
<!-- - [ ]  -->
