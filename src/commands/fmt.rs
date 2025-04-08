use std::{ fs::{ self, read_dir }, path::PathBuf, process::Stdio };

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
    if parsed_args.is_empty() {
        eprintln!("No files to format");
        std::process::exit(1);
    }
    for path in parsed_args.iter() {
        println!("Formatting {}...", path);
        let mut command = std::process::Command::new("clang-format");
        command.args(vec![path]).stdout(Stdio::piped());

        let process = command.spawn().expect("Failed to run clang-format");
        let output = process.wait_with_output().expect("Failed to get output");
        if output.status.success() {
            let formatted_code: String = String::from_utf8(output.stdout).expect(
                "Failed to run clang-format"
            );
            fs::write(path, formatted_code).expect("Failed to write formatted code");
        } else {
            let error_message = String::from_utf8(output.stderr).expect(
                "Failed to run clang-format"
            );
            eprintln!("clang-format failed:\n{}", error_message);
        }
    }
}
