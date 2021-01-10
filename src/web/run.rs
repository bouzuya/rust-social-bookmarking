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
use web::{delete, patch, post, Data, Json, Path};

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

#[derive(Debug, Deserialize)]
struct ListBookmarksByUserKeyPath {
    user_key: String,
}

#[derive(Debug, Serialize)]
struct ListBookmarksByUserKeyResponse {
    bookmarks: Vec<ListBookmarksByUserKeyBookmarkResponse>,
}

#[derive(Debug, Serialize)]
struct ListBookmarksByUserKeyBookmarkResponse {
    key: String,
    url: String,
    comment: String,
    title: String,
}

async fn list_bookmarks_by_user_key(
    app: Data<crate::app::App>,
    path: Path<ListBookmarksByUserKeyPath>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let user_key = path
        .user_key
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    let bookmarks = app
        .list_bookmarks_by_user_key_use_case()
        .list_bookmarks_by_user_key(&user_key)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok(
        actix_web::HttpResponse::Ok().json(ListBookmarksByUserKeyResponse {
            bookmarks: bookmarks
                .iter()
                .map(|bookmark| ListBookmarksByUserKeyBookmarkResponse {
                    key: bookmark.key().to_string(),
                    url: bookmark.url().to_string(),
                    comment: bookmark.comment().to_string(),
                    title: bookmark.title().to_string(),
                })
                .collect::<Vec<ListBookmarksByUserKeyBookmarkResponse>>(),
        }),
    )
}

#[derive(Debug, Serialize)]
struct ListCurrentUserBookmarksResponse {
    bookmarks: Vec<ListCurrentUserBookmarksBookmarkResponse>,
}

#[derive(Debug, Serialize)]
struct ListCurrentUserBookmarksBookmarkResponse {
    key: String,
    url: String,
    comment: String,
    title: String,
}

async fn list_current_user_bookmarks(
    app: Data<crate::app::App>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let bookmarks = app
        .list_current_user_bookmarks_use_case()
        .list_current_user_bookmarks()
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok(
        actix_web::HttpResponse::Ok().json(ListCurrentUserBookmarksResponse {
            bookmarks: bookmarks
                .iter()
                .map(|bookmark| ListCurrentUserBookmarksBookmarkResponse {
                    key: bookmark.key().to_string(),
                    url: bookmark.url().to_string(),
                    comment: bookmark.comment().to_string(),
                    title: bookmark.title().to_string(),
                })
                .collect::<Vec<ListCurrentUserBookmarksBookmarkResponse>>(),
        }),
    )
}

#[derive(Debug, Deserialize)]
struct ResetPasswordRequestBody {
    mail_address: String,
}

async fn reset_password(
    app: Data<crate::app::App>,
    body: Json<ResetPasswordRequestBody>,
) -> actix_web::Result<String> {
    let mail_address = body
        .mail_address
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.reset_password_use_case()
        .reset_password(&mail_address)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Deserialize)]
struct SignInRequestBody {
    mail_address: String,
    password: String,
}

async fn sign_in(
    app: Data<crate::app::App>,
    body: Json<SignInRequestBody>,
) -> actix_web::Result<String> {
    let mail_address = body
        .mail_address
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    let password = body
        .password
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.sign_in_use_case()
        .sign_in(&mail_address, &password)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

async fn sign_out(app: Data<crate::app::App>) -> actix_web::Result<String> {
    app.sign_out_use_case()
        .sign_out()
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Deserialize)]
struct SignUpRequestBody {
    mail_address: String,
    password: String,
}

async fn sign_up(
    app: Data<crate::app::App>,
    body: Json<SignUpRequestBody>,
) -> actix_web::Result<String> {
    let mail_address = body
        .mail_address
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    let password = body
        .password
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.sign_up_use_case()
        .sign_up(mail_address, password)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Deserialize)]
struct UpdateBookmarkPath {
    bookmark_key: String,
}

#[derive(Debug, Deserialize)]
struct UpdateBookmarkRequestBody {
    url: String,
    title: String,
    comment: String,
}

async fn update_bookmark(
    app: Data<crate::app::App>,
    path: Path<UpdateBookmarkPath>,
    body: Json<UpdateBookmarkRequestBody>,
) -> actix_web::Result<String> {
    let bookmark_key = path
        .bookmark_key
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    let bookmark_url = body
        .url
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    let bookmark_title = body
        .title
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    let bookmark_comment = body
        .comment
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.update_bookmark_use_case()
        .update_bookmark(bookmark_key, bookmark_url, bookmark_title, bookmark_comment)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Deserialize)]
struct UpdateMailAddressRequestBody {
    mail_address: String,
}

async fn update_mail_address(
    app: Data<crate::app::App>,
    body: Json<UpdateMailAddressRequestBody>,
) -> actix_web::Result<String> {
    let mail_address = body
        .mail_address
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.update_mail_address_use_case()
        .update_mail_address(&mail_address)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Deserialize)]
struct UpdatePasswordRequestBody {
    password: String,
}

async fn update_password(
    app: Data<crate::app::App>,
    body: Json<UpdatePasswordRequestBody>,
) -> actix_web::Result<String> {
    let password = body
        .password
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.update_password_use_case()
        .update_password(&password)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

#[derive(Debug, Deserialize)]
struct UpdatePasswordBySecretPath {
    secret: String,
}

#[derive(Debug, Deserialize)]
struct UpdatePasswordBySecretRequestBody {
    password: String,
}

async fn update_password_by_secret(
    app: Data<crate::app::App>,
    path: Path<UpdatePasswordBySecretPath>,
    body: Json<UpdatePasswordBySecretRequestBody>,
) -> actix_web::Result<String> {
    let secret = path
        .secret
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    let password = body
        .password
        .parse()
        .map_err(|_| actix_web::HttpResponse::BadRequest())?;
    app.update_password_by_secret_use_case()
        .update_password_by_secret(&secret, &password)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?;
    Ok("".to_string())
}

async fn main(app: crate::app::App) -> Result<()> {
    let app_data = Data::new(app);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/bookmarks", post().to(create_bookmark))
            .route("/bookmarks/{bookmark_key}", delete().to(delete_bookmark))
            .route("/bookmarks/{bookmark_key}", patch().to(update_bookmark))
            .route("/credentials", post().to(sign_up))
            .route("/mail_address_updates", post().to(update_mail_address))
            .route("/password_resets", post().to(reset_password))
            .route(
                "/password_resets/{secret}",
                patch().to(update_password_by_secret),
            )
            .route("/sessions", post().to(sign_in))
            .route("/sessions/current", delete().to(sign_out))
            .route("/users", post().to(create_user))
            .route("/users/{user_key}", delete().to(delete_user))
            .route("/users/me", get().to(get_current_user))
            .route("/users/me/password", patch().to(update_password))
            .route(
                "/users/{user_key}/bookmarks",
                get().to(list_bookmarks_by_user_key),
            )
            .route("/users/me/bookmarks", get().to(list_current_user_bookmarks))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
