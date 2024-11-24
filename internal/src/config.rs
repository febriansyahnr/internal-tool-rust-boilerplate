use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub environtment: String,
    pub service_name: String,
    pub port_api: Option<u16>,

    pub clickup: ClickupConfig,
    pub backend_portal: BackendPortalConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClickupConfig {
    pub base_url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BackendPortalConfig {
    pub base_url: String,
}

impl Config {
    fn new() -> Result<Self> {
        let workspace_config = std::env::current_dir()?.join(".config.yaml");
        println!("try read config from {:?}", workspace_config);
        let f = std::fs::File::open(workspace_config)?;
        let conf: Config = serde_yaml::from_reader(f)?;

        Ok(conf)
    }
}

pub fn get_config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::new().unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

mod test_config {
    use super::get_config;

    #[test]
    fn success_load_config() {
        let cfg = get_config();
        println!("config: {:?}", cfg)
    }
}
