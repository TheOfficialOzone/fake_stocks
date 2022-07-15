


use std::fs::File;
use std::io::Write;

pub trait SaveData {
    fn get_data(&self) -> String;
}


/*
Saves data to a file

Returns the filename ok successful execution
 */
pub fn save_to_file(filename : &str, data : &String) -> Result<String, String> {
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

    Ok(filename.to_string())
}