use actix_cors::Cors;
use actix_web::{guard, middleware::Logger, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

mod config;
mod error;
mod passman;
mod users;

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init::init().await;
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let host = std::env::var("APP_HOST").unwrap();
    let port = std::env::var("APP_PORT").unwrap().parse::<u16>().unwrap();
    log::info!("Server is running at http://{}:{}", host, port);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).wrap(Cors::permissive()).service(
            web::scope("/gql")
                .configure(users::handler::config)
                .configure(passman::handler::config)
                .service(
                    web::resource("/")
                        .guard(guard::Get())
                        .to(graphql_playground),
                ),
        )
    })
    .bind((host, port))?
    .run()
    .await
}
