use reqwest::StatusCode;

use crate::config::Config;
use crate::model::backend_portal::{ReconItem, Response as BEResponse};
use crate::port::backend_portal::CanRecon;
use crate::secret::Secret;
use crate::{Error, Result};

#[derive(Debug)]
pub struct BackendPortal<'a> {
    config: Box<&'a Config>,
    secret: Box<&'a Secret>,
}

impl<'a> BackendPortal<'a> {
    pub fn new(config: Box<&'a Config>, secret: Box<&'a Secret>) -> Self {
        Self { config, secret }
    }
}

impl CanRecon for BackendPortal<'_> {
    async fn get_list_recon(&self) -> Result<Vec<ReconItem>> {
        let crm_key = self.secret.backend_portal.crm_key.as_str();

        let mut base_url = self.config.backend_portal.base_url.clone();
        let path = "/crm/v1/recon/list";
        base_url.push_str(&path);

        let query_string = [("page", "1"), ("status", "FAILED")];
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-CRM-KEY", crm_key.parse().unwrap_or("".parse().unwrap()));

        let client = reqwest::Client::new();
        let response = client
            .get(base_url)
            .query(&query_string)
            .headers(headers)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let data = response.text().await?;
                let json_data = serde_json::from_str::<BEResponse<Vec<ReconItem>>>(data.as_str())?;

                println!("result {:?}", json_data);

                Ok(json_data.data)
            }
            StatusCode::NOT_FOUND => Err(Error::GeneralError {
                message: "page not found".into(),
            }),
            _ => Err(Error::GeneralError {
                message: "error when send request to backend portal".into(),
            }),
        }
    }
}

#[allow(unused_imports)]
mod test_recon {
    use crate::{config::get_config, secret::get_secret};

    use super::*;
    #[tokio::test]
    async fn get_recon() -> Result<()> {
        let config = get_config();
        let secret = get_secret();
        let be_portal = BackendPortal::new(Box::new(config), Box::new(secret));
        let list_recon = be_portal.get_list_recon().await?;
        assert_ne!(list_recon.len(), 0);
        Ok(())
    }
}
