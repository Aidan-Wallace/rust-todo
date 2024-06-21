use chrono::{DateTime, Local, NaiveDateTime, ParseError, TimeZone};
use colored::Colorize;
use std::collections::HashSet;
use std::process::exit;

use crate::data::Data;
use crate::db::data_repository::DataRepo;
use crate::db::repository::Repository;
use crate::{utils, DATE_FORMAT};

pub fn list_todos(db: &DataRepo) {
    let data = match db.get_all() {
        Ok(d) => d,
        Err(e) => panic!("{:?}", e),
    };

    for (index, todo) in data.iter().enumerate() {
        let due_date = match todo.due_date_local(DATE_FORMAT) {
            Some(d) => format!(" | {}: {}", "due".blue(), d),
            None => " | ".to_string(),
        };

        let overdue = todo
            .due_date
            .map_or(false, |parsed_due_date| parsed_due_date < Local::now());

        println!(
            "{:>3} | {:<20} | {}: {}{:<27} | {}: {:<5} |",
            index.to_string().bold(),
            todo.name,
            "created".blue(),
            todo.date_added_local().format(DATE_FORMAT),
            due_date,
            "overdue".blue(),
            overdue
        );
    }

    exit(0)
}

pub fn add_todo(repo: &DataRepo, name: &str) {
    let mut todo = Data::new(1, name.to_string(), None);

    let mut running = true;
    while running {
        let input = utils::get_user_input("Would you like to add a due date? [Yn]");
        match input.to_lowercase().as_str() {
            "yes" | "y" | "Y" => {
                match parse_custom_date(&utils::get_user_input("Enter due date (yyyy/m/d h:m:s):"))
                {
                    Ok(parsed_date) => todo.set_due_date_from_local(parsed_date),
                    Err(e) => eprintln!("Failed to parse date string: {}", e),
                }

                running = false;
            }
            "no" | "n" | "N" => {
                running = false;
            }
            _ => {
                println!("Please enter 'yes'/'y' or 'no'/'n'.");
                continue;
            }
        }
    }

    match repo.insert(todo) {
        Ok(_) => (),
        Err(e) => panic!("there was an error saving todo to the database {}", e),
    }
}

pub fn remove_todo(repo: &DataRepo, parsed_integers: Vec<i32>) {
    let indexes_set: HashSet<usize> = parsed_integers.iter().map(|&i| i as usize).collect();

    let data = match repo.get_all() {
        Ok(d) => d,
        Err(e) => panic!("{:?}", e),
    };

    for i in indexes_set {
        if i <= data.len() - 1 {
            let _ = repo.remove_by_id(data[i].id);
        } else {
            println!("skipping todo at index {} as it is out of range", i);
        }
    }
}

pub fn clear_todos(repo: &DataRepo) {
    repo.remove_all();
}

fn parse_custom_date(date_str: &str) -> Result<DateTime<Local>, ParseError> {
    let naive_date_time = NaiveDateTime::parse_from_str(date_str, DATE_FORMAT)?;
    Ok(Local.from_utc_datetime(&naive_date_time))
}
