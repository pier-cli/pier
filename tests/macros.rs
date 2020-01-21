/// Asserts if a Result matches a specific error kind.
///
/// # Examples
/// ```
/// err_eq!(lib.remove_script("this-alias-does-not-exist"), AliasNotFound)
/// ```
macro_rules! err_eq {
    ($enum:expr, $enumkind:ident) => {
        assert_eq!(
            PierErrorKind::from($enum.expect_err("No error to compare")),
            PierErrorKind::$enumkind
        );
    };
}

/// Creates a new test case for varying use cases
///
/// # Examples
///
/// ## Used for creating command line tests
/// ```
/// pier_test!(cli => r#"
/// [scripts.example]
/// alias = "example"
/// command = "echo example"
/// "#| _cfg: ChildPath, mut cmd: Command | {
///     ... do some cli tests here
/// });
/// ```
///
/// ## Used for creating tests using the library
/// ```
/// pier_test!(lib => r#"
/// [scripts.example]
/// alias = "example"
/// command = "echo example"
/// "#| _cfg: ChildPath, mut lib: Config | {
///     ... do some library tests here
/// });
/// ```
///
/// ## Used for creating tests which need more control over the base parameters
/// ```
/// pier_test!(basic => r#"
/// [scripts.example]
/// alias = "example"
/// command = "echo example"
/// "#| _cfg: ChildPath, mut lib: Config | {
///     ... do some tests here
/// });
/// ```
#[macro_export]
macro_rules! pier_test {
    (cli => $name:ident, cfg => $content:expr, $func:expr) => {
        #[test]
        fn $name() {
            let (cfg, _dir, cmd) = crate::common::setup_cli(trim!($content));
            $func(cfg, cmd)
        }
    };
    (lib => $name:ident, cfg => $content:expr, $func:expr) => {
        #[test]
        fn $name() {
            let (cfg, _dir, lib) = crate::common::setup_lib(trim!($content));
            $func(cfg, lib.expect("Failed to load config file."))
        }
    };
    (basic => $name:ident, $func:expr) => {
        #[test]
        fn $name() {
            let dir = crate::common::TestEnv::new();
            $func(dir)
        }
    };
}

/// Trims leading spaces on a raw string so you can properly indent it for nicer code overview.
///
/// # Examples
/// ```
/// let trimmed = trim!(r#"
///     something something something
///     something something something
///     something something something
/// "#);
///
/// let untrimmed =
/// r#"something something something
/// something something something
/// something something something"#;
/// assert_eq!(trimmed, untrimmed)
///
/// ```
#[macro_export]
macro_rules! trim {
    ($text:expr) => {
        $text
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\n")
            .trim()
    };
}
