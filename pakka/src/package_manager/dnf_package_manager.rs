use crate::{create_snapshot, rollback_to_snapshot, Filesystem};

use super::PackageManager;

pub struct DnfPackageManager;
impl DnfPackageManager {
  pub const NAME: &str = "dnf";
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
