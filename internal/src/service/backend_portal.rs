use crate::model::backend_portal::ReconItem;
use crate::{port::backend_portal::CanRecon, respository::backend_portal::BackendPortal};

#[derive(Debug)]
pub struct BackendPortalService<'a> {
    repo: Box<BackendPortal<'a>>,
}

impl<'a> BackendPortalService<'a> {
    pub fn new(repo: Box<BackendPortal<'a>>) -> Self {
        Self { repo }
    }
}

impl CanRecon for BackendPortalService<'_> {
    async fn get_list_recon(&self) -> crate::Result<Vec<ReconItem>> {
        let list_recon = self.repo.get_list_recon().await?;

        Ok(list_recon)
    }
}
