use assert_cmd::prelude::*;
use assert_fs::prelude::*;
//use assert_fs::{TempDir, fixture::ChildPath};
use assert_fs::{fixture::ChildPath, TempDir};
use std::process::Command;

pub struct TestEnv {
    pub dir: TempDir,
    pub cmd: Command,
}

impl TestEnv {
    pub fn new() -> TestEnv {
        let dir = TempDir::new().expect("Failed to create temp dir.");
        let cmd = Command::cargo_bin("pier").expect("Failed to set cargo binary pier");
        TestEnv { cmd, dir }
    }
    pub fn create_config(&mut self, path: &str, content: &str) -> ChildPath {
        let config_file = self.dir.child(path);
        config_file
            .write_str(content)
            .expect("Failed to content to file.");
        config_file
    }

    pub fn with_config(&mut self, path: &str) {
        self.cmd.args(&["-c", path]);
    }

    pub fn from_config(content: &str) -> (ChildPath, TestEnv) {
        let mut te = TestEnv::new();
        let cfg = te.create_config("pier_config", content);

        te.with_config(cfg.path().to_str().unwrap());
        (cfg, te)
    }
}

/// Construct a Pier Cli Test case.
/// 
/// Usage:
/// pier_test!(<The name of the test case>,
/// cfg => "<content of the config file>", 
/// <anonymous function>
/// );
#[macro_export]
macro_rules! pier_test {
    ($name:ident, cfg => $content:expr, $func:expr) => {
        #[test]
        fn $name() {
            let (cfg, te) = TestEnv::from_config(trim!($content));
            $func(cfg, te)
        }
    };
    ($name:ident, $func:expr) => {
        #[test]
        fn $name() {
            $func(TestEnv::new());
        }
    };
}


/// The trim macro trims each line start/end from whitespaces as well as removes empty before and
/// after the content.
#[macro_export]
macro_rules! trim {
    ($text:expr) => (

    $text.lines().map(|line| {
        line.trim()
    })
    .collect::<Vec<&str>>()
    .join("\n").trim()

    );
}


/// The predicate_str is just a convenience macro to create predicate::str::<some_func>
#[macro_export]
macro_rules! predicate_str {
    ($func:ident => $str:expr) => (
        predicate::str::$func(trim!($str)).trim()
    );
    ($func:ident) => (
        predicate::str::$func().trim()
    );
    // Invert these predicates
    (!$func:ident => $str:expr) => (
        predicate::str::$func(trim!($str)).trim().not()
    );
    (!$func:ident) => (
        predicate::str::$func().trim().not()
    )
}
