use assert_cmd::prelude::*;
use assert_fs::prelude::*; //use assert_fs::{TempDir, fixture::ChildPath};
use assert_fs::{fixture::ChildPath, TempDir};
use pier::{self, Config};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub struct TestEnv {
    pub dir: TempDir,
}
impl TestEnv {
    pub fn new() -> TestEnv {
        TestEnv {
            dir: TempDir::new().expect("Failed to create temp dir."),
        }
    }
    pub fn join_root(&self, rel_path: &str) -> PathBuf {
        self.dir
            .path()
            .join(Path::new(rel_path))
            .to_path_buf()
    }
    pub fn create_config(&mut self, path: &str, content: &str) -> ChildPath {
        let config_file = self.dir.child(path);
        config_file
            .write_str(content)
            .expect("Failed to content to file.");
        config_file
    }
}
pub fn setup_dir(content: &str) -> (ChildPath, TestEnv) {
    let mut te = TestEnv::new();
    let cfg = te.create_config("pier_config", content);
    (cfg, te)
}

pub fn setup_cli(content: &str) -> (ChildPath, TestEnv, Command) {
    let mut cmd = Command::cargo_bin("pier").expect("Failed to set cargo binary pier");
    let (cfg, te) = setup_dir(content);

    cmd.current_dir(te.dir.path());

    cmd.args(&["-c", cfg.path().to_str().unwrap()]);

    (cfg, te, cmd)
}

pub fn setup_lib(content: &str) -> (ChildPath, TestEnv, pier::Result<Config>) {
    let (cfg, te) = setup_dir(content);
    let path = te.dir.path().join(&cfg.path());
    let lib = Config::from_file(path);
    (cfg, te, lib)
}
