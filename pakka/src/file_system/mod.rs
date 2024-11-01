pub mod zfs_file_system;
use zfs_file_system::{zfs_create_snapshot, zfs_rollback_snapshot};

pub mod btrfs_file_system;
use btrfs_file_system::{btrfs_create_snapshot, btrfs_rollback_snapshot};

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
  if cfg!(target_os = "macos") {
    return Filesystem::Unsupported("macos".to_string());
  } else if cfg!(target_os = "windows") {
    return Filesystem::Unsupported("windows".to_string());
  }

  let mounts = std::fs::read_to_string("/proc/mounts")
    .expect("Failed to read /proc/mounts");

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

  return Filesystem::Unsupported("unknown".to_string());
}

pub fn create_snapshot(fs_type: &Filesystem, source: &str, dest: &str) {
  if let Filesystem::Unsupported(_) = fs_type {
    return;
  }

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
  if let Filesystem::Unsupported(_) = fs_type {
    return;
  }

  println!("rollback_to_snapshot({fs_type}, {current}, {snapshot})");
  return;

  match fs_type {
    Filesystem::Btrfs => btrfs_rollback_snapshot(current, snapshot),
    Filesystem::Zfs => zfs_rollback_snapshot(current, snapshot, "snapshot"),
    _ => eprintln!("Unsupported filesystem type"),
  }
}

pub fn rollback_last_transaction(fs_type: &Filesystem) {
  if let Filesystem::Unsupported(_) = fs_type {
    return;
  }

  println!("rollback_last_transaction()");
  return;

  // Implement rollback logic here
  println!("Rolling back last transaction");
}
