use crate::{
  create_snapshot,
  database::event_sourcing_database::{
    Event, EventSourcingDatabase, EventType,
  },
  rollback_to_snapshot, Filesystem,
};

use super::PackageManager;

pub struct AptPackageManager;
impl AptPackageManager {
  pub const NAME: &str = "apt-get";
}
impl PackageManager for AptPackageManager {
  fn get_name(&self) -> &str {
    AptPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    println!("apt_install_package({package})");

    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let pre_install_snapshot_name = &format!("{timestamp}-pre-install");

    create_snapshot(fs_type, "root", pre_install_snapshot_name);

    let update_output = std::process::Command::new("apt-get")
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

    let output = std::process::Command::new("apt-get")
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

      let event = Event::new(EventType::Install, package, self.get_name());
      let db = EventSourcingDatabase::instance();
      db.log_event(&event);
    } else {
      eprintln!("Failed to install package");
      rollback_to_snapshot(fs_type, "root", pre_install_snapshot_name);
    }
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    println!("apt_uninstall_package({package})");

    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let pre_uninstall_snapshot_name = &format!("{timestamp}-pre-uninstall");

    create_snapshot(fs_type, "root", pre_uninstall_snapshot_name);

    let output = std::process::Command::new("apt-get")
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

      let event = Event::new(EventType::Uninstall, package, self.get_name());
      let db = EventSourcingDatabase::instance();
      db.log_event(&event);
    } else {
      eprintln!("Failed to uninstall package");
      rollback_to_snapshot(fs_type, "root", pre_uninstall_snapshot_name);
    }
  }
}
