use std::fs;
use structopt::clap::Shell;

include!("src/cli.rs");

fn main() {
    let pkg_name = env!("CARGO_PKG_NAME");

    if let Ok(outdir) = std::env::var("SHELL_COMPLETIONS_DIR") {

        fs::create_dir_all(&outdir).unwrap();
        Cli::clap().gen_completions(pkg_name, Shell::Bash, &outdir);
        Cli::clap().gen_completions(pkg_name, Shell::Fish, &outdir);
        Cli::clap().gen_completions(pkg_name, Shell::Zsh, &outdir);
    }
}
