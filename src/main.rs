#![allow(unused)]

mod data;
mod db;
mod todo;
mod utils;

use crate::todo::{add_todo, clear_todos, list_todos, remove_todo};
use clap::{Arg, ArgAction, Command};
use db::{data_repository::DataRepo, database::Db, repository::Repository};
use dotenv::dotenv;
use std::env;
use std::error::Error;

const APP_NAME: &str = "rust-todos";
const DATE_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

fn main() {
    dotenv().ok();

    let db_str = utils::get_db_string().expect("something went wrong when getting app data folder");

    let db = {
        match Db::connect(&db_str) {
            Ok(d) => d,
            Err(e) => panic!("{:?}", e),
        }
    };

    let args = parse_args();

    let load_fake_data = args.get_flag("fill");
    db.setup(load_fake_data);

    let data_repo = DataRepo::new(db);

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
        Some(("clear", _)) => {
            clear_todos(&data_repo);
        }
        _ => list_todos(&data_repo),
    }
}

fn parse_args() -> clap::ArgMatches {
    let matches = Command::new("todo")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Aidan Wallace aidanwallacedev@gmail.com")
        .about("Simple Rust Todo cli app")
        .arg(
            Arg::new("fill")
                .long("fill")
                .help("Fill in the details")
                .action(ArgAction::SetTrue),
        )
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
        .subcommand(Command::new("clear").about("Clear all todos"))
        .get_matches();

    matches
}
