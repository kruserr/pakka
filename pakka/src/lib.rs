#![allow(unreachable_code)]
#![allow(unused_variables)]

pub mod file_system;
use file_system::{
  create_snapshot, get_root_filesystem_type, rollback_last_transaction,
  rollback_to_snapshot, Filesystem,
};

pub mod package_manager;
use package_manager::package_manager_detection::get_package_manager;

use chrono::Utc;
use clap::{Arg, Command};
use std::fs;
use std::process;
use std::process::Command as SystemCommand;
use std::str::FromStr;

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
