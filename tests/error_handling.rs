use crate::common::TestEnv;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use pier::error::*;
use pier::Config;

// Tests that it returns the error AliasNotFound if the alias given does not exist
pier_test!(lib => test_error_alias_not_found, cfg => r#"
[scripts.test_cmd_1]
alias = 'test_cmd_1'
command = 'echo test_1' 
"#, | _cfg: ChildPath, mut lib: Config | {
    err_eq!(lib.remove_script("non_existant"), AliasNotFound);
    err_eq!(lib.fetch_script("non_existant"), AliasNotFound);
});

// Tests that it returns the error NoScriptsExists if there is no scripts in the config
pier_test!(lib => test_error_no_scripts_exists, cfg => r#""#,
| _cfg: ChildPath, mut lib: Config | {
    err_eq!(lib.remove_script(""), NoScriptsExists);
    err_eq!(lib.fetch_script(""), NoScriptsExists);
    err_eq!(lib.list_scripts(None), NoScriptsExists);
});

// Tests that it returns the error ConfigRead if the file cannot be read.
// In this case the file is not created
pier_test!(basic => test_config_read_error, | te: TestEnv | {
    let path = te.join_root("non_existant_file");
    let lib = Config::from_input(Some(path));
    err_eq!(lib, ConfigRead);
});

// Tests that it returns the error ConfigWrite if the file cannot be written to
// In this case the file is not created
pier_test!(basic => test_config_write_error, | _te: TestEnv | {
    let lib = Config::new();
    err_eq!(lib.write(), ConfigWrite);
});

// Tests that it returns the error TomlParse if the config is not valid Toml
pier_test!(basic => test_toml_parse_error, | te: TestEnv| {
    let cfg = te.dir.child("pier_config"); 
    cfg.touch().expect("Unable to create file");
    cfg.write_str(trim!(r#"
        [scripts.test_cmd_1]
        alias = 'test_cmd_1'
        command = echo test_1 
        "#)
    ).expect("Unable to write to file");

    let lib = Config::from_file(cfg.path().to_path_buf());
    err_eq!(lib, TomlParse);
});
