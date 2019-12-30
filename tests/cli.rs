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
tags = ['info', 'echo', 'grp_1']

[scripts.test_cmd_2]
alias = 'test_cmd_2'
command = 'echo test_2' 
tags = ['debug', 'echo']
"#;

// Tests listing all scripts
pier_test!(cli => test_list_scripts, cfg => CONFIG_1,
| _cfg: ChildPath, mut cmd: Command | {
    // TODO Add some way to verify the output other than exit code.
    cmd.arg("list");
    cmd.assert()
        .success();
});

// Tests listing all aliases
pier_test!(cli => test_list_aliases, cfg => CONFIG_1,
| _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["list", "-q"]);
    cmd.assert()
        .success()
        .stdout(contains(trim!(r#"
        test_cmd_1
        test_cmd_2
    "#)).trim()
    );
});

// Tests listing all aliases with matching tag
pier_test!(cli => test_list_alias_with_matching_tag, cfg => CONFIG_1,
| _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["list", "-q", "-t", "info"]);
    cmd.assert()
        .success()
        .stdout(contains(trim!(r#"
        test_cmd_1
    "#)).trim()
    );
});

// Tests that no aliases is listed when no tag is matched
pier_test!(cli => test_list_without_tag_match, cfg => CONFIG_1,
| _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["list", "-q", "-t", "bad_tag"]);
    cmd.assert()
        .success()
        .stdout(contains("").trim()
    );
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

// Tests show a script
pier_test!(cli => test_show_script, cfg => r#"
[scripts.test_show]
alias = 'test_show'
command = '''
for line in l1 l2 l3 l4; do
    echo "$line"
done
'''
"#,
| _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["show", "test_show"]);
    cmd.assert()
        .success()
        .stdout(contains(trim!(r#"
            for line in l1 l2 l3 l4; do
                echo "$line"
            done"#
        )));
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

// Tests that stderr is printed when the command outputs it.
pier_test!(cli => test_stderr_output, cfg => r#"
[scripts.write_to_stderr]
alias = "write_to_stderr"
command = '''
    echo "WRITE THIS TO STDERR" >&2
'''
"#, | _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["run", "write_to_stderr"]);
    cmd.assert()
        .success()
        .stderr(contains("WRITE THIS TO STDERR"));
});

// Tests that shebangs work.
pier_test!(cli => test_run_with_shebang, cfg => r#"
[scripts.test_shebang]
alias = "test_shebang"
command = '''
#!/usr/bin/env python3
print("Running python script with shebang!")
'''
"#, | _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["run", "test_shebang"]);
    cmd.assert()
        .success()
        .stdout(contains("Running python script with shebang!"));
});

// Tests that default interpreter works
pier_test!(cli => test_run_with_interpreter, cfg => r#"
default_interpreter = ["python3", "-c"]
[scripts.test_default_interpreter]
alias = "test_default_interpreter"
command = '''
print("Running python script with interpreter!")
'''
"#, | _cfg: ChildPath, mut cmd: Command | {
    cmd.args(&["run", "test_default_interpreter"]);
    cmd.assert()
        .success()
        .stdout(contains("Running python script with interpreter!"));
});
