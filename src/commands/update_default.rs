use std::env;
use std::fs;
use std::path::PathBuf;
use clap::ArgMatches;

pub fn run(args: &ArgMatches) {
    // Check if we have write permissions to the executable directory
    if !check_write_permissions() {
        eprintln!("Warning: No write permissions to the executable directory.");
        eprintln!("You may need to run this program with administrator privileges.");
        eprintln!("Continuing anyway, but default flags might not be saved.");
    }

    // Get the flags from the command line
    let flags = match args.get_many::<String>("flags") {
        Some(values) => values.collect::<Vec<_>>(),
        None => {
            eprintln!("No flags provided to set as default.");
            std::process::exit(1);
        }
    };

    if flags.is_empty() {
        eprintln!("No flags provided to set as default.");
        std::process::exit(1);
    }

    // Join the flags into a single space-separated string
    let mut default_flags_str = String::new();
    for flag in &flags {
        default_flags_str.push_str(flag);
        default_flags_str.push(' ');
    }
    let default_flags = default_flags_str.trim_end().to_string();

    // Save to a config file in the user's home directory
    let config_path = get_config_path();

    // No need to create directories as we're saving in the executable directory

    // Also set the environment variable for the current process
    unsafe {
        env::set_var("CGG_DEFAULT_FLAGS", &default_flags);
    }

    println!("Default flags set to: {}", default_flags);
    match fs::write(&config_path, &default_flags) {
        Ok(_) => println!("Configuration saved to: {}", config_path.display()),
        Err(e) => eprintln!("Failed to save configuration: {}", e),
    }
}

// Get the path to the config file in the executable directory
fn get_config_path() -> PathBuf {
    // Get the executable path
    let exe_path = env::current_exe().expect("Failed to get executable path");

    // Get the directory containing the executable
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");

    // Return the path to the config file in the same directory as the executable
    exe_dir.join("default_flags.cfg")
}

// Function to read the default flags (to be used elsewhere in your app)
pub fn get_default_flags() -> Option<String> {
    let config_path = get_config_path();

    // Try to read from the config file first
    match fs::read_to_string(&config_path) {
        Ok(flags) => Some(flags),
        Err(_) => {
            // If the file doesn't exist, check the environment variable
            env::var("CGG_DEFAULT_FLAGS").ok()
        }
    }
}

// Check if we have write permissions to the executable directory
fn check_write_permissions() -> bool {
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // Try to create a temporary file to test write permissions
            let test_file = exe_dir.join("write_test.tmp");
            match fs::File::create(&test_file) {
                Ok(_) => {
                    // Clean up the test file
                    let _ = fs::remove_file(test_file);
                    return true;
                }
                Err(_) => {
                    return false;
                }
            }
        }
    }
    false
}
