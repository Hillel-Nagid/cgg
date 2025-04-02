use super::lock_file;
pub fn run(args: &clap::ArgMatches) {
    let mut name = String::from("new_app");
    let mut version = String::from("0.0.1");
    if let Some(name_arg) = args.get_one::<String>("name") {
        name = name_arg.clone();
    }
    if let Some(version_arg) = args.get_one::<String>("version") {
        version = version_arg.clone();
    }
    lock_file::create(name, version);
}
