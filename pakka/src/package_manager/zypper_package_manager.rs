use crate::{create_snapshot, rollback_to_snapshot, Filesystem};

use super::PackageManager;

pub struct ZypperPackageManager;
impl ZypperPackageManager {
  pub const NAME: &str = "zypper";
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
