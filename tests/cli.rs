use assert_cmd::prelude::*;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use predicates::prelude::*;
use predicates::str::contains;
use std::process::Command;

const CONFIG_1: &'static str = r#"
[scripts.test_cmd_1]
alias = 'test_cmd_1'
command = 'echo test_1' 

[scripts.test_cmd_2]
alias = 'test_cmd_2'
command = 'echo test_2' 
"#;

// Tests listing all scripts
pier_test!(cli => test_list_scripts, cfg => CONFIG_1,
| _cfg: ChildPath, mut cmd: Command | {
    // TODO Add some way to verify the output other than exit code.
    cmd.arg("list");
    cmd.assert().success();
});

// Tests adding a script
pier_test!(cli => test_add_script, cfg => CONFIG_1,
| cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["add", r#"echo test_3"#, "-a", "test_cmd_3"]);
    cmd.assert().success();

    cfg.assert(contains(trim!(r#"
        [scripts.test_cmd_3]
        alias = 'test_cmd_3'
        command = 'echo test_3' 
    "#)).trim()
    );
});

// Tests removing a script
pier_test!(cli => test_remove_script, cfg => CONFIG_1,
| cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["remove", "test_cmd_1"]);
    cmd.assert().success();
    cfg.assert(contains(trim!(r#"
        [scripts.test_cmd_1]
        alias = 'test_cmd_1'
        command = 'echo test_1' 
        "#)).trim().not()
    );

});

// Tests running a very basic script
pier_test!(cli => test_run_script, cfg => CONFIG_1,
| _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["run", "test_cmd_1"]);
    cmd.assert()
        .success()
        .stdout(contains("test_1"));
});

// Tests running a script with a pipe in it
pier_test!(cli => test_run_script_pipe, cfg => r#"
[scripts.test_pipe]
alias = "test_pipe"
command = '''
    echo "TEST_RUN_SCRIPT_PIPE_OUTPUT" | tr '[:upper:]' '[:lower:]'
'''
"#, | _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["run", "test_pipe"]);
    cmd.assert()
        .success()
        .stdout(contains("test_run_script_pipe_output"));
});
//
// Tests running a script which requires waiting for the process to end. 
pier_test!(cli => test_run_script_non_blocking, cfg => r#"
[scripts.test_while_loop]
alias = "test_while_loop"
command = '''
echo "1\\n2\\n3" | while read i; do echo "line read $i"; done
'''
"#, | _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["run", "test_while_loop"]);
    cmd.assert()
        .success();
});

// Tests running a script with a &&
pier_test!(cli => test_run_script_and, cfg => r#"
[scripts.test_and]
alias = "test_and"
command = '''
    echo "test-x-1" && echo "test-x-2"
'''
"#, | _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["run", "test_and"]);
    cmd.assert()
        .success()
        .stdout(contains("test-x-1\ntest-x-2"));
});
