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

use crate::cli::run as cli_run;
use crate::fake::run as fake_run;

fn main() {
    if true {
        cli_run().expect("error");
    } else {
        fake_run();
    }
}
