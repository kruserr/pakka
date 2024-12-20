#![allow(unreachable_code)]
#![allow(unused_variables)]

pub mod file_system;
use file_system::{
  create_snapshot, get_root_filesystem_type, rollback_last_transaction,
  rollback_to_snapshot, Filesystem,
};

pub mod package_manager;
use package_manager::package_manager_detection::get_package_manager;

pub mod database;
use database::event_sourcing_database;
use event_sourcing_database::{Event, EventSourcingDatabase, EventType};

use clap::{Arg, Command};
use std::{
  fs::File,
  io::{BufRead, BufReader},
  process,
};

pub fn cli_main() {
  let install_id = "install";
  let install_package_id = "package";

  let uninstall_id = "uninstall";
  let uninstall_package_id = "package";

  let rollback_id = "rollback";

  let list_id = "list";
  let export_id = "export";

  let history_id = "history";
  let diff_id = "diff";
  let from_date_arg = "from";
  let to_date_arg = "to";

  let import_id = "import";
  let import_file_arg = "file";

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
      Command::new(uninstall_id).about("Uninstall a package").arg(
        Arg::new(uninstall_package_id)
          .help("Package to uninstall")
          .required(true)
          .index(1),
      ),
    )
    .subcommand(
      Command::new(rollback_id).about("Roll back the last transaction"),
    )
    .subcommand(Command::new(list_id).about("List all installed packages"))
    .subcommand(Command::new(export_id).about("Export installed packages"))
    .subcommand(
      Command::new(import_id).about("Import packages from a file").arg(
        Arg::new(import_file_arg)
          .help("File to import packages from")
          .required(true)
          .index(1),
      ),
    )
    .subcommand(
      Command::new(history_id)
        .about("Show history of installed and uninstalled packages")
        .arg(
          Arg::new("date")
            .help("Filter events by date (e.g., 2023-10-15)")
            .required(false),
        ),
    )
    .subcommand(
      Command::new(diff_id)
        .about("Show differences in packages between two dates")
        .arg(
          Arg::new(from_date_arg)
            .help("Start date (e.g., 2023-10-01)")
            .required(true),
        )
        .arg(
          Arg::new(to_date_arg)
            .help("End date (e.g., 2023-10-15)")
            .required(true),
        ),
    )
    .get_matches();

  let fs_type = &get_root_filesystem_type();
  if let Filesystem::Unsupported(_) = fs_type {
    println!("W: Snapshotting disabled, '{fs_type}' filesystem type");
  }

  let package_manager = &get_package_manager();

  let db = EventSourcingDatabase::instance();

  if let Some(matches) = matches.subcommand_matches(install_id) {
    let package = matches
      .get_one::<String>(install_package_id)
      .expect("Package name is required");
    package_manager.install_package(package, fs_type);

    // let event = Event::new(EventType::Install, package,
    // package_manager.get_name()); db.log_event(&event);
  } else if let Some(matches) = matches.subcommand_matches(uninstall_id) {
    let package = matches
      .get_one::<String>(uninstall_package_id)
      .expect("Package name is required");
    package_manager.uninstall_package(package, fs_type);

    // let event = Event::new(EventType::Uninstall, package,
    // package_manager.get_name()); db.log_event(&event);
  } else if matches.subcommand_matches(list_id).is_some() {
    match db.get_installed_packages() {
      Ok(packages) => {
        if packages.is_empty() {
          println!("No packages are currently installed.");
        } else {
          println!("Installed packages:");
          for package in packages {
            println!("- {}", package);
          }
        }
      }
      Err(e) => eprintln!("Error listing packages: {}", e),
    }
  } else if matches.subcommand_matches(export_id).is_some() {
    let export_file = "exported_packages.txt";
    match db.export_installed_packages(export_file) {
      Ok(_) => println!("Exported installed packages to {}", export_file),
      Err(e) => eprintln!("Error exporting packages: {}", e),
    }
  } else if let Some(matches) = matches.subcommand_matches(import_id) {
    let import_file = matches
      .get_one::<String>(import_file_arg)
      .expect("Import file is required");
    match import_packages(import_file, fs_type) {
      Ok(_) => println!("Import successful."),
      Err(e) => eprintln!("Error importing packages: {}", e),
    }
  } else if let Some(matches) = matches.subcommand_matches(history_id) {
    let date_filter = matches.get_one::<String>("date").cloned();
    db.show_history(date_filter);
  } else if let Some(matches) = matches.subcommand_matches(diff_id) {
    let from_date =
      matches.get_one::<String>(from_date_arg).expect("From date is required");
    let to_date =
      matches.get_one::<String>(to_date_arg).expect("To date is required");
    db.show_diff(from_date, to_date);
  } else {
    eprintln!("No valid subcommand was provided.");
    process::exit(1);
  }
}

fn import_packages(
  import_file: &str,
  fs_type: &Filesystem,
) -> Result<(), Box<dyn std::error::Error>> {
  let file = File::open(import_file)?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line?;
    let parts: Vec<&str> = line.trim().split('\t').collect();
    if parts.len() != 2 {
      eprintln!("Invalid line format: {}", line);
      continue;
    }
    let package = parts[0];
    let package_manager_name = parts[1];

    let package_manager = &get_package_manager();
    package_manager.install_package(package, fs_type);
  }

  Ok(())
}
