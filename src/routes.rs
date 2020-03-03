//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::health::get_health;
use crate::handlers::user::{create_user, get_user};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        .service(
            web::scope("/api/v1").service(
                web::scope("/user")
                    .route("/create", web::post().to(create_user))
                    .route("/find/{username}", web::get().to(get_user)),
            ),
        );
}
