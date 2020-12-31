use crate::cli::ConsoleSendMailService;
use crate::cli::FsSessionService;
use crate::pg::*;
use actix_web::{
    web::{self, get},
    App, HttpServer,
};
use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use web::{delete, post, Data, Json, Path};

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

#[derive(Debug, Deserialize)]
struct CreateBookmarkRequestBody {
    url: String,
    title: String,
    comment: String,
}

async fn create_bookmark(
    app: Data<crate::app::App>,
    json: Json<CreateBookmarkRequestBody>,
) -> actix_web::Result<String> {
    let url = json
        .url
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    let title = json
        .title
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    let comment = json
        .comment
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.create_bookmark_use_case()
        .create_bookmark(url, title, comment)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Deserialize)]
struct CreateUserRequestBody {
    secret: String,
}

async fn create_user(
    app: Data<crate::app::App>,
    json: Json<CreateUserRequestBody>,
) -> actix_web::Result<String> {
    let secret = json
        .secret
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.create_user_use_case()
        .create_user(secret)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Deserialize)]
struct DeleteBookmarkPath {
    bookmark_key: String,
}

async fn delete_bookmark(
    app: Data<crate::app::App>,
    path: Path<DeleteBookmarkPath>,
) -> actix_web::Result<String> {
    let bookmark_key = path
        .bookmark_key
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.delete_bookmark_use_case()
        .delete_bookmark(&bookmark_key)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Deserialize)]
struct DeleteUserPath {
    user_key: String,
}

async fn delete_user(
    app: Data<crate::app::App>,
    path: Path<DeleteUserPath>,
) -> actix_web::Result<String> {
    let user_key = path
        .user_key
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.delete_user_use_case()
        .delete_user(&user_key)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Serialize)]
struct GetCurrentUserResponse {
    key: String,
}

async fn get_current_user(
    app: Data<crate::app::App>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let current_user = app
        .get_current_user_use_case()
        .get_current_user()
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    let current_user = current_user.ok_or_else(|| actix_web::HttpResponse::NotFound())?;
    Ok(actix_web::HttpResponse::Ok().json(GetCurrentUserResponse {
        key: current_user.key().to_string(),
    }))
}

async fn main(app: crate::app::App) -> Result<()> {
    let app_data = Data::new(app);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/bookmarks", post().to(create_bookmark))
            .route("/bookmarks/{bookmark_key}", delete().to(delete_bookmark))
            .route("/users", post().to(create_user))
            .route("/users/{user_key}", delete().to(delete_user))
            .route("/users/me", get().to(get_current_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(anyhow::Error::from)
}