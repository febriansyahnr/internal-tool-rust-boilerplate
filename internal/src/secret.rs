use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub backend_portal: BackendPortalSecret,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendPortalSecret {
    pub internal_key: String,
    pub crm_key: String,
}

impl Secret {
    fn new() -> Result<Self> {
        let workspace_secret = std::env::current_dir()?.join(".secret.yaml");
        println!("try read config from {:?}", workspace_secret);
        let f = std::fs::File::open(workspace_secret)?;
        let conf: Secret = serde_yaml::from_reader(f)?;

        Ok(conf)
    }
}

pub fn get_secret() -> &'static Secret {
    static INSTANCE: OnceLock<Secret> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Secret::new().unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}
