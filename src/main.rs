mod commands;
mod lock_file;
use clap::{ Arg, ArgAction, Command };

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("fmt", args)) => {
            println!("Formatting files...");
            commands::fmt::run(args);
        }
        Some(("get", _)) => {
            println!("Executing 'get' like 'go get'");
            todo!("Implement get");
            // url to git repo and save to deps dir
        }
        Some(("list", _)) => {
            commands::list::run();
        }
        Some(("run", args)) => {
            println!("Running program...");
            commands::run::run(args);
        }
        Some(("build", args)) => {
            println!("Building program...");
            commands::build::run(args);
        }
        Some(("remove", _)) => {
            println!("Executing 'remove' like 'cargo remove'");
            todo!("Implement remove");
            // remove from deps dir and from lock file
        }
        Some(("update", _)) => {
            println!("Executing 'update' like 'cargo update'");
            todo!("Implement update");
            // update lock file and deps dir
        }
        Some(("search", _)) => {
            println!("Executing 'search' like 'cargo search'");
            todo!("Implement search");
            // search GitHub
        }
        Some(("init", args)) => {
            println!("Initializing new project...");
            commands::init::run(args);
        }
        Some(("change-default", args)) => {
            println!("Changing default flags...");
            commands::update_default::run(args);
        }
        _ => unreachable!(), // If no subcommand was used, this should not happen
    }
}
fn cli() -> Command {
    Command::new("cgg")
        .version("0.1.0")
        .author("Hillel Nagid <hillel.nagid@mail.huji.ac.il>")
        .about("A GCC wrapper that resembles the Go and Cargo CLIs")
        .subcommand_required(true)
        .subcommand(
            Command::new("fmt").arg(
                Arg::new("clang-format-args")
                    .trailing_var_arg(true)
                    .action(ArgAction::Append)
                    .num_args(0..)
            )
        )
        .subcommand(Command::new("get").hide(true))
        .subcommand(Command::new("list").hide(true))
        .subcommand(
            Command::new("run")
                .arg(Arg::new("default").short('d').action(ArgAction::SetTrue))
                .arg(
                    Arg::new("gcc_args")
                        .trailing_var_arg(true)
                        .action(ArgAction::Append)
                        .num_args(0..)
                )
        )
        .subcommand(
            Command::new("build")
                .arg(Arg::new("default").short('d').action(ArgAction::SetTrue))
                .arg(
                    Arg::new("gcc_args")
                        .trailing_var_arg(true)
                        .action(ArgAction::Append)
                        .num_args(0..)
                )
        )
        .subcommand(Command::new("remove").hide(true))
        .subcommand(Command::new("update").hide(true))
        .subcommand(Command::new("search").hide(true))
        .subcommand(Command::new("init"))
        .subcommand(
            Command::new("change-default").arg(
                Arg::new("flags")
                    .trailing_var_arg(true)
                    .action(ArgAction::Append)
                    .num_args(0..)
            )
        )
}
