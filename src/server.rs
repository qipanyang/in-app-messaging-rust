//! Spin up a HTTPServer

use crate::config::CONFIG;
use crate::database::init_mysql_pool;
use crate::routes::routes;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Create the application state
    // String is used here, but it can be anything
    // Invoke in hanlders using data: AppState<'_, String>
    let pool = init_mysql_pool().unwrap();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(routes)
    })
    .workers(4);

    server = server.bind(&CONFIG.server)?;

    server.run().await
}
