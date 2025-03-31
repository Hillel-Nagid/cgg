use std::{ fs::read_dir, path::PathBuf };

const DEFAULT_ARGS: [&'static str; 5] = ["-Wextra", "-Wall", "-Wvla", "-lm", "-std=c99"];
pub fn parse_args(args: &clap::ArgMatches) -> Vec<String> {
    let mut parsed_args: Vec<String> = vec![];
    if let Some(default) = args.get_one::<bool>("default") {
        if *default {
            for arg in DEFAULT_ARGS.iter() {
                parsed_args.push(String::from(*arg));
            }
        }
    }
    let build_args = args.get_many::<String>("gcc_args").into_iter().flatten().collect::<Vec<_>>();
    if let Some(path) = build_args.last() {
        let path = PathBuf::from(path);
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
    } else {
        eprintln!("No files to compile");
        std::process::exit(1);
    }
    for arg in build_args[..build_args.len() - 1].iter() {
        println!("Compiling {}", arg);
        parsed_args.push((*arg).clone());
    }
    parsed_args
}

pub fn run(args: &clap::ArgMatches) {
    let parsed_args = parse_args(args);
    let result = std::process::Command::new("gcc").args(parsed_args).status().unwrap();
    if !result.success() {
        eprintln!("Failed to run GCC");
        std::process::exit(1);
    }
}
