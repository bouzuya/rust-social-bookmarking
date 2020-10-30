mod mail_address;
mod password;
mod send_mail_service;
mod send_mail_service_impl;
mod use_case;
mod user;
mod user_key;
mod user_repository;
mod user_repository_impl;
mod verify_user_secret;

use crate::mail_address::MailAddress;
use crate::password::Password;
use crate::send_mail_service_impl::SendMailServiceImpl;
use crate::use_case::create_user_use_case::CreateUserUseCase;
use crate::use_case::verify_user_use_case::VerifyUserUseCase;
use crate::user_repository_impl::UserRepositoryImpl;
use crate::verify_user_secret::VerifyUserSecret;
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

fn main() {
    create_user();
    verify_user().expect("verify user error");
}
