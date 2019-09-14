use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn add_and_list() {
    let mut add_cmd = Command::cargo_bin("scriv").unwrap();
    let note = "Hello, world!".to_string();
    add_cmd.args(&["add", &note]).assert().success();

    let mut list_cmd = Command::cargo_bin("scriv").unwrap();
    list_cmd
        .args(&["list"])
        .assert()
        .success()
        .stdout(predicate::str::ends_with(note + "\n"));
}
