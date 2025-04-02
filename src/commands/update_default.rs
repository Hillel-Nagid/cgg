use std::env;

pub fn run(args: &clap::ArgMatches) {
    let flags = args.get_many::<String>("flags").into_iter().flatten().collect::<Vec<_>>();
    unsafe {
        let mut env_flags = String::new();
        for flag in flags.iter() {
            env_flags.push_str(flag);
            env_flags.push(' ');
        }
        env::set_var("CGG_DEFAULT_FLAGS", env_flags.trim_end());
    }
}
