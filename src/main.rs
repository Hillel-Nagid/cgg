use std::ffi::OsString;

use clap::{ Command, Parser };

#[derive(Parser)]
struct RunArgs {}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("fmt", _)) => {
            println!("Executing 'fmt' like 'go fmt'");
            // clang-format
        }
        Some(("get", _)) => {
            println!("Executing 'get' like 'go get'");
            // url to git repo and save to deps dir
        }
        Some(("list", _)) => {
            println!("Executing 'list' like 'go list'");
            // read from lock file
        }
        Some(("run", args)) => {
            // compile to temporary file and run using GCC
            let run_args = args
                .get_many::<OsString>("id")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            let result = std::process::Command::new("gcc").args(run_args).status().unwrap();
            if !result.success() {
                eprintln!("Failed to run GCC");
                std::process::exit(1);
            }
        }
        Some(("build", args)) => {
            // compile using GCC
            let build_args = args
                .get_many::<OsString>("id")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            let result = std::process::Command::new("gcc").args(build_args).status().unwrap();
            if !result.success() {
                eprintln!("Failed to run GCC");
                std::process::exit(1);
            }
        }
        Some(("remove", _)) => {
            println!("Executing 'remove' like 'cargo remove'");
            // remove from deps dir and from lock file
        }
        Some(("update", _)) => {
            println!("Executing 'update' like 'cargo update'");
            // update lock file and deps dir
        }
        Some(("search", _)) => {
            println!("Executing 'search' like 'cargo search'");
            // search GitHub
        }
        Some(("init", _)) => {
            println!("Executing 'init' like 'cargo init'");
            // create a new project with a template
        }
        _ => unreachable!(), // If no subcommand was used, this should not happen
    }
}
fn cli() -> Command {
    Command::new("cgg")
        .version("0.1.0")
        .author("Hillel Nagid <hillel.nagid@mail.huji.ac.il>")
        .about("A GCC wrapper that resembles the Go and cargo clis")
        .subcommand_required(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("fmt"))
        .subcommand(Command::new("get"))
        .subcommand(Command::new("list"))
        .subcommand(Command::new("run"))
        .allow_external_subcommands(true)
        .subcommand(Command::new("build"))
        .subcommand(Command::new("remove"))
        .subcommand(Command::new("update"))
        .subcommand(Command::new("search"))
        .subcommand(Command::new("init"))
}
