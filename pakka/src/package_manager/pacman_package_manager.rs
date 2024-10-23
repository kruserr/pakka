use crate::{create_snapshot, rollback_to_snapshot, Filesystem};

use super::PackageManager;

pub struct PacmanPackageManager;
impl PacmanPackageManager {
  pub const NAME: &str = "pacman";
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
