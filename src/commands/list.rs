use std::fs::File;
use serde_json;
pub fn run() {
    let lock_file = File::open("cgg.lock.json").expect("Unable to open lock file");
    let lock_file_content: serde_json::Value = serde_json
        ::from_reader(lock_file)
        .expect("Unable to read lock file");
    println!("{}", serde_json::to_string_pretty(&lock_file_content).unwrap());
}
