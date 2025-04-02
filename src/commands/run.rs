use super::build;
use uuid::Uuid;
use std::fs::remove_file;
use std::path;
pub fn run(args: &clap::ArgMatches) {
    // TODO: use temp file and do immidiate executaion
    let uuid = Uuid::new_v4().to_string();
    let tmp_path = path::Path::join(path::Path::new("./tmp"), path::Path::new(&uuid));
    let mut parsed_args = build::parse_args(args);
    if let Some(output_index) = parsed_args.iter().position(|s| s == "-o") {
        parsed_args.splice(
            output_index..output_index + 1,
            vec![String::from("-o"), tmp_path.to_string_lossy().to_string()]
        );
    }
    let result = std::process::Command::new("gcc").args(parsed_args).status().unwrap();
    remove_file(&tmp_path).unwrap();
    if !result.success() {
        eprintln!("Failed to run GCC");
        std::process::exit(1);
    }
}
