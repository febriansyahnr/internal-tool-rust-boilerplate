use std::sync::Arc;

use internal::service::backend_portal::BackendPortalService;

#[derive(Debug, Clone)]
pub struct AppState<'a> {
    pub backend_portal_service: Arc<BackendPortalService<'a>>,
}
