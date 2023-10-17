use std::env;
use std::path::Path;
use std::path::PathBuf;
use clap::{Command, Arg, ArgAction};
mod colors;
use colors::Color;
use std::collections::HashMap;
use serde::Serialize;
use std::fs;
//use colored::*;

#[derive(Debug, Serialize)]
enum Node {
    Directory(HashMap<String, Box<Node>>),
    File(String),
}

fn convert_to_json_structure(path: &Path) -> Node {
    if path.is_dir() {
        let mut children = HashMap::new();
        for entry in fs::read_dir(path).expect("Failed to read directory") {
            let entry_path = entry.expect("Failed to read entry").path();
            let filename = entry_path.file_name().and_then(|name| name.to_str()).unwrap_or("").to_string();
            if filename != ".DS_Store" && filename != "target" && filename != "env" {
                children.insert(filename, Box::new(convert_to_json_structure(&entry_path)));
            }
        }
        Node::Directory(children)
    } else {
        Node::File(path.to_str().unwrap_or("").to_string())
    }
}

fn print_tree(path: &Path, prefix: &str, is_last: bool) {
    let name = match path.file_name() {
        Some(filename) => {
            if filename.to_str() == Some(".") {
                match env::current_dir() {
                    Ok(curr_dir) => curr_dir.file_name().and_then(|name| name.to_str()).unwrap_or("").to_string(),
                    Err(_) => String::new(),
                }
            } else {
                filename.to_str().unwrap_or("").to_string()
            }
        },
        None => String::new(),
    };

    let colored_name = if path.is_dir() && prefix.is_empty() {
        Color::Blue.paint(&name)
    } else if path.is_dir() {
        Color::Yellow.paint(&name)
    } else {
        name.to_string()
    };

    if prefix.is_empty() {
        println!("{}", colored_name);
    } else {
        let curr_prefix = if is_last { "└── " } else { "├── " };
        println!("{}{}{}", prefix, curr_prefix, colored_name);
    }

    if path.is_dir() {
        let entries: Vec<_> = path.read_dir().unwrap()
            .map(|res| res.unwrap().path())
            .filter(|entry| {
                let filename = entry.file_name().unwrap().to_str().unwrap();
                filename != ".DS_Store" && filename != "target" && filename != "env"
            })
            .collect();

        for (index, entry) in entries.iter().enumerate() {
            let next_is_last = index == entries.len() - 1;
            let next_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            print_tree(&entry, &next_prefix, next_is_last); 
        }
    }
}

fn main() {
    let matches = Command::new("rtree")
        .version("1.0")
        .about("its like tree but in rust")
        .arg(Arg::new("DIRECTORY")
            .short('d')
            .long("directory")
            .value_name("DIRECTORY")
            .help("Sets a custom directory")
            .default_value("."))
        .arg(Arg::new("json")
            .short('j')
            .long("json")
            .help("Outputs in JSON format")
            .action(ArgAction::SetTrue))
        .get_matches();

    let path_str: String = matches.get_one::<String>("DIRECTORY")
        .map(|s| if s == "." {
            env::current_dir().unwrap().to_str().unwrap().to_string()
        } else {
            s.to_string()
        })
        .unwrap_or_else(|| String::from("."));

    let path = PathBuf::from(path_str);

    if matches.get_flag("json") {
        let tree = convert_to_json_structure(&path);
        let json = serde_json::to_string_pretty(&tree).expect("Failed to serialize to JSON");
        println!("{}", json);
    } else {
        print_tree(&path, "", true);
    }
}


    