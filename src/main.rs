mod entity;
mod fake;
mod repository;
mod service;
mod use_case;

use crate::fake::fake_env::FakeEnv;
use crate::use_case::create_bookmark_use_case::{CreateBookmarkUseCase, UseCreateBookmarkUseCase};
use crate::use_case::create_credential_use_case::{
    CreateCredentialUseCase, UseCreateCredentialUseCase,
};
use crate::use_case::create_user_use_case::{CreateUserUseCase, UseCreateUserUseCase};
use crate::use_case::update_bookmark_use_case::UseUpdateBookmarkUseCase;
use anyhow::Result;

fn create_credential<T: UseCreateCredentialUseCase>(env: &T) -> Result<()> {
    let mail_address = "m@bouzuya.net".parse().unwrap();
    let password = "password".parse().unwrap();
    env.create_credential_use_case()
        .create_credential(mail_address, password)
}

fn create_user<T: UseCreateUserUseCase>(env: &T) -> Result<()> {
    let verify_user_secret = "1".repeat(255).parse().unwrap();
    env.create_user_use_case().create_user(verify_user_secret)
}

fn create_bookmark<T: UseCreateBookmarkUseCase>(env: &T) -> Result<()> {
    let url = "https://bouzuya.net".parse().unwrap();
    let title = "bouzuya.net".parse().unwrap();
    let comment = "bouzuya's webpage".parse().unwrap();
    env.create_bookmark_use_case()
        .create_bookmark(url, title, comment)
}

fn update_bookmark<T: UseUpdateBookmarkUseCase>(_: &T) -> Result<()> {
    todo!()
}

fn main() {
    let env = FakeEnv::new();
    create_credential(&env).expect("create credential");
    create_user(&env).expect("create user error");
    create_bookmark(&env).expect("create bookmark");
    update_bookmark(&env).expect("update bookmark");
}
