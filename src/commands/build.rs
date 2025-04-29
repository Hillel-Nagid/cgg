use std::{ fs::read_dir, path::PathBuf };

use super::update_default;
const DEFAULT_ARGS: [&'static str; 5] = ["-Wextra", "-Wall", "-Wvla", "-lm", "-std=c99"];
pub fn parse_args(args: &clap::ArgMatches) -> Vec<String> {
    let mut parsed_args: Vec<String> = vec![];
    let default = args.get_flag("default");
    if default {
        let default_flags: String = update_default
            ::get_default_flags()
            .unwrap_or(DEFAULT_ARGS.join(" "));
        let default_flags = default_flags.split_whitespace().collect::<Vec<_>>();
        for flag in default_flags {
            parsed_args.push(flag.to_string());
        }
    }
    if let Some(build_args) = args.get_many::<String>("gcc_args") {
        let last_arg = build_args.clone().last().unwrap();
        for arg in build_args {
            if arg == last_arg {
                let path = PathBuf::from(arg.clone());
                if path.is_dir() {
                    for entry in read_dir(path).unwrap() {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(extension) = path.extension() {
                                if let Some(ext) = extension.to_str() {
                                    if ext == "c" {
                                        parsed_args.push(path.to_str().unwrap().to_string());
                                    }
                                }
                            }
                        }
                    }
                } else {
                    parsed_args.push(path.to_str().unwrap().to_string());
                }
                break;
            } else {
                parsed_args.push(arg.clone());
            }
        }
    } else {
        eprintln!("No GCC arguments provided");
        std::process::exit(1);
    }
    println!("Parsed arguments: {:?}", parsed_args);
    if parsed_args.is_empty() {
        eprintln!("No arguments to pass to GCC");
        std::process::exit(1);
    }
    parsed_args
}

pub fn run(args: &clap::ArgMatches) {
    let mut parsed_args = parse_args(args);
    if !parsed_args.contains(&String::from("-o")) {
        if let Some(current_dir_name) = std::env::current_dir().unwrap().file_name() {
            let output_path = format!("{}.out", current_dir_name.to_str().unwrap());
            parsed_args.insert(0, String::from("-o"));
            parsed_args.insert(1, output_path);
        };
    }
    let result = std::process::Command::new("gcc").args(parsed_args).status().unwrap();
    if !result.success() {
        eprintln!("Failed to run GCC");
        std::process::exit(1);
    }
}
