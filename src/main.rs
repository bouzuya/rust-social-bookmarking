#[macro_use]
extern crate diesel;

mod app;
mod cli;
mod entity;
mod fake;
mod pg;
mod repository;
mod schema;
mod service;
mod use_case;
mod web;

use crate::cli::run as cli_run;
use crate::fake::run as fake_run;
use crate::web::run as web_run;
use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    if true {
        web_run().await
    } else if false {
        cli_run()
    } else {
        fake_run();
        Ok(())
    }
}
