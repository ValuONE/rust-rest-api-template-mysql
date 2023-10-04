use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::api::todo::*;
use crate::utility::middleware::middleware;
use crate::utility::setup::setup;

mod model;
mod utility;
mod repository;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (app_state, config) = setup().await;

    HttpServer::new(move || {
        let middleware = HttpAuthentication::bearer(middleware);

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(middleware)
            .wrap(cors)
            .app_data(Data::new(app_state.clone()))
            .service(update_todo)
            .service(delete_todo)
            .service(create_todo)
            .service(get_all_todos)
            .service(get_todo_by_id)
    })
        .bind((config.ip, config.port))?
        .run()
        .await
}
