use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Get the year for the copyright notice
    let year = Command::new("date")
        .arg("+%Y")
        .output()
        .expect("Failed to get current year")
        .stdout;
    let year = String::from_utf8_lossy(&year);

    // Check all Rust files for license headers
    let paths = fs::read_dir("src").unwrap();
    
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            check_license_header(&path, &year);
        }
    }
}

fn check_license_header(path: &Path, year: &str) {
    let content = fs::read_to_string(path).unwrap();
    let has_header = content.contains("Copyright (C)") && 
                    content.contains("GNU General Public License") &&
                    content.contains("This program is free software");

    if !has_header {
        panic!("Missing license header in {:?}", path);
    }
}