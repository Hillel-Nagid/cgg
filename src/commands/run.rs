use super::build;

pub fn run(args: &clap::ArgMatches) {
    // TODO: use temp file and do immidiate executaion
    let parsed_args = build::parse_args(args);
    let result = std::process::Command::new("gcc").args(parsed_args).status().unwrap();
    if !result.success() {
        eprintln!("Failed to run GCC");
        std::process::exit(1);
    }
}
