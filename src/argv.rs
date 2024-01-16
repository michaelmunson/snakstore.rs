use std::{collections::HashMap, env};

pub fn get_map(commands:Vec<&str>) -> HashMap<String, String> {
    let mut argv_map:HashMap<String, String> = HashMap::new();
    let argv: Vec<String> = env::args().collect();
    let mut last_arg = String::new();

    for i in 1..argv.len() {
        let arg = (&argv[i]).trim().to_string();
        if arg.starts_with("--") {
            if arg.contains("="){
                if let Some((first,second)) = arg.split_once("=") {
                    argv_map.insert(first.to_string(), second.to_string());
                }
            } else {
                argv_map.insert(arg.to_string(), String::from(""));
                last_arg = arg.to_string();
            }
        }
        else if arg.starts_with("-") {
            argv_map.insert(arg.to_string(), String::from(""));
            last_arg = arg.to_string();
        }
        else if commands.contains(&arg.as_str()) {
            argv_map.insert(arg.to_string(), String::from(""));
            last_arg = arg.to_string();
        }
        else {
            if last_arg.len() == 0 {
                // println!("Arg: {arg} ... Last Arg: {last_arg}");
                argv_map.insert(arg.to_string(), String::from(""));
                last_arg = arg.to_string();
            }
            else {
                // println!("Arg: {arg} ... Last Arg: {last_arg}");
                argv_map.insert(last_arg, arg.to_string());
                last_arg = String::from("");
            }
        }
    }

    return argv_map;
}   