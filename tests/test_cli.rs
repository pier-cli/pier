use assert_cmd::prelude::*;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use predicates::prelude::*;

mod common;
use common::*;

const CONFIG_1: &'static str = r#"
[scripts.test_cmd_1]
alias = 'test_cmd_1'
command = 'echo test_1' 

[scripts.test_cmd_2]
alias = 'test_cmd_2'
command = 'echo test_2' 
"#;


// # Basic tests
pier_test!(test_list_scripts, cfg => CONFIG_1,
| _cfg: ChildPath, mut te: TestEnv | {
    // TODO Add some way to verify the output other than exit code.
    te.cmd.arg("list");
    te.cmd.assert().success();
});

pier_test!(test_add_script, cfg => CONFIG_1,
| cfg: ChildPath, mut te: TestEnv | {
    te.cmd.args(&["add", r#"echo test_3"#, "-a", "test_cmd_3"]);
    te.cmd.assert().success();

    cfg.assert(predicate_str!(contains => r#"
        [scripts.test_cmd_3]
        alias = 'test_cmd_3'
        command = 'echo test_3' 
    "#));
});

pier_test!(test_remove_script, cfg => CONFIG_1,
| cfg: ChildPath, mut te: TestEnv | {
    te.cmd.args(&["remove", "test_cmd_1"]);
    te.cmd.assert().success();
    cfg.assert(predicate_str!(!contains => r#"
        [scripts.test_cmd_1]
        alias = 'test_cmd_1'
        command = 'echo test_1' 
        "#)
    );

});

pier_test!(test_run_script, cfg => CONFIG_1,
| _cfg: ChildPath, mut te: TestEnv | {
    te.cmd.args(&["run", "test_cmd_1"]);
    te.cmd.assert()
        .success()
        .stdout(predicate_str!(contains => "test_1"));
});

// # Run command tests
pier_test!(test_run_script_pipe, cfg => r#"
[scripts.test_pipe]
alias = "test_pipe"
command = '''
    echo "TEST_RUN_SCRIPT_PIPE_OUTPUT" | tr '[:upper:]' '[:lower:]'
'''
"#, | _cfg: ChildPath, mut te: TestEnv | {
    te.cmd.args(&["run", "test_pipe"]);
    te.cmd.assert()
        .success()
        .stdout(predicate_str!(contains => "test_run_script_pipe_output"));
});

pier_test!(test_run_script_and, cfg => r#"
[scripts.test_and]
alias = "test_and"
command = '''
    echo "test-x-1" && echo "test-x-2"
'''
"#, | _cfg: ChildPath, mut te: TestEnv | {
    te.cmd.args(&["run", "test_and"]);
    te.cmd.assert()
        .success();
});
