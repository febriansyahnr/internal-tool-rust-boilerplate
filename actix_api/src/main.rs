mod handler;
mod state;

use std::sync::Arc;

use env_logger::Env;
use internal::respository::backend_portal::BackendPortal;
use internal::service::backend_portal::BackendPortalService;
use log::info;
use state::AppState;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> internal::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = internal::config::get_config();
    let secret = internal::secret::get_secret();
    let mut port: u16 = 8081;

    if let Some(p) = config.port_api {
        port = p;
    }

    let backend_portal_repo = BackendPortal::new(Box::new(config), Box::new(secret));

    let backend_portal_service = BackendPortalService::new(Box::new(backend_portal_repo));

    let app_state = AppState {
        backend_portal_service: Arc::new(backend_portal_service),
    };

    info!("Starting web api at port {}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(app_state.clone()))
            .service(handler::backend_portal::get_list_recon)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}
