use actix_web::{get, web, HttpResponse, Responder};
use internal::model::backend_portal::ReconItem;
use internal::port::backend_portal::CanRecon;
use internal::Response;

use crate::state::AppState;

#[get("/api/v1/backend-portal/recon")]
pub async fn get_list_recon<'a>(state: web::Data<AppState<'a>>) -> impl Responder {
    let mut response: Response<Vec<ReconItem>> = Response::default();
    let backend_portal_service = state.backend_portal_service.as_ref();

    let result = backend_portal_service.get_list_recon().await;

    match result {
        Ok(list_recon) => {
            response.success("success get list recon");
            response.with_data(list_recon);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            response.code = "01".to_string();
            response.message = e.to_string();
            HttpResponse::InternalServerError().json(response)
        }
    }
}
