use std::{env, io::{self, Write}, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "rfe" {
        replace_file_extension();
    } else if args[1] == "oksft" {
        only_keep_selected_file_type();
    }
}

fn replace_file_extension() {
    print!("Please provide a path: ");
    io::stdout().flush().unwrap();
    let mut path = String::new();
    std::io::stdin().read_line(&mut path).unwrap();
    println!("{}", path); 
   //TODO: check if path is valid
}

fn only_keep_selected_file_type() {
    println!("only_keep_selected_file_type");
}
