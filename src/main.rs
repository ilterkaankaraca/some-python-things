use std::env;
use std::fs::{self, DirEntry};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "rfe" {
        replace_file_extension(&args[2]);
    } else if args[1] == "oksft" {
        only_keep_selected_file_type();
    }
}

fn replace_file_extension(file_extension: &str) {
    let mut file_names: Vec<DirEntry> = Vec::new();
    let path = std::env::current_dir().unwrap();
    //println!("{}", path.display());
    // println!("{}", Path::new(&path).exists());
    for entry in fs::read_dir(path).unwrap() {
        // println!("{}", entry.unwrap().path().display());
        if entry.as_ref().unwrap().path().is_file() {
            // println!("{}", entry.as_ref().unwrap().path().display());
            file_names.push(entry.unwrap());
        } else {
            continue;
        }
    }
    for i in file_names {
        println!("{}", i.path().file_stem().unwrap().to_str().unwrap());
        fs::rename(
            i.path().file_name().unwrap().to_str().unwrap(),
            format!(
                "{}.{}",
                i.path().file_stem().unwrap().to_str().unwrap(),
                file_extension.trim().to_string()
            ),
        ).ok();
    }
}


fn only_keep_selected_file_type() {
    println!("only_keep_selected_file_type");
}
