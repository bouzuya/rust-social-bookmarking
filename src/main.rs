mod entity;
mod fake;
mod repository;
mod service;
mod use_case;

use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::verify_user_secret::VerifyUserSecret;
use crate::fake::fake_env::FakeEnv;
use crate::use_case::create_bookmark_use_case::{CreateBookmarkUseCase, UseCreateBookmarkUseCase};
use crate::use_case::create_user_use_case::{CreateUserUseCase, UseCreateUserUseCase};
use crate::use_case::verify_user_use_case::{UseVerifyUserUseCase, VerifyUserUseCase};
use anyhow::Result;

fn create_user<T: UseCreateUserUseCase>(env: &T) -> Result<()> {
    let mail_address = "m@bouzuya.net".parse::<MailAddress>().unwrap();
    let password = Password::from_str("password").unwrap();
    env.create_user_use_case()
        .create_user(mail_address, password)
}

fn verify_user<T: UseVerifyUserUseCase>(env: &T) -> Result<()> {
    let verify_user_secret = VerifyUserSecret::from_str("verify-user-secret1").unwrap();
    env.verify_user_use_case().verify_user(verify_user_secret)
}

fn create_bookmark<T: UseCreateBookmarkUseCase>(env: &T) -> Result<()> {
    let url = BookmarkUrl::from_str("https://bouzuya.net").unwrap();
    let title = BookmarkTitle::from_str("bouzuya.net").unwrap();
    let comment = "bouzuya's webpage".parse::<BookmarkComment>().unwrap();
    env.create_bookmark_use_case()
        .create_bookmark(url, title, comment)
}

fn main() {
    let env = FakeEnv::new();
    create_user(&env).expect("create user");
    verify_user(&env).expect("verify user error");
    create_bookmark(&env).expect("create bookmark");
}
