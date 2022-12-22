//! Async writer used to test async file writing
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::logger;

/// Special `Result` type declaration
///   - Used to deal with special `async` return types
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Basic static text to write to new file
static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

/// Public `async` function to test multiple file writes to a specified location
pub async fn write_file(file_name: &str, folder_path: PathBuf) -> Result<()> {

    //! Create a directory for writing output to
    //!   - Returns with a newly created directory
    //!   - Prints `info` message if one already exists; passes
    match fs::create_dir(&folder_path) {
        Err(why) => logger::info(format!("{:?}", why.kind()).as_str()),
        Ok(_) => {},
    }

    // Create a new format string to join to provided filepath
    let file = format!("{}.txt", file_name);
    let new_path = folder_path.join(file);
    let display = new_path.display();

    // Obtain file handle to append new data to; creates file if it doesn't exist
    let mut file = match OpenOptions::new().append(true).create(true).open(&new_path) {
        Err(why) => panic!("[!] Couldn't open/create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write string to `file`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("[!] Couldn't write to {}: {}", display, why),
        Ok(_) => logger::success(format!("[+] Successfully wrote to {}", display).as_str()),
    }

    Ok(())
}