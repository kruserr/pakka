use crate::{create_snapshot, rollback_to_snapshot, Filesystem};

use super::PackageManager;

pub struct ScoopPackageManager;
impl ScoopPackageManager {
  pub const NAME: &str = "scoop";
}
impl PackageManager for ScoopPackageManager {
  fn get_name(&self) -> &str {
    ScoopPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }
}
