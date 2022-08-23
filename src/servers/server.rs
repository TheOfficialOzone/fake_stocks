

use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Arc, RwLock};

use httparse;

use crate::users::ranking::{Ranker, RankerHistory};
use crate::users::user_manager::UserManager;
use crate::companies::company_manager::CompanyManager;
use crate::data::data_saving::{SaveData, read_from_file};
use crate::{Password, ClientTracker, User, ID};

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
            body_pos = match size {
                httparse::Status::Complete(body_pos) => body_pos,
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

/// Gets the cookie from a request
/// Returns a String with all the text for the cookie
fn get_cookie_from_request(buffer : &[u8; 1024]) -> Result<String, String> {
    //Makes some headers
    let mut headers = [httparse::EMPTY_HEADER; 32];
    //Places the headers into the request
    let mut request = httparse::Request::new(&mut headers);

    //Parses for the body's position
    match request.parse(buffer) {
        //If the size is found
        Ok(size) => {
            //Ensures the position is valid
            match size {
                httparse::Status::Complete(_) => (),
                httparse::Status::Partial => return Err(String::from("Buffer could not fit entire HTTP request")),
            }
        }, 
        Err(error) => return Err(error.to_string()),
    };

    //Looks for the cookie header
    let cookie_header : Vec<&httparse::Header> = request.headers
        .iter()
        .filter(|header| header.name == "Cookie")
        .collect();

    if cookie_header.len() != 1 { return Err(String::from("Error parsing the cookie header")); }

    //Gets the value
    match std::str::from_utf8(cookie_header[0].value) {
        Ok(text) => Ok(text.to_string()),
        Err(error) => Err(error.to_string()),
    }
}


/// Gets the clients ID from an HTTP Request
fn get_client_id_from_request(buffer : &[u8; 1024]) -> Result<ID, String> {
    // Gets the cookie text
    let mut cookie_text;
    match get_cookie_from_request(buffer) {
        Ok(text) => cookie_text = text,
        Err(error) => return Err(error),
    };

    //Retains only text, no whitespace
    cookie_text.retain(|c| !c.is_whitespace());

    //Split the text by ';'
    let cookie_split = cookie_text.split(";");

    //Loop through each split
    for str in cookie_split {
        let text = str.to_string();
        //We have the ID
        if text.contains("ID=") {
            return ID::from_string(&text);
        }
    };

    Err(String::from("Client ID was not found!"))
}

/// Sells a stock from a user
fn sell_stock(buffer : &[u8; 1024], client_tracker_rw : &Arc<RwLock<ClientTracker>>, company_manager_rw : &Arc<RwLock<CompanyManager>>, user_manager_rw : &Arc<RwLock<UserManager>>) -> Result<String, String> {
    //Gets the data from the request
    let request_data = match get_text_from_request(buffer) {
        Ok(request_data) => request_data,
        Err(error) => return Err(error),
    };

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
    
    //Gets the clients ID from the request
    let client_id : ID = match get_client_id_from_request(buffer) {
        Ok(client_id) => client_id,
        Err(error) => return Err(error),
    };

    // Gets the users ID from the client tracker
    let client_tracker = match client_tracker_rw.read() {
        Ok(client_tracker) => client_tracker,
        Err(error) => return Err(error.to_string()),
    };

    //Gets the users ID
    let user_id : ID = match client_tracker.get_user_id_by_client_id(client_id) {
        Ok(user_id) => user_id,
        Err(error) => return Err(error),
    };

    // Gets the user manager
    let mut user_manager = match user_manager_rw.write() {
        Ok(user_manager) => user_manager,
        Err(error) => panic!("User manager lock was poisoned: {}", error),
    };

    // Gets the user mutably
    let user : &mut User = match user_manager.get_user_by_id_mut(user_id) {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    //Gets the company manager
    let company_manager = match company_manager_rw.read() {
        Ok(company_manager) => company_manager,
        Err(error) => panic!("User manager lock was poisoned: {}", error),
    };

    //Gets the company
    let company = match company_manager.get_company_by_name(&company_name) {
        Ok(company) => company,
        Err(error) => return Err(error),
    };
    
    //Sells the users stock
    match user.sell_stock(&company_manager, company.id(), sell_amount) {
        Ok(_) => return Ok(String::from("Sold")),
        Err(error) => return Err(error),
    }
}

/// Buys a stock mentioned by the buffer
fn buy_stock(buffer : &[u8; 1024], client_tracker_rw : &Arc<RwLock<ClientTracker>>, company_manager_rw : &Arc<RwLock<CompanyManager>>, user_manager_rw : &Arc<RwLock<UserManager>>) -> Result<String, String> {    
    //Gets the data from the request
    let request_data;
    match get_text_from_request(buffer) {
        Ok(name) => request_data = name,
        Err(error) => return Err(error),
    }

    //Splits the request
    let split_request : Vec<&str> = request_data.split(',').collect();

    //Gets the company name and the amount to buy
    let company_name : String;
    let buy_amount : usize;
    
    match split_request.len() {
        2 => {
            buy_amount = match split_request[0].parse::<usize>(){
                Ok(buy_amount) => buy_amount,
                Err(_error) => return Err(String::from("Error parsing through HTTP request!")),
            };
            company_name = split_request[1].to_string();
        }
        _ => return Err(String::from("Error with HTTP request!")),
    }

    //Gets the clients ID from the request
    let client_id = match get_client_id_from_request(buffer) {
        Ok(client_id) => client_id,
        Err(error) => return Err(error),
    };

    // Gets the users ID from the client tracker
    let client_tracker = match client_tracker_rw.read() {
        Ok(client_tracker) => client_tracker,
        Err(error) => return Err(error.to_string()),
    };

    //Gets the users ID
    let user_id : ID = match client_tracker.get_user_id_by_client_id(client_id) {
        Ok(user_id) => user_id,
        Err(error) => return Err(error),
    };
    
    // Gets the user manager
    let mut user_manager = match user_manager_rw.write() {
        Ok(user_manager) => user_manager,
        Err(error) => panic!("User manager lock was poisoned: {}", error),
    };

    // Gets the user mutably
    let user : &mut User = match user_manager.get_user_by_id_mut(user_id) {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    // Gets the company manager
    let company_manager = match company_manager_rw.read() {
        Ok(company_manager) => company_manager,
        Err(error) => panic!("User manager lock was poisoned: {}", error),
    };

    //Gets the company
    let company= match company_manager.get_company_by_name(&company_name) {
        Ok(company) => company,
        Err(error) => return Err(error),
    };

    //Buys the users stock
    match company.purchase_stock(user, buy_amount) {
        Ok(_) => return Ok(String::from("Bought")),
        Err(error) => return Err(error),
    };
}

/// Creates an Account for the user
fn create_account(buffer : &[u8; 1024], client_tracker_rw : &Arc<RwLock<ClientTracker>>, user_manager_rw : &Arc<RwLock<UserManager>>) -> Result<String, String> {
    //Gets the data from the request
    let request_data : String = match get_text_from_request(buffer) {
        Ok(name) => name,
        Err(error) => return Err(error),
    };

    //Gets the user name
    let user_name : String = match parse_text(&String::from("USERNAME:"), &request_data) {
        Ok(name) => name,
        Err(_error) => return Err(String::from("Could not parse username!")),
    };

    //Gets the display name
    let display_name : String = match parse_text(&String::from("DISPLAYNAME:"), &request_data) {
        Ok(name) => name,
        Err(_error) => return Err(String::from("Could not parse display name!")),
    };

    //Gets the password
    let password : String = match parse_text(&String::from("PASSWORD:"), &request_data) {
        Ok(pass) => pass,
        Err(_error) => return Err(String::from("Could not parse password!")),
    };

    //Generates the password from the text
    let user_passord : Password = match Password::from_text(&password) {
        Ok(pass) => pass,
        Err(error) => return Ok(error.to_string()),
    };

    // Validates that the users info is valid
    {
        //The User name / Display name must be less than 20 characters long
        if user_name.len() > 20 { return Ok(String::from("User name must be less than 20 characters long")); }
        if display_name.len() > 20 { return Ok(String::from("Display name must be less than 20 characters long"))}
    }
    //We know have a valid User, so lets make one!

    // Gets the user manager
    let mut user_manager = match user_manager_rw.write() {
        Ok(user_manager) => user_manager,
        Err(error) => return Err(error.to_string()),
    };

    // Gets the user by the user name (Should not find one)
    // (MUST BE CHECKED WHILE WE HAVE THE WRITE LOCK TO AVOID ANY CONCURRENCY BS)
    if let Ok(_user) = user_manager.get_user_by_username(&user_name) {
        return Ok(format!("User name {} already in use!", user_name));
    }

    // Gets the client tracker
    let mut client_tracker = match client_tracker_rw.write() {
        Ok(client_tracker) => client_tracker,
        Err(error) => return Err(error.to_string()),
    };

    // Adds the new User
    let user_id = user_manager.new_user(user_name.clone(), display_name.clone(), user_passord);

    //Now adds the new user to the tracker
    match client_tracker.add_client(user_id, user_name, display_name) {
        Ok(new_id) => Ok(format!("ID={}", new_id)),
        Err(error) => Err(error),
    }
}

/// Logins the client to their account
fn login(buffer : &[u8; 1024], client_tracker_rw : &Arc<RwLock<ClientTracker>>, user_manager_rw : &Arc<RwLock<UserManager>>) -> Result<String, String> {
    //Gets the data from the request
    let request_data = match get_text_from_request(buffer) {
        Ok(request_data) => request_data,
        Err(error) => return Err(error),
    };

    //Gets the user name
    let user_name : String = match parse_text(&String::from("USERNAME:"), &request_data) {
        Ok(name) => name,
        Err(_error) => return Err(String::from("Could not parse username!")),
    };

    //Gets the password
    let password : String = match parse_text(&String::from("PASSWORD:"), &request_data) {
        Ok(pass) => pass,
        Err(_error) => return Err(String::from("Could not parse password!")),
    };

    //Generates the password from the text
    let user_passord = match Password::from_text(&password) {
        Ok(user_passord) => user_passord,
        Err(error) => return Err(error.to_string()),
    };

    //Gets the user manager
    let user_manager = match user_manager_rw.read() {
        Ok(user_manager) => user_manager,
        Err(error) => return Err(error.to_string()),
    };

    //Checks that the account exists
    let user : &User = match user_manager.get_user_by_username(&user_name) {
        Ok(read_user) => read_user,
        Err(_error) => return Ok(format!("No user with the name {} exist", user_name)),
    };

    //Ensures the password is correct
    if !user.try_password(user_passord) {
        return Ok(String::from("Incorrect password"));
    }

    //Gets the socket tracker
    let mut client_tracker = match client_tracker_rw.write() {
        Ok(client_tracker) => client_tracker,
        Err(error) => return Err(error.to_string()),
    };

    //Adds the client to the socket tracker
    match client_tracker.add_client(user.id(), user_name, user.display_name().clone()) {
        Ok(client_id) => Ok(format!("ID={}", client_id)),
        Err(_) => {
            match client_tracker.get_client_id_by_user_id(user.id()) {
                Ok(client_id) => Ok(format!("ID={}", client_id)),
                Err(error) => Err(error),
            }
        },
    }
}

/// Loads the new leaderboards
fn load_new_leaderboards(ranker_rw : &Arc<RwLock<Ranker>>) -> Result<String, String> {
    //Reads the ranker
    match ranker_rw.read() {
        Ok(ranker) => load_leaderboards(&ranker),
        Err(error) => Err(error.to_string()),
    }
}

/// Loads the old leaderboards
fn load_old_leaderboards(ranker_history_rw : &Arc<RwLock<RankerHistory>>) -> Result<String, String>  {
    //Reads the ranker
    match ranker_history_rw.read() {
        Ok(ranker_history) => {
            match ranker_history.get_recent() {
                Some(ranker) => load_leaderboards(ranker),
                None => Err(String::from("No ranker in history!")),
            }
        },
        Err(error) => Err(error.to_string()),
    }
}

/// Loads the leaderboards from a ranker
fn load_leaderboards(ranker : &Ranker) -> Result<String, String>  {
    //Gets the leaderboard data
    ranker.get_data_range(0..10)
}

/// Parses text for whatever is in 'to_find'
/// # Examples
/// ```
/// let parse_this : String = String::from("USER:Ozone");
/// let username = parse_text(&String::from("USER:"), &parse_this).unwrap();
/// assert_eq!(username, String::from("Ozone"));
/// ```
fn parse_text(to_find : &String, to_parse : &String) -> Result<String, String> {
    //Splits by line
    let lines = to_parse.split('\n');

    //Loops through each line looking for the user name
    for line in lines {
        //Finds the USERNAME:
        if let Some(mut pos) = line.find(to_find) {
            //Adds the offset from the name
            pos += to_find.len();
            let found_text = &line[pos..];
            //Returns the rest of the string
            return Ok(found_text.to_string());
        }
    };

    //Err
    Err(format!("Could not find {} in {}", to_find, to_parse))
}


/// Gets the response based off the HTTPS request
fn get_response(buffer : &[u8; 1024], client_tracker : &Arc<RwLock<ClientTracker>>, company_manager : &Arc<RwLock<CompanyManager>>, user_manager : &Arc<RwLock<UserManager>>, ranker : &Arc<RwLock<Ranker>>, ranker_history_rw : &Arc<RwLock<RankerHistory>>) -> Result<String, String> {
    //All the possible request headers
    let load_page = b"GET / ";
    let load_login_page = b"GET /login.html";
    let load_stock_data = b"GET /stock_data";
    let load_stock_amount = b"GET /stock_amount";
    let load_cash_amount = b"GET /money";
    let load_leaderboard = b"GET /leaderboard_data";
    let load_old_leaderboard = b"GET /old_leaderboard_data";
    let buy_stock_text = b"POST /buy_request";
    let sell_stock_text = b"POST /sell_request";
    let login_text = b"POST /login";
    let create_account_text = b"POST /create_account";

    //Getting the webpage
    if buffer.starts_with(load_page) {
        return Ok(read_from_file("html/hello.html").unwrap());
    } else 
    //Loads the login page
    if buffer.starts_with(load_login_page) {
        return Ok(read_from_file("html/login.html").unwrap());
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
        //Gets the clients ID from the request
        let client_id : ID;
        match get_client_id_from_request(buffer) {
            Ok(id) => client_id = id,
            Err(error) => return Err(error),
        }

        // Gets the users ID from the client tracker
        let client_track;
        match client_tracker.read() {
            Ok(tracker) => client_track = tracker,
            Err(error) => return Err(error.to_string()),
        }

        // Tracks the client
        let user_id : ID;
        match client_track.get_user_id_by_client_id(client_id) {
            Ok(id) => user_id = id,
            Err(error) => return Err(error),
        }

        //Reads from the user manager
        let user_manager_lock = user_manager.read();

        let user_manager;
        match user_manager_lock {
            Ok(user_man) => user_manager = user_man,
            Err(error) => panic!("User manager lock was poisoned: {}", error),
        }

        //Gets the user
        let user : &User;
        match user_manager.get_user_by_id(user_id) {
            Ok(usr) => user = usr,
            Err(error) => return Err(error),
        }

        //Returns the users stock amount
        return Ok(String::from(user.wallet().get_data()));
    } else 
    //Load the cash
    if buffer.starts_with(load_cash_amount) {
        //Gets the clients ID from the request
        let client_id : ID;
        match get_client_id_from_request(buffer) {
            Ok(id) => client_id = id,
            Err(error) => return Err(error),
        }

        // Gets the users ID from the client tracker
        let client_track;
        match client_tracker.read() {
            Ok(tracker) => client_track = tracker,
            Err(error) => return Err(error.to_string()),
        }

        //client_track.
        let user_id : ID;
        match client_track.get_user_id_by_client_id(client_id) {
            Ok(id) => user_id = id,
            Err(error) => return Err(error),
        }

        //Reads from the user manager
        let user_manager_lock = user_manager.read();

        let user_manager;
        match user_manager_lock {
            Ok(user_man) => user_manager = user_man,
            Err(error) => panic!("User manager lock was poisoned: {}", error),
        }

        //Gets the user
        let user : &User;
        match user_manager.get_user_by_id(user_id) {
            Ok(usr) => user = usr,
            Err(error) => return Err(error),
        }

        //Returns the users stock amount
        return Ok(user.money().to_string());
    } else
    //Loads the leaderboards
    if buffer.starts_with(load_leaderboard) {
        return load_new_leaderboards(ranker);
    } else
    // Loads the old leaderboards
    if buffer.starts_with(load_old_leaderboard) {
        return load_old_leaderboards(ranker_history_rw);
    }
    //Sells a stock
    if buffer.starts_with(sell_stock_text){
        return sell_stock(buffer, client_tracker, company_manager, user_manager);
    } else 
    //Buys a stock
    if buffer.starts_with(buy_stock_text) {
        return buy_stock(buffer, client_tracker, company_manager, user_manager);
    } else
    if buffer.starts_with(login_text) {
        return login(buffer, client_tracker, user_manager);
    } else
    //Creates an account
    if buffer.starts_with(create_account_text) {
        return create_account(buffer, client_tracker, user_manager);
    }

    //If we are here, we do not have any valid responses
    Err(String::from("No response programmed"))
}



/// Handles all possible requests from a client
/// If a request is not pre-programmed, Error 404 is returned
pub fn handle_connection(mut stream : TcpStream, client_tracker : &Arc<RwLock<ClientTracker>>, company_manager : &Arc<RwLock<CompanyManager>>, user_manager : &Arc<RwLock<UserManager>>, ranker : &Arc<RwLock<Ranker>>, ranker_history_rw : &Arc<RwLock<RankerHistory>>) -> Result<(), String> {
    //The Buffer
    let mut buffer = [0; 1024];

    //Reads the Stream
    let read_result = stream.read(&mut buffer);
    match read_result {
        Err(error) => return Err(error.to_string()),
        _ => (),
    }

    //DEBUG: Prints the request!
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    //Gets the response text
    let response_text_result = get_response(&buffer, client_tracker, company_manager, user_manager, ranker, ranker_history_rw);

    //Defaults to the invalid response
    let status_line;
    let contents;

    match response_text_result {
        //The response was ok
        Ok(response) => { 
            contents = response; 
            status_line = "HTTP/1.1 200 OK"; 
        },
        //There was an error processing the request
        Err(error) => { 
            // If the ID is wrong make the log back in!
            if error.starts_with("No client with ID") {
                contents = "INVALID ID".to_string();
                status_line = "HTTP/1.1 200 OK";
            } else {
                println!("Error: {}", error);
                status_line = "HTTP/1.1 404 NOT FOUND";
                
                if error.starts_with("No client with ID") {
                    contents = String::from("INVALID ID");
                } else {
                    contents = read_from_file("html/404.html").unwrap(); 
                }
            }
        },
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
