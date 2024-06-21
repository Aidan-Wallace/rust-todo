use crate::APP_NAME;
use home::home_dir;
use std::{env, io, path::PathBuf};

pub fn get_user_input(message: &str) -> String {
    println!("{}", message);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error reading input: {}", error);
            return String::new();
        }
    }

    input.trim().to_string()
}

/// Get the folder to store app data. Uses `TODO_DATABASE_STRING` if envvar is set.
/// If not it resorts to creating an app data folder in the users root directory.
/// If that fails, it uses the local directory
pub fn get_data_folder() -> std::io::Result<String> {
    let db_path = if let Ok(env_path) = env::var("TODO_DATABASE_STRING") {
        env_path
    } else {
        if let Some(home) = home_dir() {
            let mut new_dir = PathBuf::from(home);
            new_dir.push(format!(".{}", APP_NAME));

            std::fs::create_dir_all(&new_dir)?;

            new_dir.push("data.db");

            new_dir.into_os_string().into_string().unwrap()
        } else {
            "data.db".to_string()
        }
    };

    Ok(db_path)
}
