use clap::Command;

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("fmt", _)) => {
            println!("Executing 'fmt' like 'go fmt'");
            // Add your logic for the 'fmt' command here
        }
        Some(("get", _)) => {
            println!("Executing 'get' like 'go get'");
            // Add your logic for the 'get' command here
        }
        Some(("list", _)) => {
            println!("Executing 'list' like 'go list'");
            // Add your logic for the 'list' command here
        }
        Some(("run", _)) => {
            println!("Executing 'run' like 'go run'");
            // Add your logic for the 'run' command here
        }
        Some(("build", _)) => {
            println!("Executing 'build' like 'cargo build'");
            // Add your logic for the 'build' command here
        }
        Some(("remove", _)) => {
            println!("Executing 'remove' like 'cargo remove'");
            // Add your logic for the 'remove' command here
        }
        Some(("update", _)) => {
            println!("Executing 'update' like 'cargo update'");
            // Add your logic for the 'update' command here
        }
        Some(("search", _)) => {
            println!("Executing 'search' like 'cargo search'");
            // Add your logic for the 'search' command here
        }
        Some(("init", _)) => {
            println!("Executing 'init' like 'cargo init'");
            // Add your logic for the 'init' command here
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
