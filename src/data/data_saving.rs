

use std::io::prelude::*;
use std::fs::File;
use std::io::Write;


pub trait SaveData {
    fn get_data(&self) -> String;
}

/// Reads the file into a string
pub fn read_from_file(filename : &str) -> Result<String, String> {
    //Opens the file
    let file_result = File::open(filename);

    let mut file;
    match file_result {
        Ok(opened_file) => file = opened_file,
        Err(error) => return Err(error.to_string()),
    }

    //Reads the contents
    let mut contents : String = String::new();
    match file.read_to_string(&mut contents) {
        Err(error) => return Err(error.to_string()),
        _ => (),
    }
    //Return what was read from the file
    Ok(contents)
}

/// Saves data to a file
pub fn save_to_file(filename : &str, data : &String) -> Result<(), String> {
    //Creates the file
    let file_result = File::create(filename);
    
    //Opens the file
    let mut file;
    match file_result {
        Ok(new_file) => file = new_file,
        Err(error) => return Err(error.to_string()),
    }
    //Writes all the data to the file
    let write_result = file.write(data.as_bytes());

    //Checks that the write result is fully valid
    match write_result {
        Ok(value) => (if value == 0 {return Err(String::from("Unknown error writing to file"))}),
        Err(error) => return Err(error.to_string()),
    }

    Ok(())
}