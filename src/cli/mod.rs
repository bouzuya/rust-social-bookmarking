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
        .subcommand(clap::SubCommand::with_name("get-current-user").about("get-current-user"))
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
        .get_matches();
    match matches.subcommand() {
        ("create-user", Some(sub_matches)) => {
            let secret = sub_matches
                .value_of("SECRET")
                .unwrap()
                .parse()
                .map_err(anyhow::Error::msg)?;
            app.create_user_use_case().create_user(secret)?;
        }
        ("get-current-user", Some(_)) => {
            let current_user = app.get_current_user_use_case().get_current_user()?;
            println!("{:?}", current_user);
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
        _ => {}
    }
    Ok(())
}
