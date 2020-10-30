mod create_user_use_case;
mod mail_address;
mod password;
mod send_mail_service;
mod send_mail_service_impl;
mod user;
mod user_dao;
mod user_key;
mod user_repository;
mod user_repository_impl;
mod verify_user_secret;

use crate::create_user_use_case::CreateUserUseCase;
use crate::mail_address::MailAddress;
use crate::password::Password;
use crate::send_mail_service_impl::SendMailServiceImpl;
use crate::user_dao::UserDao;
use crate::user_repository_impl::UserRepositoryImpl;

fn main() {
    let send_mail_service = SendMailServiceImpl::new();
    let user_dao = UserDao::new();
    let user_repository = UserRepositoryImpl::new(user_dao);
    let create_user_use_case = CreateUserUseCase::new(send_mail_service, user_repository);
    let mail_address = MailAddress::from_str("m@bouzuya.net").unwrap();
    let password = Password::from_str("password").unwrap();
    create_user_use_case.create_user(mail_address, password);
}
