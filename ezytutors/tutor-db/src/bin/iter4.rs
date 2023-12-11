use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter4/db_access.rs"]
mod db_access;
#[path = "../iter4/handlers.rs"]
mod handlers;
#[path = "../iter4/models.rs"]
mod models;
#[path = "../iter4/routes.rs"]
mod routes;
#[path = "../iter4/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    // Construct AppState - note that we are storing the connection pool in the db field
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    // Construct the app instance
    let app = move || {
        App::new()
            .app_data(shared_data.clone()) // Inject the app state into the instance
            .configure(general_routes) // Configure the routes
            .configure(course_routes)
    };

    //Start HTTP server
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    HttpServer::new(app).bind(&host_port)?.run().await
}
