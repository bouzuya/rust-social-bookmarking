pub mod bookmark_repository_impl;
pub mod credential_repository_impl;
pub mod send_mail_service_impl;
pub mod session_service_impl;
pub mod user_repository_impl;

use crate::app::App;
use crate::fake::{
    bookmark_repository_impl::BookmarkRepositoryImpl,
    credential_repository_impl::CredentialRepositoryImpl,
    send_mail_service_impl::SendMailServiceImpl, session_service_impl::SessionServiceImpl,
    user_repository_impl::UserRepositoryImpl,
};
use anyhow::Result;
use std::sync::Arc;

fn create_bookmark(app: &App) -> Result<()> {
    let url = "https://bouzuya.net".parse().unwrap();
    let title = "bouzuya.net".parse().unwrap();
    let comment = "bouzuya's webpage".parse().unwrap();
    app.create_bookmark_use_case()
        .create_bookmark(url, title, comment)
}

fn create_user(app: &App) -> Result<()> {
    let secret = "1".repeat(64).parse().unwrap();
    app.create_user_use_case().create_user(secret)
}

fn delete_bookmark(app: &App) -> Result<()> {
    let bookmark_key = "1234567890123456".parse().unwrap();
    app.delete_bookmark_use_case()
        .delete_bookmark(&bookmark_key)
}

fn delete_user(app: &App) -> Result<()> {
    let user_key = "123456789012".parse().unwrap();
    app.delete_user_use_case().delete_user(&user_key)
}

fn get_current_user(app: &App) -> Result<()> {
    let current_user = app.get_current_user_use_case().get_current_user()?;
    println!("{:?}", current_user);
    Ok(())
}

fn list_current_user_bookmarks(app: &App) -> Result<()> {
    let bookmarks = app
        .list_current_user_bookmarks_use_case()
        .list_current_user_bookmarks()?;
    println!("{:?}", bookmarks);
    Ok(())
}

fn list_bookmarks_by_user_key(app: &App) -> Result<()> {
    let user_key = "123456789012".parse().unwrap();
    let bookmarks = app
        .list_bookmarks_by_user_key_use_case()
        .list_bookmarks_by_user_key(&user_key)?;
    println!("{:?}", bookmarks);
    Ok(())
}

fn reset_password(app: &App) -> Result<()> {
    let mail_address = "m@bouzuya.net".parse().unwrap();
    app.reset_password_use_case().reset_password(&mail_address)
}

fn sign_in(app: &App) -> Result<()> {
    let mail_address = "m@bouzuya.net".parse().unwrap();
    let password = "password".parse().unwrap();
    app.sign_in_use_case().sign_in(&mail_address, &password)
}

fn sign_out(app: &App) -> Result<()> {
    app.sign_out_use_case().sign_out()
}

fn sign_up(app: &App) -> Result<()> {
    let mail_address = "m@bouzuya.net".parse().unwrap();
    let password = "password".parse().unwrap();
    app.sign_up_use_case().sign_up(mail_address, password)
}

fn update_bookmark(app: &App) -> Result<()> {
    let bookmark_key = "1234567890123456".parse().unwrap();
    let bookmark_url = "https://bouzuya.net".parse().unwrap();
    let bookmark_title = "bouzuya.net".parse().unwrap();
    let bookmark_comment = "bouzuya's website".parse().unwrap();
    app.update_bookmark_use_case().update_bookmark(
        bookmark_key,
        bookmark_url,
        bookmark_title,
        bookmark_comment,
    )
}

fn update_mail_address(app: &App) -> Result<()> {
    let mail_address = "m2@bouzuya.net".parse().unwrap();
    app.update_mail_address_use_case()
        .update_mail_address(&mail_address)
}

fn update_password(app: &App) -> Result<()> {
    let password = "password1".parse().unwrap();
    app.update_password_use_case().update_password(&password)
}

fn update_password_by_secret(app: &App) -> Result<()> {
    let secret = "1".repeat(255).parse().unwrap();
    let password = "password1".parse().unwrap();
    app.update_password_by_secret_use_case()
        .update_password_by_secret(&secret, &password)
}

fn verify_mail_address(app: &App) -> Result<()> {
    let secret = "1".repeat(255).parse().unwrap();
    app.verify_mail_address_use_case()
        .verify_mail_address(&secret)
}

pub fn run() {
    let app = crate::app::App::new(
        Arc::new(BookmarkRepositoryImpl::new()),
        Arc::new(CredentialRepositoryImpl::new()),
        Arc::new(SendMailServiceImpl::new()),
        Arc::new(SessionServiceImpl::new()),
        Arc::new(UserRepositoryImpl::new()),
    );
    sign_up(&app).expect("sign up");
    create_user(&app).expect("create user");
    sign_in(&app).expect("sign in");
    get_current_user(&app).expect("get current user");
    create_bookmark(&app).expect("create bookmark");
    list_current_user_bookmarks(&app).expect("list current_user bookmarks");
    list_bookmarks_by_user_key(&app).expect("list bookmarks by user_key");
    update_bookmark(&app).expect("update bookmark");
    delete_bookmark(&app).expect("delete bookmark");
    update_mail_address(&app).expect("update mail_address");
    verify_mail_address(&app).expect("verify mail_address");
    update_password(&app).expect("update password");
    sign_out(&app).expect("sign out");
    reset_password(&app).expect("reset password");
    update_password_by_secret(&app).expect("update password by secret");
    sign_in(&app).expect("sign in (2)");
    delete_user(&app).expect("delete user");
}
