use crate::{create_snapshot, rollback_to_snapshot, Filesystem};

use super::PackageManager;

pub struct BrewPackageManager;
impl BrewPackageManager {
  pub const NAME: &str = "brew";
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
