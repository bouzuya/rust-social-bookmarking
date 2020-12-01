mod env;

use crate::cli::env::CliEnv;
use crate::use_case::*;
use anyhow::Result;

pub fn run() -> Result<()> {
    let env = CliEnv::new();
    let matches = clap::App::new("rust-social-bookmarking")
        .subcommand(
            clap::SubCommand::with_name("create-user")
                .about("create-user")
                .arg(clap::Arg::with_name("SECRET").help("secret").required(true)),
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
            env.create_user_use_case().create_user(secret)?;
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
            env.sign_in_use_case().sign_in(&mail_address, &password)?;
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
            env.sign_up_use_case().sign_up(mail_address, password)?;
        }
        _ => {}
    }
    Ok(())
}
