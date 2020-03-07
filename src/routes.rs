//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::admin::{assign_admin, get_admin};
use crate::handlers::content::{create_content, get_content};
use crate::handlers::health::get_health;
use crate::handlers::inbox::{get_inbox_by_user, insert};
use crate::handlers::message::{get_message, send_message};
use crate::handlers::user::{create_user, get_user};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        .service(
            web::scope("/api/v1")
                .service(
                    web::scope("/user")
                        .route("/create", web::post().to(create_user))
                        .route("/find/{username}", web::get().to(get_user)),
                )
                .service(
                    web::scope("/content")
                        .route("/create", web::post().to(create_content))
                        .route("/find/{id}", web::get().to(get_content)),
                )
                .service(
                    web::scope("/message")
                        .route("/send/{username}", web::post().to(send_message))
                        .route("/find/{id}", web::get().to(get_message)),
                )
                .service(
                    web::scope("/admin")
                        .route("/assign", web::post().to(assign_admin))
                        .route("/is_admin/{id}", web::get().to(get_admin)),
                )
                .service(
                    web::scope("/inbox")
                        .route("/insert", web::post().to(insert))
                        .route("/find/{id}", web::get().to(get_inbox_by_user)),
                ),
        );
}
