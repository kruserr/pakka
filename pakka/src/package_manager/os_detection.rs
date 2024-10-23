use std::str::FromStr;

#[derive(Debug)]
pub enum OperatingSystemFamily {
  Debian,
  RedHat,
  Arch,
  Suse,
  NixOS,
  MacOs,
  Windows,
  Unknown,
}
impl std::str::FromStr for OperatingSystemFamily {
  type Err = ();

  fn from_str(s: &str) -> Result<OperatingSystemFamily, Self::Err> {
    match s {
      "debian" => Ok(OperatingSystemFamily::Debian),
      "rhel" | "fedora" => Ok(OperatingSystemFamily::RedHat),
      "arch" => Ok(OperatingSystemFamily::Arch),
      "suse" => Ok(OperatingSystemFamily::Suse),
      "nixos" => Ok(OperatingSystemFamily::NixOS),
      "macos" => Ok(OperatingSystemFamily::MacOs),
      "windows" => Ok(OperatingSystemFamily::Windows),
      _ => Err(()),
    }
  }
}

impl std::fmt::Display for OperatingSystemFamily {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub fn get_distro_family() -> OperatingSystemFamily {
  if cfg!(target_os = "macos") {
    return OperatingSystemFamily::MacOs;
  } else if cfg!(target_os = "windows") {
    return OperatingSystemFamily::Windows;
  }

  let os_release = std::fs::read_to_string("/etc/os-release")
    .expect("Failed to read /etc/os-release");
  for line in os_release.lines() {
    if line.starts_with("ID_LIKE=") {
      return OperatingSystemFamily::from_str(
        line.trim_start_matches("ID_LIKE=").replace("\"", "").as_str(),
      )
      .unwrap_or(OperatingSystemFamily::Unknown);
    }
  }
  for line in os_release.lines() {
    if line.starts_with("ID=") {
      return OperatingSystemFamily::from_str(
        line.trim_start_matches("ID=").replace("\"", "").as_str(),
      )
      .unwrap_or(OperatingSystemFamily::Unknown);
    }
  }
  eprintln!("Error: Failed to determine Linux distribution family");
  std::process::exit(1);
}
