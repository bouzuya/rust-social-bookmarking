mod fs_session_service;

use crate::cli::fs_session_service::FsSessionService;
use crate::fake::send_mail_service_impl::SendMailServiceImpl;
use crate::pg::*;
use anyhow::Result;
use diesel::{Connection, PgConnection};
use std::sync::Arc;

pub fn run() -> Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let connection = Arc::new(connection);
    let user_repository = Arc::new(PgUserRepository::new(connection.clone()));
    let app = crate::app::App::new(
        Arc::new(PgBookmarkRepository::new(connection.clone())),
        Arc::new(PgCredentialRepository::new(connection.clone())),
        Arc::new(SendMailServiceImpl::new()),
        Arc::new(FsSessionService::new(user_repository.clone())),
        user_repository,
    );
    let matches = clap::App::new("rust-social-bookmarking")
        .subcommand(
            clap::SubCommand::with_name("create-user")
                .about("create-user")
                .arg(clap::Arg::with_name("SECRET").help("secret").required(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("create-bookmark")
                .about("create-bookmark")
                .arg(clap::Arg::with_name("URL").help("url").required(true))
                .arg(clap::Arg::with_name("TITLE").help("title").required(true))
                .arg(
                    clap::Arg::with_name("COMMENT")
                        .help("comment")
                        .required(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("delete-bookmark")
                .about("delete-bookmark")
                .arg(
                    clap::Arg::with_name("BOOKMARK_KEY")
                        .help("bookmark key")
                        .required(true),
                ),
        )
        .subcommand(clap::SubCommand::with_name("get-current-user").about("get-current-user"))
        .subcommand(
            clap::SubCommand::with_name("list-bookmarks-by-user-key")
                .about("list-bookmarks-by-user-key")
                .arg(
                    clap::Arg::with_name("USER_KEY")
                        .help("user-key")
                        .required(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("list-current-user-bookmarks")
                .about("list-current-user-bookmarks"),
        )
        .subcommand(
            clap::SubCommand::with_name("sign-in")
                .about("sign-in")
                .arg(
                    clap::Arg::with_name("MAIL_ADDRESS")
                        .help("mail address")
                        .required(true),
                )
                .arg(
                    clap::Arg::with_name("PASSWORD")
                        .help("password")
                        .required(true),
                ),
        )
        .subcommand(clap::SubCommand::with_name("sign-out").about("sign-out"))
        .subcommand(
            clap::SubCommand::with_name("sign-up")
                .about("sign-up")
                .arg(
                    clap::Arg::with_name("MAIL_ADDRESS")
                        .help("mail address")
                        .required(true),
                )
                .arg(
                    clap::Arg::with_name("PASSWORD")
                        .help("password")
                        .required(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("update-bookmark")
                .about("update-bookmark")
                .arg(
                    clap::Arg::with_name("BOOKMARK_KEY")
                        .help("bookmark key")
                        .required(true),
                )
                .arg(clap::Arg::with_name("URL").help("url").required(true))
                .arg(clap::Arg::with_name("TITLE").help("title").required(true))
                .arg(
                    clap::Arg::with_name("COMMENT")
                        .help("comment")
                        .required(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("update-mail-address")
                .about("update-mail-address")
                .arg(
                    clap::Arg::with_name("MAIL_ADDRESS")
                        .help("mail address")
                        .required(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("update-password")
                .about("update-password")
                .arg(
                    clap::Arg::with_name("PASSWORD")
                        .help("password")
                        .required(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("verify-mail-address")
                .about("verify-mail-address")
                .arg(clap::Arg::with_name("SECRET").help("secret").required(true)),
        )
        .get_matches();
    match matches.subcommand() {
        ("create-bookmark", Some(sub_matches)) => {
            let url = sub_matches
                .value_of("URL")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            let title = sub_matches
                .value_of("TITLE")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            let comment = sub_matches
                .value_of("COMMENT")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.create_bookmark_use_case()
                .create_bookmark(url, title, comment)?;
        }
        ("create-user", Some(sub_matches)) => {
            let secret = sub_matches
                .value_of("SECRET")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.create_user_use_case().create_user(secret)?;
        }
        ("delete-bookmark", Some(sub_matches)) => {
            let bookmark_key = sub_matches
                .value_of("BOOKMARK_KEY")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.delete_bookmark_use_case()
                .delete_bookmark(&bookmark_key)?;
        }
        ("get-current-user", Some(_)) => {
            let current_user = app.get_current_user_use_case().get_current_user()?;
            println!("{:?}", current_user);
        }
        ("list-bookmarks-by-user-key", Some(sub_matches)) => {
            let user_key = sub_matches
                .value_of("USER_KEY")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            let bookmarks = app
                .list_bookmarks_by_user_key_use_case()
                .list_bookmarks_by_user_key(&user_key)?;
            println!("{:?}", bookmarks);
        }
        ("list-current-user-bookmarks", Some(_)) => {
            let bookmarks = app
                .list_current_user_bookmarks_use_case()
                .list_current_user_bookmarks()?;
            println!("{:?}", bookmarks);
        }
        ("sign-in", Some(sub_matches)) => {
            let mail_address = sub_matches
                .value_of("MAIL_ADDRESS")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            let password = sub_matches
                .value_of("PASSWORD")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.sign_in_use_case().sign_in(&mail_address, &password)?;
        }
        ("sign-out", Some(_)) => {
            app.sign_out_use_case().sign_out()?;
        }
        ("sign-up", Some(sub_matches)) => {
            let mail_address = sub_matches
                .value_of("MAIL_ADDRESS")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            let password = sub_matches
                .value_of("PASSWORD")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.sign_up_use_case().sign_up(mail_address, password)?;
        }
        ("update-bookmark", Some(sub_matches)) => {
            let bookmark_key = sub_matches
                .value_of("BOOKMARK_KEY")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            let url = sub_matches
                .value_of("URL")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            let title = sub_matches
                .value_of("TITLE")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            let comment = sub_matches
                .value_of("COMMENT")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.update_bookmark_use_case()
                .update_bookmark(bookmark_key, url, title, comment)?;
        }
        ("update-mail-address", Some(sub_matches)) => {
            let mail_address = sub_matches
                .value_of("MAIL_ADDRESS")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.update_mail_address_use_case()
                .update_mail_address(&mail_address)?;
        }
        ("update-password", Some(sub_matches)) => {
            let password = sub_matches
                .value_of("PASSWORD")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.update_password_use_case().update_password(&password)?;
        }
        ("verify-mail-address", Some(sub_matches)) => {
            let secret = sub_matches
                .value_of("SECRET")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.verify_mail_address_use_case()
                .verify_mail_address(&secret)?;
        }
        _ => {}
    }
    Ok(())
}
