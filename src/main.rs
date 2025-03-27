use clap::Command;

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
        Some(("run", _)) => {
            println!("Executing 'run' like 'go run'");
            // compile to temporary file and run using GCC
        }
        Some(("build", _)) => {
            println!("Executing 'build' like 'cargo build'");
            // compile using GCC
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
        .subcommand(Command::new("build"))
        .subcommand(Command::new("remove"))
        .subcommand(Command::new("update"))
        .subcommand(Command::new("search"))
        .subcommand(Command::new("init"))
}
