/// macro to raise the error specified
#[macro_export]
macro_rules! pier_err {
    ($type:expr) => {
        return Err($type)?;
    };
}

/// macro to return an absolute path base on the relative_path from $XDG_CONFIG_HOME
#[macro_export]
macro_rules! xdg_config_home {
    ($relative_path:expr) => {
        match dirs::config_dir() {
            Some(base) => Some(base.join($relative_path)),
            None => None,
        }
    };
}

/// macro to return an absolute path base on the relative_path from $HOME
#[macro_export]
macro_rules! home {
    ($relative_path:expr) => {
        match dirs::home_dir() {
            Some(base) => Some(base.join($relative_path)),
            None => None,
        }
    };
}

///
#[macro_export]
macro_rules! clap_arg {
    ($relative_path:expr) => {
        match dirs::home_dir() {
            Some(base) => Some(base.join($relative_path)),
            None => None,
        }
    };
}
