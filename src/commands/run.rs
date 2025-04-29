use super::build;
use uuid::Uuid;
use std::fs::{ create_dir, remove_dir_all };
use std::path;
use std::process::Stdio;

pub fn run(args: &clap::ArgMatches) {
    let mut program_args: String = String::new();
    let uuid = Uuid::new_v4().to_string();
    create_dir(path::Path::new("./.tmp")).unwrap();
    let tmp_path = path::Path
        ::join(path::Path::new("./.tmp"), path::Path::new(&uuid))
        .to_string_lossy()
        .to_string();
    let mut parsed_args = build::parse_args(args);
    if parsed_args.contains(&String::from("--program_args")) {
        let extra_args_index = parsed_args
            .iter()
            .position(|s| s == "--program_args")
            .unwrap();
        let extra_args = parsed_args[extra_args_index + 1].clone();
        program_args = extra_args.clone();
        parsed_args.remove(extra_args_index);
        parsed_args.remove(extra_args_index);
    }
    if !parsed_args.contains(&String::from("-o")) {
        parsed_args.insert(0, String::from("-o"));
        parsed_args.insert(1, tmp_path.clone());
    } else if let Some(output_index) = parsed_args.iter().position(|s| s == "-o") {
        parsed_args.splice(
            output_index..output_index + 1,
            vec![String::from("-o"), tmp_path.clone()]
        );
    }

    let result = std::process::Command::new("gcc").args(parsed_args).status().unwrap();
    if !result.success() {
        remove_dir_all(path::Path::new("./tmp")).unwrap();
        eprintln!("Failed to run GCC");
        std::process::exit(1);
    }
    let mut command = std::process::Command::new(&tmp_path);
    command
        .args(program_args.split(",").collect::<Vec<_>>())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let result = command.status().unwrap();
    let output = command.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stdout.is_empty() {
        println!("Output: {}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("Error: {}", stderr);
    }
    if !result.success() {
        remove_dir_all(path::Path::new("./.tmp")).unwrap();
        eprintln!("Failed to run executable");
        std::process::exit(1);
    }
    remove_dir_all(path::Path::new("./.tmp")).unwrap();
}
