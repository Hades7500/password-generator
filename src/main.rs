use std::{io, fs};
use std::io::Write;

#[allow(unused)]
#[allow(dead_code)]

fn main() {
    println!("Hello, world!");
    let master_pwd: String = get_string("Enter your Master Password: ");
    let mode = get_string("Do you want to add a new password or view existing ones? (add/view) ");
    match mode.as_str() {
        "add" => add(),
        "view" => view(),
        "q" => {println!("Quitting...");
                return},
        _ => println!("Invalid mode"),
    }
}

fn get_string(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_owned()
}

fn add() {
    let acc_name: String = get_string("Account Name: ");
    let pwd: String = get_string("Password: ");
    write(acc_name, pwd)
}

fn write(acc_name: String, pwd: String) {
    let data = format!("{acc_name}: {pwd}");

    let mut file = fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open("./pwd.txt")
    .unwrap();

    writeln!(file , "{data}").expect("Unable to write to file")
}
#[allow(unused)]
#[allow(dead_code)]
fn view() {
    let data: String = fs::read_to_string("./pwd.txt").expect("Unable to read file");
    println!("{data}")
}