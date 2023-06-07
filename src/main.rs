use actix_web::{get, web, App, HttpServer, Responder, middleware::Logger};
use sqlx::postgres::{PgPoolOptions, PgPool};
use env_logger::Env;

#[get("/")]
async fn index(pool: web::Data<PgPool>) -> impl Responder {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool.as_ref()).await.unwrap();

    println!("{}", row.0);

    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = web::Data::new(PgPoolOptions::new()
        .max_connections(8)
        .connect("postgres://postgres:iam@localhost/sqlx-test").await
        .map_err(|e| println!("{}", e.to_string()))
        .expect("Can't connect to DB"));

    HttpServer::new(move || 
        App::new()
            .app_data(pool.clone())
            .wrap(Logger::default())
            .service(index)
            .service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}