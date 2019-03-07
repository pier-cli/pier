use clap::load_yaml;
use clap::App;

use toml;
#[cfg(test)]
use serde::{Debug, Deserialize};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    //...
}
