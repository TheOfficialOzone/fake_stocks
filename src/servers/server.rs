

use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Arc, RwLock};

use crate::account::user_manager::UserManager;
use crate::companies::company_manager::CompanyManager;
use crate::data::data_saving::{SaveData, read_from_file};


/// Gets the response based off the HTTPS request
fn get_response(buffer : &[u8; 1024], company_manager : &Arc<RwLock<CompanyManager>>, user_manager : &Arc<RwLock<UserManager>>) -> Result<String, String> {

    let load_page = b"GET / HTTP/1.1\r\n";
    let load_stock_data = b"GET /html/data.txt HTTP/1.1";
    let load_stock_amount = b"GET /html/stock_amount.txt HTTP/1.1";

    //Getting the webpage
    if buffer.starts_with(load_page) {
        return Ok(read_from_file("html/hello.html").unwrap());
    } else 
    //Load the stocks valuation(s)
    if buffer.starts_with(load_stock_data) {
        let company_manager_lock = company_manager.read();

        match company_manager_lock {
            Ok(company_man) => return Ok(company_man.get_data()),
            Err(error) => panic!("Stock data mutex was poisoned: {}", error),
        }
    } else 
    //Load the amount of stocks a user has
    if buffer.starts_with(load_stock_amount) {
        //Reads from the user manager
        let user_manager_lock = user_manager.read();

        let user_manager;
        match user_manager_lock {
            Ok(user_man) => user_manager = user_man,
            Err(error) => panic!("User manager lock was poisoned: {}", error),
        }

        //Gets the user
        let user = user_manager.get_user(0);
        //Returns the users stock amount
        return Ok(String::from(user.get_data()));
    }

    //If we are here, we do not have any valid responses
    Err(String::from("No response programmed"))
}

pub fn handle_connection(mut stream : TcpStream, company_manager : &Arc<RwLock<CompanyManager>>, user_manager : &Arc<RwLock<UserManager>>) -> Result<(), String> {
    //The Buffer
    let mut buffer = [0; 1024];

    //Reads the Stream
    let read_result = stream.read(&mut buffer);
    match read_result {
        Err(error) => return Err(error.to_string()),
        _ => (),
    }

    //DEBUG: Prints the request!
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    //Gets the response text
    let response_text_result = get_response(&buffer, company_manager, user_manager);

    //Defaults to the invalid response
    let mut status_line = "HTTP/1.1 404 NOT FOUND";
    let mut contents = read_from_file("html/404.html").unwrap();


    match response_text_result {
        Ok(response) => { contents = response; status_line = "HTTP/1.1 200 OK" },
        Err(error) => println!("{}", error),
    }

    //Formats the response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    
    //Writes the response
    match stream.write(response.as_bytes()) {
        Err(error) => return Err(error.to_string()),
        _ => (),
    }

    //Flushes the response
    match stream.flush() {
        Err(error) => return Err(error.to_string()),
        _ => (),
    }

    Ok(())
}
