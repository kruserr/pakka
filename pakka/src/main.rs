use pakka;

fn main() {
  // Enable heap protection
  std::env::set_var("MALLOC_CHECK_", "3"); // Maximum heap consistency checking
  std::env::set_var("MALLOC_PERTURB_", "123"); // Randomize heap allocations

  pakka::cli_main();
}

#[test]
fn cli_main_smoke_test() {
  let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

  cmd
    .arg("--help")
    .assert()
    .success()
    .stdout(predicates::str::contains(env!("CARGO_PKG_NAME")));
}
