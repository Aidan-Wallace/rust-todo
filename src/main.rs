#![allow(unused)]

mod data;
mod db;
mod todo;
mod utils;

use crate::todo::{add_todo, list_todos, remove_todo};
use clap::{Arg, Command};
use db::{data_repository::DataRepo, database::Db, repository::Repository};
use dotenv::dotenv;
use std::env;

const DATE_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

fn main() {
    dotenv().ok();
    let db_str = env::var("TODO_DATABASE_STRING").expect("TODO_DATABASE_STRING must be set.");

    let db = {
        match Db::connect(&db_str) {
            Ok(d) => d,
            Err(e) => panic!("{:?}", e),
        }
    };
    let _ = db.setup(true);

    let data_repo = DataRepo::new(db);

    let args = parse_args();
    match args.subcommand() {
        Some(("add", sub_matches)) => {
            let task: String = sub_matches
                .get_many::<String>("TASK")
                .unwrap()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(" ");

            add_todo(&data_repo, &task);
        }
        Some(("done", sub_matches)) => {
            let task = sub_matches
                .get_many::<String>("TASK")
                .unwrap()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>();

            let parsed_integers: Vec<i32> = task
                .iter()
                .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
                .collect();

            remove_todo(&data_repo, parsed_integers);
        }
        _ => list_todos(&data_repo),
    }
}

fn parse_args() -> clap::ArgMatches {
    let matches = Command::new("todo")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Aidan Wallace aidanwallacedev@gmail.com")
        .about("Simple Rust Todo cli app")
        .subcommand(
            Command::new("add").about("Add a new task").arg(
                Arg::new("TASK")
                    .help("The task to add")
                    .num_args(1..)
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("done").about("Mark a task as done").arg(
                Arg::new("TASK")
                    .help("The task to mark as done")
                    .num_args(1..)
                    .required(true),
            ),
        )
        .get_matches();

    matches
}
