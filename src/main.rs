use std::io::{self, Write};
// use json;
// use json::object::Object;
use std::fs::File;
use std::path::Path;
use json::object;
// use std::io::Write;
use prompt::*;
mod prompt;
// pub user prompt::promptLine;

fn check_config() {
    let mut user = object! {
        name: null,
    };
    let tasks = object! {
        "tasks": []
    };
    let path = Path::new("config.json");
    let in_progress = Path::new("in_progress.json");
    // let done = Path::new("done.json");
    let display = path.display();
    if path.exists() == false {
        let mut data = get_user_name();
        data.remove(data.len() - 1);
        let mut file = match File::create(&path) {
            Err(why) => panic!("Can't create {}:{}", display, why),
            Ok(file) => file
        };
        user["name"] = data.into();
        match file.write_all(user.to_string().as_bytes()) {
            Err(why) => panic!("can't write to {display}:{why}"),
            Ok(_) => println!("successfully wrote to {}", display),
        };

        // Create utils files
        let mut _second_file = match File::create(&in_progress) {
            Err(why) => panic!("Can't create {}:{}", in_progress.display(), why),
            Ok(file) => file
        };
        match _second_file.write_all(tasks.to_string().as_bytes()) {
            Err(why) => panic!("can't write to in_progress.json:{why}"),
            Ok(_) => println!("Successfully wrote to in_progress.json")
        }
        prompt_line();
    } else {
        prompt_line();
    }
}


fn get_user_name() -> String {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Read name failed");
    if name.len() == 0 {
        println!("Error: invalid name");
        return get_user_name();
    }
    return name;
}

fn welcome() {
    println!("Welcome on Task track");
}



fn main() {
    welcome();
    check_config();
}
