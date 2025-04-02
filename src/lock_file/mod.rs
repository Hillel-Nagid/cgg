use std::fs;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct LockFile {
    pub name: String,
    pub version: String,
    pub deps: Vec<Dependency>,
}
impl LockFile {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            deps: vec![],
        }
    }
    pub fn update_to_disk(&self) {
        let lock_file_path = "cgg.lock.json";
        let lock_file_content = serde_json::to_string(self).unwrap();
        fs::write(lock_file_path, lock_file_content).expect("Unable to write to lock file");
    }
}
pub fn create(name: String, version: String) -> LockFile {
    let lock_file = LockFile::new(name, version);
    lock_file.update_to_disk();
    fs::write("main.c", "int main() { return 0; }").expect("Unable to write to main.c");
    lock_file
}
