use std::io;

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
