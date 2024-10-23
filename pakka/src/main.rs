use pakka;

fn main() {
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
