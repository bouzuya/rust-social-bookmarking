use crate::cli::ConsoleSendMailService;
use crate::cli::FsSessionService;
use crate::pg::*;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::sync::Arc;

pub async fn run() -> Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect(&format!("Error connecting to {}", database_url));

    let user_repository = Arc::new(PgUserRepository::new(pool.clone()));
    let app = crate::app::App::new(
        Arc::new(PgBookmarkRepository::new(pool.clone())),
        Arc::new(PgCredentialRepository::new(pool.clone())),
        Arc::new(ConsoleSendMailService::new()),
        Arc::new(FsSessionService::new(user_repository.clone())),
        user_repository,
    );
    main(app).await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn main(_: crate::app::App) -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
