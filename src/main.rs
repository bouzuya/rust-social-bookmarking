mod entity;
mod fake;
mod repository;
mod service;
mod use_case;

use crate::fake::fake_env::FakeEnv;
use crate::use_case::create_bookmark_use_case::{CreateBookmarkUseCase, UseCreateBookmarkUseCase};
use crate::use_case::create_user_use_case::{CreateUserUseCase, UseCreateUserUseCase};
use crate::use_case::delete_bookmark_use_case::{DeleteBookmarkUseCase, UseDeleteBookmarkUseCase};
use crate::use_case::delete_user_use_case::UseDeleteUserUseCase;
use crate::use_case::get_current_user_use_case::{GetCurrentUserUseCase, UseGetCurrentUserUseCase};
use crate::use_case::list_bookmarks_use_case::{ListBookmarksUseCase, UseListBookmarksUseCase};
use crate::use_case::reset_password_use_case::UseResetPasswordUseCase;
use crate::use_case::sign_in_use_case::{SignInUseCase, UseSignInUseCase};
use crate::use_case::sign_out_use_case::UseSignOutUseCase;
use crate::use_case::sign_up_use_case::{SignUpUseCase, UseSignUpUseCase};
use crate::use_case::update_bookmark_use_case::{UpdateBookmarkUseCase, UseUpdateBookmarkUseCase};
use crate::use_case::update_mail_address_use_case::{
    UpdateMailAddressUseCase, UseUpdateMailAddressUseCase,
};
use crate::use_case::update_password_by_secret_use_case::UseUpdatePasswordBySecretUseCase;
use crate::use_case::update_password_use_case::{UpdatePasswordUseCase, UseUpdatePasswordUseCase};
use crate::use_case::verify_mail_address_use_case::{
    UseVerifyMailAddressUseCase, VerifyMailAddressUseCase,
};
use anyhow::Result;

fn create_bookmark<T: UseCreateBookmarkUseCase>(env: &T) -> Result<()> {
    let url = "https://bouzuya.net".parse().unwrap();
    let title = "bouzuya.net".parse().unwrap();
    let comment = "bouzuya's webpage".parse().unwrap();
    env.create_bookmark_use_case()
        .create_bookmark(url, title, comment)
}

fn create_user<T: UseCreateUserUseCase>(env: &T) -> Result<()> {
    let verify_user_secret = "1".repeat(255).parse().unwrap();
    env.create_user_use_case().create_user(verify_user_secret)
}

fn delete_bookmark<T: UseDeleteBookmarkUseCase>(env: &T) -> Result<()> {
    let bookmark_key = "1234567890123456".parse().unwrap();
    env.delete_bookmark_use_case()
        .delete_bookmark(&bookmark_key)
}

fn delete_user<T: UseDeleteUserUseCase>(_: &T) -> Result<()> {
    todo!()
}

fn get_current_user<T: UseGetCurrentUserUseCase>(env: &T) -> Result<()> {
    let current_user = env.get_current_user_use_case().get_current_user()?;
    println!("{:?}", current_user);
    Ok(())
}

fn list_bookmarks<T: UseListBookmarksUseCase>(env: &T) -> Result<()> {
    let bookmarks = env.list_bookmarks_use_case().list_bookmarks()?;
    println!("{:?}", bookmarks);
    Ok(())
}

fn reset_password<T: UseResetPasswordUseCase>(_: &T) -> Result<()> {
    todo!()
}

fn sign_in<T: UseSignInUseCase>(env: &T) -> Result<()> {
    let mail_address = "m@bouzuya.net".parse().unwrap();
    let password = "password".parse().unwrap();
    env.sign_in_use_case().sign_in(&mail_address, &password)
}

fn sign_out<T: UseSignOutUseCase>(_: &T) -> Result<()> {
    todo!()
}

fn sign_up<T: UseSignUpUseCase>(env: &T) -> Result<()> {
    let mail_address = "m@bouzuya.net".parse().unwrap();
    let password = "password".parse().unwrap();
    env.sign_up_use_case().sign_up(mail_address, password)
}

fn update_bookmark<T: UseUpdateBookmarkUseCase>(env: &T) -> Result<()> {
    let bookmark_key = "1234567890123456".parse().unwrap();
    let bookmark_url = "https://bouzuya.net".parse().unwrap();
    let bookmark_title = "bouzuya.net".parse().unwrap();
    let bookmark_comment = "bouzuya's website".parse().unwrap();
    env.update_bookmark_use_case().update_bookmark(
        bookmark_key,
        bookmark_url,
        bookmark_title,
        bookmark_comment,
    )
}

fn update_mail_address<T: UseUpdateMailAddressUseCase>(env: &T) -> Result<()> {
    let mail_address = "m2@bouzuya.net".parse().unwrap();
    env.update_mail_address_use_case()
        .update_mail_address(&mail_address)
}

fn update_password<T: UseUpdatePasswordUseCase>(env: &T) -> Result<()> {
    let password = "password1".parse().unwrap();
    env.update_password_use_case().update_password(&password)
}

fn update_password_by_secret<T: UseUpdatePasswordBySecretUseCase>(_: &T) -> Result<()> {
    todo!()
}

fn verify_mail_address<T: UseVerifyMailAddressUseCase>(env: &T) -> Result<()> {
    let verify_user_secret = "1".repeat(255).parse().unwrap();
    env.verify_mail_address_use_case()
        .verify_mail_address(&verify_user_secret)
}

fn main() {
    let env = FakeEnv::new();
    sign_up(&env).expect("sign up");
    create_user(&env).expect("create user");
    sign_in(&env).expect("sign in");
    get_current_user(&env).expect("get current user");
    create_bookmark(&env).expect("create bookmark");
    list_bookmarks(&env).expect("list bookmarks");
    // TODO: add list_bookmarks_by_user_key
    update_bookmark(&env).expect("update bookmark");
    delete_bookmark(&env).expect("delete bookmark");
    update_mail_address(&env).expect("update mail_address");
    verify_mail_address(&env).expect("verify mail_address");
    update_password(&env).expect("update password");
    sign_out(&env).expect("sign out");
    reset_password(&env).expect("reset password");
    update_password_by_secret(&env).expect("update password by secret");
    sign_in(&env).expect("sign in (2)");
    delete_user(&env).expect("delete user");
}
