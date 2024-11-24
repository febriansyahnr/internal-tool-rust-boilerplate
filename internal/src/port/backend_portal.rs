use crate::{model::backend_portal::ReconItem, Result};
use std::future::Future;

// -- recon
pub trait CanRecon {
    fn get_list_recon(&self) -> impl Future<Output = Result<Vec<ReconItem>>> + Send;
}
