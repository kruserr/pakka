pub fn zfs_create_snapshot(pool: &str, dataset: &str, snapshot: &str) {
  println!("zfs_create_snapshot({pool}, {dataset}, {snapshot})");
  return;

  let snapshot_name = format!("{}@{}", dataset, snapshot);
  let output = std::process::Command::new("zfs")
    .arg("snapshot")
    .arg(&snapshot_name)
    .output()
    .expect("Failed to create ZFS snapshot");

  if output.status.success() {
    println!("ZFS snapshot created successfully");
  } else {
    eprintln!("Failed to create ZFS snapshot");
  }
}

pub fn zfs_rollback_snapshot(pool: &str, dataset: &str, snapshot: &str) {
  println!("zfs_rollback_snapshot({pool}, {dataset}, {snapshot})");
  return;

  let snapshot_name = format!("{}@{}", dataset, snapshot);
  let output = std::process::Command::new("zfs")
    .arg("rollback")
    .arg(&snapshot_name)
    .output()
    .expect("Failed to rollback ZFS snapshot");

  if output.status.success() {
    println!("ZFS rollback successful");
  } else {
    eprintln!("Failed to rollback ZFS snapshot");
  }
}
