use clap::{Arg, Command};
use std::process::Command as SystemCommand;
use chrono::Utc;
use std::process;
use std::fs;

fn main() {
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
          Command::new(install_id)
            .about("Install a package")
            .arg(
              Arg::new(install_package_id)
                .help("Package to install")
                .required(true)
                .index(1),
            )
        )
        .subcommand(
          Command::new(uninstall_id)
            .about("uninstall a package")
            .arg(
              Arg::new(uninstall_package_id)
                .help("Package to uninstall")
                .required(true)
                .index(1),
            )
        )
        .subcommand(
          Command::new(rollback_id)
            .about("Rolls back the last transaction")
        )
        .get_matches();

    let fs_type = &get_root_filesystem_type();
    match fs_type {
        Filesystem::Unsupported(_) => {
          eprintln!("Error: Unsupported filesystem type '{fs_type}'. Only 'btrfs' and 'zfs' are supported.");
          process::exit(1);
        },
        _ => {}
    }

    println!("{fs_type}");
    
    if let Some(matches) = matches.subcommand_matches(install_id) {
      let temp = String::new();
      let package = matches.get_one::<String>(install_package_id).unwrap_or(&temp);
      apt_install_package(package, fs_type);
    }

    if let Some(matches) = matches.subcommand_matches(uninstall_id) {
      let temp = String::new();
      let package = matches.get_one::<String>(uninstall_package_id).unwrap_or(&temp);
      apt_uninstall_package(package, fs_type);
    }

    if let Some(matches) = matches.subcommand_matches(rollback_id) {
        rollback_last_transaction(fs_type);
    }
}

#[derive(Debug)]
enum Filesystem {
  Btrfs,
  Zfs,
  Unsupported(String),
}

impl std::fmt::Display for Filesystem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

fn get_root_filesystem_type() -> Filesystem {
  let mounts = fs::read_to_string("/proc/mounts").expect("Failed to read /proc/mounts");
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

fn apt_install_package(package: &str, fs_type: &Filesystem) {
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

fn apt_uninstall_package(package: &str, fs_type: &Filesystem) {
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

fn btrfs_create_snapshot(source: &str, dest: &str) {
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

fn zfs_create_snapshot(pool: &str, dataset: &str, snapshot: &str) {
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

fn btrfs_rollback_snapshot(current: &str, snapshot: &str) {
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

fn zfs_rollback_snapshot(pool: &str, dataset: &str, snapshot: &str) {
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

fn create_snapshot(fs_type: &Filesystem, source: &str, dest: &str) {
    println!("create_snapshot({fs_type}, {source}, {dest})");
    return;

    match fs_type {
        Filesystem::Btrfs => btrfs_create_snapshot(source, dest),
        Filesystem::Zfs => zfs_create_snapshot(source, dest, "snapshot"),
        _ => eprintln!("Unsupported filesystem type"),
    }
}

fn rollback_to_snapshot(fs_type: &Filesystem, current: &str, snapshot: &str) {
    println!("rollback_to_snapshot({fs_type}, {current}, {snapshot})");
    return;

    match fs_type {
        Filesystem::Btrfs => btrfs_rollback_snapshot(current, snapshot),
        Filesystem::Zfs => zfs_rollback_snapshot(current, snapshot, "snapshot"),
        _ => eprintln!("Unsupported filesystem type"),
    }
}

fn rollback_last_transaction(fs_type: &Filesystem) {
    println!("rollback_last_transaction()");
    return;

    // Implement rollback logic here
    println!("Rolling back last transaction");
}
