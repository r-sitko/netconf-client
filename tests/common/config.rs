use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: Config = load_config();
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub netconf: Netconf,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Netconf {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub docker_image: String,
    pub docker_sha: String,
    pub container_name: String,
}

fn load_config() -> Config {
    let mut test_config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_config_path.push("tests/resources/config.toml");
    confy::load_path(test_config_path).unwrap()
}
