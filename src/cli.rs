use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Spongebob";
    let status = "ready";

    //println!("Args: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Your status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}