use crate::{create_snapshot, rollback_to_snapshot, Filesystem};

use super::PackageManager;

pub struct NixPackageManager;
impl NixPackageManager {
  pub const NAME: &str = "nix";
}
impl PackageManager for NixPackageManager {
  fn get_name(&self) -> &str {
    NixPackageManager::NAME
  }

  fn install_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }

  fn uninstall_package(&self, package: &str, fs_type: &Filesystem) {
    todo!()
  }
}
