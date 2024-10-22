use clap::{Arg, Command};
use std::process::Command as SystemCommand;
use chrono::Utc;

fn main() {
    let install_id = "install";
    let install_package_id = "package";

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
          Command::new(rollback_id)
            .about("Rolls back the last transaction")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches(install_id) {
      let temp = String::new();
      let package = matches.get_one::<String>(install_package_id).unwrap_or(&temp);
      println!("Installing package: {}", package);
      install_package(package);
    }

    if let Some(matches) = matches.subcommand_matches(rollback_id) {
        println!("Rolling back last transaction");
        rollback_last_transaction();
    }
}

fn install_package(package: &str) {
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let pre_install_snapshot_name = &format!("{timestamp}-pre-install");

    create_snapshot("btrfs", "root", pre_install_snapshot_name);

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
      create_snapshot("btrfs", "root", &format!("{timestamp}-post-install"));
    } else {
        eprintln!("Failed to install package");
        rollback_to_snapshot("btrfs", "root", pre_install_snapshot_name);
    }
}

fn create_btrfs_snapshot(source: &str, dest: &str) {
    println!("create_btrfs_snapshot({source}, {dest})");
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

fn create_zfs_snapshot(pool: &str, dataset: &str, snapshot: &str) {
    println!("create_zfs_snapshot({pool}, {dataset}, {snapshot})");
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

fn rollback_btrfs_snapshot(current: &str, snapshot: &str) {
    println!("rollback_btrfs_snapshot({current}, {snapshot})");
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

fn rollback_zfs_snapshot(pool: &str, dataset: &str, snapshot: &str) {
    println!("rollback_zfs_snapshot({pool}, {dataset}, {snapshot})");
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

fn create_snapshot(fs_type: &str, source: &str, dest: &str) {
    println!("create_snapshot({fs_type}, {source}, {dest})");
    return;

    match fs_type {
        "btrfs" => create_btrfs_snapshot(source, dest),
        "zfs" => create_zfs_snapshot(source, dest, "snapshot"),
        _ => eprintln!("Unsupported filesystem type"),
    }
}

fn rollback_to_snapshot(fs_type: &str, current: &str, snapshot: &str) {
    println!("rollback_to_snapshot({fs_type}, {current}, {snapshot})");
    return;

    match fs_type {
        "btrfs" => rollback_btrfs_snapshot(current, snapshot),
        "zfs" => rollback_zfs_snapshot(current, snapshot, "snapshot"),
        _ => eprintln!("Unsupported filesystem type"),
    }
}

fn rollback_last_transaction() {
    println!("rollback_last_transaction()");
    return;

    // Implement rollback logic here
    println!("Rolling back last transaction");
}
