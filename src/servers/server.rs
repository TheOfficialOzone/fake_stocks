

use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Arc, RwLock};

use httparse;

use crate::users::user_manager::UserManager;
use crate::companies::company_manager::CompanyManager;
use crate::data::data_saving::{SaveData, read_from_file};


/// Gets the sent text from a request
/// Returns a String with the bodies text!
fn get_text_from_request(buffer : &[u8; 1024]) -> Result<String, String> {
    //Makes some headers
    let mut headers = [httparse::EMPTY_HEADER; 32];
    //Places the headers into the request
    let mut request = httparse::Request::new(&mut headers);

    //Parses for the body's position
    let body_pos ;
    match request.parse(buffer) {
        //If the size is found
        Ok(size) => {
            //Ensures the position is valid
            match size {
                httparse::Status::Complete(pos) => body_pos = pos,
                httparse::Status::Partial => return Err(String::from("Buffer could not fit entire HTTP request")),
            }
        }, 
        Err(error) => return Err(error.to_string()),
    };

    //Gets the text from the rest (Unwraps are fine since we would have errored out by here)
    let str_buffer = std::str::from_utf8(buffer).unwrap();
    let str_slice = &str_buffer[body_pos..str_buffer.find('\0').unwrap()];

    Ok(String::from(str_slice))
}


/// Creates an account for the new user
fn create_account(buffer : &[u8; 1024], company_manager : &Arc<RwLock<CompanyManager>>, user_manager : &Arc<RwLock<UserManager>>) -> Result<String, String> { 
    //Gets the data from the request
    let request_data;
    match get_text_from_request(buffer) {
        Ok(name) => request_data = name,
        Err(error) => return Err(error),
    }

    Ok(String::from("Account created!"))
}


/// Sells a stock from a user
fn sell_stock(buffer : &[u8; 1024], company_manager : &Arc<RwLock<CompanyManager>>, user_manager : &Arc<RwLock<UserManager>>) -> Result<String, String> {
    //Gets the data from the request
    let request_data;
    match get_text_from_request(buffer) {
        Ok(name) => request_data = name,
        Err(error) => return Err(error),
    }

    //Splits the request
    let split_request : Vec<&str> = request_data.split(',').collect();

    let company_name : String;
    let sell_amount : usize;
    
    match split_request.len() {
        2 => {
            match split_request[0].parse::<usize>(){
                Ok(amount) => sell_amount = amount,
                Err(_error) => return Err(String::from("Error parsing thrrough HTTP request!")),
            };
            company_name = split_request[1].to_string();
        }
        _ => return Err(String::from("Error with HTTP request!")),
    }
    
    // Gets the user manager
    let user_manager_lock = user_manager.write();

    let mut user_manager;
    match user_manager_lock {
        Ok(user_man) => user_manager = user_man,
        Err(error) => panic!("User manager lock was poisoned: {}", error),
    }

    //Gets the user mutably
    let user = user_manager.get_user_mut(0);

    //Gets the company manager
    let company_manager_lock = company_manager.read();

    let company_manager;
    match company_manager_lock {
        Ok(user_man) => company_manager = user_man,
        Err(error) => panic!("User manager lock was poisoned: {}", error),
    }

    //Gets the company
    let company;
    match company_manager.get_company_by_name(&company_name) {
        Ok(comp) => company = comp,
        Err(error) => return Err(error),
    };
    
    //Sells the users stock
    let sell_result = user.sell_stock(&company_manager, company.id(), sell_amount);

    match sell_result {
        Ok(_) => return Ok(String::from("Sold")),
        Err(error) => return Err(error),
    }
}

/// Buys a stock mentioned by the buffer
fn buy_stock(buffer : &[u8; 1024], company_manager : &Arc<RwLock<CompanyManager>>, user_manager : &Arc<RwLock<UserManager>>) -> Result<String, String> {
    //Gets the data from the request
    let request_data;
    match get_text_from_request(buffer) {
        Ok(name) => request_data = name,
        Err(error) => return Err(error),
    }

    //Splits the request
    let split_request : Vec<&str> = request_data.split(',').collect();

    let company_name : String;
    let buy_amount : usize;
    
    match split_request.len() {
        2 => {
            match split_request[0].parse::<usize>(){
                Ok(amount) => buy_amount = amount,
                Err(_error) => return Err(String::from("Error parsing thrrough HTTP request!")),
            };
            company_name = split_request[1].to_string();
        }
        _ => return Err(String::from("Error with HTTP request!")),
    }
    
    // Gets the user manager
    let user_manager_lock = user_manager.write();

    let mut user_manager;
    match user_manager_lock {
        Ok(user_man) => user_manager = user_man,
        Err(error) => panic!("User manager lock was poisoned: {}", error),
    }

    //Gets the user mutably
    let user = user_manager.get_user_mut(0);

    //Gets the company manager
    let company_manager_lock = company_manager.read();

    let company_manager;
    match company_manager_lock {
        Ok(user_man) => company_manager = user_man,
        Err(error) => panic!("User manager lock was poisoned: {}", error),
    };

    //Gets the company
    let company;
    match company_manager.get_company_by_name(&company_name) {
        Ok(comp) => company = comp,
        Err(error) => return Err(error),
    };

    //Buys the users stock
    let purchase_result = company.purchase_stock(user, buy_amount);
    match purchase_result {
        Ok(_) => return Ok(String::from("Bought")),
        Err(error) => return Err(error),
    };
}

/// Gets the response based off the HTTPS request
fn get_response(buffer : &[u8; 1024], company_manager : &Arc<RwLock<CompanyManager>>, user_manager : &Arc<RwLock<UserManager>>) -> Result<String, String> {
    //All the possible request headers
    let load_page = b"GET / HTTP/1.1\r\n";
    let load_stock_data = b"GET /stock_data HTTP/1.1";
    let load_stock_amount = b"GET /stock_amount HTTP/1.1";
    let load_cash_amount = b"GET /money HTTP/1.1";
    let buy_stock_text = b"POST /buy_request HTTP/1.1";
    let sell_stock_text = b"POST /sell_request HTTP/1.1";
    let create_account = b"POST /create_account HTTP/1.1";

    //Getting the webpage
    if buffer.starts_with(load_page) {
        return Ok(read_from_file("html/hello.html").unwrap());
    } else 
    //Load the stocks valuations
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
        return Ok(String::from(user.wallet().get_data()));
    } else 
    //Load the cash
    if buffer.starts_with(load_cash_amount) {
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
        return Ok(user.money().to_string());
    } else
    //Sells a stock
    if buffer.starts_with(sell_stock_text){
        return sell_stock(buffer, company_manager, user_manager);
    } else 
    //Buys a stock
    if buffer.starts_with(buy_stock_text) {
        return buy_stock(buffer, company_manager, user_manager);
    } else
    //Creates an account
    if buffer.starts_with(create_account) {

    }

    //If we are here, we do not have any valid responses
    Err(String::from("No response programmed"))
}



/// Handles all possible requests from a client
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
