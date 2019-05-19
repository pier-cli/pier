use super::*;

#[test]
fn can_run_command() {
  let alias = "echo";
  let command = "echo hello";
  let arg = "";

  run_command(alias, command, arg)
}