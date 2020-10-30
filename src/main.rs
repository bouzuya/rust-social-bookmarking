mod bookmark_repository_impl;
mod entity;
mod repository;
mod send_mail_service_impl;
mod service;
mod session_service_impl;
mod use_case;
mod user_repository_impl;

use crate::bookmark_repository_impl::BookmarkRepositoryImpl;
use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::verify_user_secret::VerifyUserSecret;
use crate::send_mail_service_impl::SendMailServiceImpl;
use crate::session_service_impl::SessionServiceImpl;
use crate::use_case::create_bookmark_use_case::CreateBookmarkUseCase;
use crate::use_case::create_user_use_case::CreateUserUseCase;
use crate::use_case::verify_user_use_case::VerifyUserUseCase;
use crate::user_repository_impl::UserRepositoryImpl;
use anyhow::Result;

fn create_user() {
    let send_mail_service = SendMailServiceImpl::new();
    let user_repository = UserRepositoryImpl::new();
    let create_user_use_case = CreateUserUseCase::new(send_mail_service, user_repository);
    let mail_address = MailAddress::from_str("m@bouzuya.net").unwrap();
    let password = Password::from_str("password").unwrap();
    create_user_use_case.create_user(mail_address, password);
}

fn verify_user() -> Result<()> {
    let send_mail_service = SendMailServiceImpl::new();
    let user_repository = UserRepositoryImpl::new();
    let verify_user_use_case = VerifyUserUseCase::new(send_mail_service, user_repository);
    let verify_user_secret = VerifyUserSecret::from_str("verify-user-secret1").unwrap();
    verify_user_use_case.verify_user(verify_user_secret)
}

fn create_bookmark() -> Result<()> {
    let bookmark_repository = BookmarkRepositoryImpl::new();
    let session_service = SessionServiceImpl::new();
    let create_bookmark_use_case = CreateBookmarkUseCase::new(bookmark_repository, session_service);
    let url = BookmarkUrl::from_str("https://bouzuya.net").unwrap();
    let title = BookmarkTitle::from_str("bouzuya.net").unwrap();
    let comment = BookmarkComment::from_str("bouzuya's webpage").unwrap();
    create_bookmark_use_case.create_bookmark(url, title, comment)?;
    Ok(())
}

fn main() {
    create_user();
    verify_user().expect("verify user error");
    create_bookmark().expect("create bookmark");
}
