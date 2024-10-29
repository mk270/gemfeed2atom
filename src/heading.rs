/*  gemfeed2atom: generates Atom feeds from gemlog directories

    by Martin Keegan

    The single function below is a hand translation of Solderpunk's
    Python code in "gemfeed". It's not likely to attract copyright
    protection by itself.
*/

use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};

// translated from the Python
pub fn extract_first_heading(filename: PathBuf, default: &str) -> String {
    let file = File::open(filename);

    let file = match file {
        Ok(file) => file,
        Err(_) => return default.to_string(),
    };

    let reader = io::BufReader::new(file);

    for mut line in reader.lines().map_while(Result::ok) {
        if line.starts_with('#') {
            // Strip leading '#' characters
            while line.starts_with('#') {
                line = line[1..].to_string();
            }

            return line.trim().to_string();
        }
    }

    default.to_string()
}
