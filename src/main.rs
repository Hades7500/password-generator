use std::io::Write;
use std::{fs, io};

#[allow(while_true)]

fn main() {
    let _master_pwd: String = get_string("Enter your Master Password: ");
    while true {
        let mode: String = get_string("Do you want to add a new password or view existing ones? (add/view) ").to_lowercase();
        match mode.as_str() {
            "add" => add(),
            "view" => view(),
            "q" => {
                println!("Quitting...");
                return;
            }
            _ => println!("Invalid mode"),
        }
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
    let data: String = format!("{acc_name}: {pwd}");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./pwd.txt")
        .unwrap();

    writeln!(file, "{data}").expect("Unable to write to file")
}
fn view() {
    let data: String = fs::read_to_string("./pwd.txt").expect("Unable to read file");
    println!("{data}")
}
