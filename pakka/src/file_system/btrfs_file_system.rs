pub fn btrfs_create_snapshot(source: &str, dest: &str) {
  println!("btrfs_create_snapshot({source}, {dest})");
  return;

  let output = std::process::Command::new("btrfs")
    .arg("subvolume")
    .arg("snapshot")
    .arg(source)
    .arg(dest)
    .output()
    .expect("Failed to create Btrfs snapshot");

  if output.status.success() {
    println!("Btrfs snapshot created successfully");
  } else {
    eprintln!("Failed to create Btrfs snapshot");
  }
}

pub fn btrfs_rollback_snapshot(current: &str, snapshot: &str) {
  println!("btrfs_rollback_snapshot({current}, {snapshot})");
  return;

  let delete_output = std::process::Command::new("btrfs")
    .arg("subvolume")
    .arg("delete")
    .arg(current)
    .output()
    .expect("Failed to delete current Btrfs subvolume");

  if delete_output.status.success() {
    let create_output = std::process::Command::new("btrfs")
      .arg("subvolume")
      .arg("snapshot")
      .arg(snapshot)
      .arg(current)
      .output()
      .expect("Failed to create Btrfs snapshot");

    if create_output.status.success() {
      println!("Btrfs rollback successful");
    } else {
      eprintln!("Failed to create Btrfs snapshot for rollback");
    }
  } else {
    eprintln!("Failed to delete current Btrfs subvolume");
  }
}
