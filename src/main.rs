use clap::App;

use toml;
use serde_derive::{Debug, Deserialize};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    //...
}
