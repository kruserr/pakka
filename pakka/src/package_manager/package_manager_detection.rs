use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs};

use super::apt_package_manager::AptPackageManager;
use super::brew_package_manager::BrewPackageManager;
use super::dnf_package_manager::DnfPackageManager;
use super::nix_package_manager::NixPackageManager;
use super::os_detection::{get_distro_family, OperatingSystemFamily};
use super::pacman_package_manager::PacmanPackageManager;
use super::scoop_package_manager::ScoopPackageManager;
use super::zypper_package_manager::ZypperPackageManager;
use super::PackageManager;

pub fn which(binary: &str) -> Option<PathBuf> {
  if let Ok(paths) = env::var("PATH") {
    for path in env::split_paths(&paths) {
      let full_path = path.join(binary);
      if full_path.is_file() {
        return Some(full_path);
      }
    }
  }
  return None;
}

pub fn detect_package_manager() -> Option<&'static dyn PackageManager> {
  let managers: Vec<(&str, &'static dyn PackageManager)> = vec![
    (AptPackageManager::NAME, &AptPackageManager),
    (DnfPackageManager::NAME, &DnfPackageManager),
    (PacmanPackageManager::NAME, &PacmanPackageManager),
    (ZypperPackageManager::NAME, &ZypperPackageManager),
    (NixPackageManager::NAME, &NixPackageManager),
    (BrewPackageManager::NAME, &BrewPackageManager),
    (ScoopPackageManager::NAME, &ScoopPackageManager),
  ];

  let distro_family = get_distro_family();

  // for (cmd, manager) in &managers {
  //   if which(cmd).is_some() {
  //     // TODO fix this, it just returns Err()
  //     match OperatingSystemFamily::from_str(&distro_family.to_string()) {
  //       Ok(OperatingSystemFamily::Debian)
  //         if *cmd == AptPackageManager::NAME =>
  //       {
  //         return Some(*manager)
  //       }
  //       Ok(OperatingSystemFamily::RedHat)
  //         if *cmd == DnfPackageManager::NAME =>
  //       {
  //         return Some(*manager)
  //       }
  //       Ok(OperatingSystemFamily::Arch)
  //         if *cmd == PacmanPackageManager::NAME =>
  //       {
  //         return Some(*manager)
  //       }
  //       Ok(OperatingSystemFamily::Suse)
  //         if *cmd == ZypperPackageManager::NAME =>
  //       {
  //         return Some(*manager)
  //       }
  //       Ok(OperatingSystemFamily::NixOS) if *cmd == NixPackageManager::NAME
  // => {         return Some(*manager)
  //       }
  //       Ok(OperatingSystemFamily::MacOs)
  //         if *cmd == BrewPackageManager::NAME =>
  //       {
  //         return Some(*manager)
  //       }
  //       Ok(OperatingSystemFamily::Windows)
  //         if *cmd == ScoopPackageManager::NAME =>
  //       {
  //         return Some(*manager)
  //       }
  //       _ => {}
  //     }
  //   }
  // }

  // Fallback to the first detected package manager if no match with distro
  // family
  for (cmd, manager) in managers {
    println!("{:?}", which(cmd));

    if which(cmd).is_some() {
      return Some(manager);
    }
  }

  None
}

pub fn get_package_manager() -> &'static dyn PackageManager {
  detect_package_manager().unwrap_or_else(|| {
    eprintln!("Error: No supported package manager found.");
    std::process::exit(1);
  })
}
