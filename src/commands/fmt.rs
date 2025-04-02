use std::{ fs::read_dir, path::PathBuf };

pub fn run(args: &clap::ArgMatches) {
    let mut parsed_args = vec![];
    let format_args = args
        .get_many::<String>("clang-format-args")
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    if let Some(path) = format_args.last() {
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
                            } else if ext == "h" {
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
    std::process::Command
        ::new("clang-format")
        .args(parsed_args)
        .status()
        .expect("Failed to run clang-format");
}
