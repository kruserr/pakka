#![allow(unreachable_code)]
#![allow(unused_variables)]

use chrono::Utc;
use clap::{Arg, Command};
use std::fs;
use std::process;
use std::process::Command as SystemCommand;

pub fn cli_main() {
  let install_id = "install";
  let install_package_id = "package";

  let uninstall_id = "uninstall";
  let uninstall_package_id = "package";

  let rollback_id = "rollback";

  let matches = Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand(
      Command::new(install_id).about("Install a package").arg(
        Arg::new(install_package_id)
          .help("Package to install")
          .required(true)
          .index(1),
      ),
    )
    .subcommand(
      Command::new(uninstall_id).about("uninstall a package").arg(
        Arg::new(uninstall_package_id)
          .help("Package to uninstall")
          .required(true)
          .index(1),
      ),
    )
    .subcommand(
      Command::new(rollback_id).about("Rolls back the last transaction"),
    )
    .get_matches();

  let fs_type = &get_root_filesystem_type();
  if let Filesystem::Unsupported(_) = fs_type {
    eprintln!("Error: Unsupported filesystem type '{fs_type}'. Only 'btrfs' and 'zfs' are supported.");
    process::exit(1);
  }

  let package_manager = &get_package_manager();

  println!("{fs_type}");
  println!("{package_manager}");

  if let Some(matches) = matches.subcommand_matches(install_id) {
    let temp = String::new();
    let package =
      matches.get_one::<String>(install_package_id).unwrap_or(&temp);
    package_manager.install_package(package, fs_type);
  }

  if let Some(matches) = matches.subcommand_matches(uninstall_id) {
    let temp = String::new();
    let package =
      matches.get_one::<String>(uninstall_package_id).unwrap_or(&temp);
    package_manager.uninstall_package(package, fs_type);
  }

  if let Some(matches) = matches.subcommand_matches(rollback_id) {
    rollback_last_transaction(fs_type);
  }
}

#[derive(Debug)]
pub enum Filesystem {
  Btrfs,
  Zfs,
  Unsupported(String),
}

impl std::fmt::Display for Filesystem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub fn get_root_filesystem_type() -> Filesystem {
  let mounts =
    fs::read_to_string("/proc/mounts").expect("Failed to read /proc/mounts");
  for line in mounts.lines() {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() > 2 && fields[1] == "/" {
      return match fields[2] {
        "btrfs" => Filesystem::Btrfs,
        "zfs" => Filesystem::Zfs,
        other => Filesystem::Unsupported(other.to_string()),
      };
    }
  }

  eprintln!("Error: Failed to determine filesystem type");
  process::exit(1);
}

pub trait PackageManager {
  fn get_name(&self) -> &str;
  fn install_package(&self, package: &str, fs_type: &Filesystem);
  fn uninstall_package(&self, package: &str, fs_type: &Filesystem);
}

impl std::fmt::Debug for dyn PackageManager {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl std::fmt::Display for dyn PackageManager {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
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

  for (cmd, manager) in &managers {
    if SystemCommand::new("which").arg(cmd).output().is_ok() {
      match OperatingSystemFamily::from_str(&distro_family.to_string()) {
        Some(OperatingSystemFamily::Debian)
          if *cmd == AptPackageManager::NAME =>
        {
          return Some(*manager)
        }
        Some(OperatingSystemFamily::RedHat)
          if *cmd == DnfPackageManager::NAME =>
        {
          return Some(*manager)
        }
        Some(OperatingSystemFamily::Arch)
          if *cmd == PacmanPackageManager::NAME =>
        {
          return Some(*manager)
        }
        Some(OperatingSystemFamily::Suse)
          if *cmd == ZypperPackageManager::NAME =>
        {
          return Some(*manager)
        }
        Some(OperatingSystemFamily::NixOS)
          if *cmd == NixPackageManager::NAME =>
        {
          return Some(*manager)
        }
        Some(OperatingSystemFamily::MacOs)
          if *cmd == BrewPackageManager::NAME =>
        {
          return Some(*manager)
        }
        Some(OperatingSystemFamily::Windows)
          if *cmd == ScoopPackageManager::NAME =>
        {
          return Some(*manager)
        }
        _ => {}
      }
    }
  }

  // Fallback to the first detected package manager if no match with distro
  // family
  for (cmd, manager) in managers {
    if SystemCommand::new("which").arg(cmd).output().is_ok() {
      return Some(manager);
    }
  }

  None
}

#[derive(Debug)]
pub enum OperatingSystemFamily {
  Debian,
  RedHat,
  Arch,
  Suse,
  NixOS,
  MacOs,
  Windows,
  Unknown,
}
impl OperatingSystemFamily {
  fn from_str(s: &str) -> Option<OperatingSystemFamily> {
    match s {
      "debian" => Some(OperatingSystemFamily::Debian),
      "rhel" | "fedora" => Some(OperatingSystemFamily::RedHat),
      "arch" => Some(OperatingSystemFamily::Arch),
      "suse" => Some(OperatingSystemFamily::Suse),
      "nixos" => Some(OperatingSystemFamily::NixOS),
      "macos" => Some(OperatingSystemFamily::MacOs),
      "windows" => Some(OperatingSystemFamily::Windows),
      _ => None,
    }
  }
}

impl std::fmt::Display for OperatingSystemFamily {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub fn get_distro_family() -> OperatingSystemFamily {
  if cfg!(target_os = "macos") {
    return OperatingSystemFamily::MacOs;
  } else if cfg!(target_os = "windows") {
    return OperatingSystemFamily::Windows;
  }

  let os_release = fs::read_to_string("/etc/os-release")
    .expect("Failed to read /etc/os-release");
  for line in os_release.lines() {
    if line.starts_with("ID_LIKE=") {
      return OperatingSystemFamily::from_str(
        line.trim_start_matches("ID_LIKE=").replace("\"", "").as_str(),
      )
      .unwrap_or(OperatingSystemFamily::Unknown);
    }
  }
  for line in os_release.lines() {
    if line.starts_with("ID=") {
      return OperatingSystemFamily::from_str(
        line.trim_start_matches("ID=").replace("\"", "").as_str(),
      )
      .unwrap_or(OperatingSystemFamily::Unknown);
    }
  }
  eprintln!("Error: Failed to determine Linux distribution family");
  process::exit(1);
}

pub fn get_package_manager() -> &'static dyn PackageManager {
  detect_package_manager().unwrap_or_else(|| {
    eprintln!("Error: No supported package manager found.");
    process::exit(1);
  })
}

pub struct AptPackageManager;
impl AptPackageManager {
  const NAME: &str = "apt-get";
}
impl PackageManager for AptPackageManager {
  fn get_name(&self) -> &str {
    AptPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    println!("apt_install_package({package})");

    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let pre_install_snapshot_name = &format!("{timestamp}-pre-install");

    create_snapshot(fs_type, "root", pre_install_snapshot_name);

    let update_output = SystemCommand::new("apt-get")
      .arg("update")
      .output()
      .expect("Failed to update package list");

    if !update_output.stdout.is_empty() {
      print!("{}", String::from_utf8_lossy(&update_output.stdout));
    }

    if !update_output.status.success() {
      eprint!("{}", String::from_utf8_lossy(&update_output.stderr));
      return;
    }

    let output = SystemCommand::new("apt-get")
      .arg("install")
      .arg("-y")
      .arg(package)
      .output()
      .expect("Failed to install package");

    if !output.stdout.is_empty() {
      print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
      eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if output.status.success() {
      println!("Package installed successfully");
      create_snapshot(fs_type, "root", &format!("{timestamp}-post-install"));
    } else {
      eprintln!("Failed to install package");
      rollback_to_snapshot(fs_type, "root", pre_install_snapshot_name);
    }
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    println!("apt_uninstall_package({package})");

    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let pre_uninstall_snapshot_name = &format!("{timestamp}-pre-uninstall");

    create_snapshot(fs_type, "root", pre_uninstall_snapshot_name);

    let output = SystemCommand::new("apt-get")
      .arg("remove")
      .arg("-y")
      .arg(package)
      .output()
      .expect("Failed to uninstall package");

    if !output.stdout.is_empty() {
      print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
      eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if output.status.success() {
      println!("Package uninstalled successfully");
      create_snapshot(fs_type, "root", &format!("{timestamp}-post-uninstall"));
    } else {
      eprintln!("Failed to uninstall package");
      rollback_to_snapshot(fs_type, "root", pre_uninstall_snapshot_name);
    }
  }
}

pub struct DnfPackageManager;
impl DnfPackageManager {
  const NAME: &str = "dnf";
}
impl PackageManager for DnfPackageManager {
  fn get_name(&self) -> &str {
    DnfPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }
}

pub struct PacmanPackageManager;
impl PacmanPackageManager {
  const NAME: &str = "pacman";
}
impl PackageManager for PacmanPackageManager {
  fn get_name(&self) -> &str {
    PacmanPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }
}

pub struct ZypperPackageManager;
impl ZypperPackageManager {
  const NAME: &str = "zypper";
}
impl PackageManager for ZypperPackageManager {
  fn get_name(&self) -> &str {
    ZypperPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }
}

pub struct NixPackageManager;
impl NixPackageManager {
  const NAME: &str = "nix";
}
impl PackageManager for NixPackageManager {
  fn get_name(&self) -> &str {
    NixPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }
}

pub struct BrewPackageManager;
impl BrewPackageManager {
  const NAME: &str = "brew";
}
impl PackageManager for BrewPackageManager {
  fn get_name(&self) -> &str {
    BrewPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }
}

pub struct ScoopPackageManager;
impl ScoopPackageManager {
  const NAME: &str = "scoop";
}
impl PackageManager for ScoopPackageManager {
  fn get_name(&self) -> &str {
    ScoopPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }
}

pub fn btrfs_create_snapshot(source: &str, dest: &str) {
  println!("btrfs_create_snapshot({source}, {dest})");
  return;

  let output = SystemCommand::new("btrfs")
    .arg("subvolume")
    .arg("snapshot")
    .arg(source)
    .arg(dest)
    .output()
    .expect("Failed to create Btrfs snapshot");

  if output.status.success() {
    println!("Btrfs snapshot created successfully");
  } else {
    eprintln!("Failed to create Btrfs snapshot");
  }
}

pub fn zfs_create_snapshot(pool: &str, dataset: &str, snapshot: &str) {
  println!("zfs_create_snapshot({pool}, {dataset}, {snapshot})");
  return;

  let snapshot_name = format!("{}@{}", dataset, snapshot);
  let output = SystemCommand::new("zfs")
    .arg("snapshot")
    .arg(&snapshot_name)
    .output()
    .expect("Failed to create ZFS snapshot");

  if output.status.success() {
    println!("ZFS snapshot created successfully");
  } else {
    eprintln!("Failed to create ZFS snapshot");
  }
}

pub fn btrfs_rollback_snapshot(current: &str, snapshot: &str) {
  println!("btrfs_rollback_snapshot({current}, {snapshot})");
  return;

  let delete_output = SystemCommand::new("btrfs")
    .arg("subvolume")
    .arg("delete")
    .arg(current)
    .output()
    .expect("Failed to delete current Btrfs subvolume");

  if delete_output.status.success() {
    let create_output = SystemCommand::new("btrfs")
      .arg("subvolume")
      .arg("snapshot")
      .arg(snapshot)
      .arg(current)
      .output()
      .expect("Failed to create Btrfs snapshot");

    if create_output.status.success() {
      println!("Btrfs rollback successful");
    } else {
      eprintln!("Failed to create Btrfs snapshot for rollback");
    }
  } else {
    eprintln!("Failed to delete current Btrfs subvolume");
  }
}

pub fn zfs_rollback_snapshot(pool: &str, dataset: &str, snapshot: &str) {
  println!("zfs_rollback_snapshot({pool}, {dataset}, {snapshot})");
  return;

  let snapshot_name = format!("{}@{}", dataset, snapshot);
  let output = SystemCommand::new("zfs")
    .arg("rollback")
    .arg(&snapshot_name)
    .output()
    .expect("Failed to rollback ZFS snapshot");

  if output.status.success() {
    println!("ZFS rollback successful");
  } else {
    eprintln!("Failed to rollback ZFS snapshot");
  }
}

pub fn create_snapshot(fs_type: &Filesystem, source: &str, dest: &str) {
  println!("create_snapshot({fs_type}, {source}, {dest})");
  return;

  match fs_type {
    Filesystem::Btrfs => btrfs_create_snapshot(source, dest),
    Filesystem::Zfs => zfs_create_snapshot(source, dest, "snapshot"),
    _ => eprintln!("Unsupported filesystem type"),
  }
}

pub fn rollback_to_snapshot(
  fs_type: &Filesystem,
  current: &str,
  snapshot: &str,
) {
  println!("rollback_to_snapshot({fs_type}, {current}, {snapshot})");
  return;

  match fs_type {
    Filesystem::Btrfs => btrfs_rollback_snapshot(current, snapshot),
    Filesystem::Zfs => zfs_rollback_snapshot(current, snapshot, "snapshot"),
    _ => eprintln!("Unsupported filesystem type"),
  }
}

pub fn rollback_last_transaction(fs_type: &Filesystem) {
  println!("rollback_last_transaction()");
  return;

  // Implement rollback logic here
  println!("Rolling back last transaction");
}
