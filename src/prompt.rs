use std::io::{self, Write};
use std::fs;
use std::fs::File;
use std::path::Path;
// use json::{self, Array};
// use json::object;
use serde_json::{json, Value};

fn get_user_input(field: String) -> String {
    print!("Enter {}: ", field);
    io::stdout().flush().unwrap();
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Read data failed");
    if data.len() == 0 {
        println!("Error: invalid data {}", field);
        return get_user_input(field);
    }
    data.remove(data.len() - 1);
    return data;
}

fn new_task() {
    let mut new_name = String::from("");
    let mut description = String::from("");
    let task_file = Path::new("in_progress.json");
    let mut name_used = false;

    new_name = get_user_input(String::from("new task name"));
    let content = fs::read_to_string(task_file).expect("Cannot open this file in_progress.json");
    let mut json_content: Value = serde_json::from_str(&content).unwrap(); 

    if let Some(array) = json_content["tasks"].as_array() {
        for task in array {
            if task["name"] == new_name {
                name_used = true;
                break;
            }
        }
    }
    if name_used {
        println!("This task already exists");
        return;
    }
    description = get_user_input(String::from("new task brief"));
    if let Some(tasks) = json_content["tasks"].as_array_mut() {
        tasks.push(json!({
            "name": new_name,
            "brief": description
        }));
    }
    let new_content = serde_json::to_string(&json_content).unwrap();
    let mut file = File::create(task_file).expect("Failed to open in progress.json");
    let display = task_file.display();

    match file.write_all(new_content.as_bytes()) {
        Err(why) => panic!("can't write to {display}:{why}"),
        Ok(_) => println!("successfully wrote to {}", display),
    };
    println!("\tNew task added successfully");
}

fn display_tasks() {
    let path = Path::new("in_progress.json");
    let content = fs::read_to_string(path).expect("Cannot open this file in_progress.json");
    let json_content: Value = serde_json::from_str(&content).unwrap(); 

    if let Some(array) = json_content["tasks"].as_array() {
        for task in array {
            println!(">>>>>>>>>>>>");
            println!("\tName: {}", task["name"]);
            println!("\tBrief: {}", task["brief"]);
            println!(">>>>>>>>>>>>");
        }
    }
}

fn remove_task() {
    let name = get_user_input(String::from("task name to removed"));
    let path = Path::new("in_progress.json");
    let content = fs::read_to_string(path).expect("Cannot open this file in_progress.json");
    let mut json_content: Value = serde_json::from_str(&content).unwrap(); 
    
    if let Some(array) = json_content["tasks"].as_array_mut() {
        if let Some(pos) = array.iter().position(|task| task["name"] == name) {
            array.remove(pos);
            println!("Task {} removed", name);
        } else {
            println!("Task {} not found", name);
        }
    }
    let new_content = serde_json::to_string(&json_content).unwrap();
    let mut file = File::create(path).expect("Failed to open in progress.json");
    let display = path.display();

    match file.write_all(new_content.as_bytes()) {
        Err(why) => panic!("can't write to {display}:{why}"),
        Ok(_) => println!("successfully wrote to {}", display),
    };
}

fn help() {
    let usage = "Welcome on Task track;
      new: Add a new task
      list: Get current tasks
      remove: Remove a task by its name
      exit: quit the program
    ";
    println!("{}", usage);
}

pub fn prompt_line() {
    loop {
        let data = get_user_input(String::from("a prompt"));
        // println!("\tYou entered {}", data);
        if data == "exit" {
            break;
        }
        if data == "new" {
            new_task();
            continue;
        }
        if data == "list" {
            display_tasks();
            continue;
        }
        if data == "remove" {
            remove_task();
            continue;
        }
        if data == "help" {
            help();
            continue;
        }
        println!("{}: not found", data);
    }
}
