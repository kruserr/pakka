use crate::Filesystem;

pub mod apt_package_manager;
pub mod brew_package_manager;
pub mod dnf_package_manager;
pub mod nix_package_manager;
pub mod pacman_package_manager;
pub mod scoop_package_manager;
pub mod zypper_package_manager;

pub trait PackageManager {
  fn get_name(&self) -> &str;
  fn install_package(&self, package: &str, fs_type: &Filesystem);
  fn uninstall_package(&self, package: &str, fs_type: &Filesystem);
}

impl std::fmt::Debug for dyn PackageManager {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_name())
  }
}

impl std::fmt::Display for dyn PackageManager {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
