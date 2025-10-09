// Copyright (C) 2025  laxenta
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // We will Get the year for the copyright notice
    let year = Command::new("date")
        .arg("+%Y")
        .output()
        .expect("cant get current year")
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