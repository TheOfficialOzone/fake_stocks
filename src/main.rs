

use crate::account::user_manager::UserManager;
//For storing IDs
use crate::id::ID;

use crate::companies::company::Company;
use crate::companies::company_manager::CompanyManager;
use crate::companies::stock::Stock;
use crate::account::user::User;
use crate::data::data_saving::{SaveData, save_to_file};
use crate::servers::server;


use core::time;
use crate::time::Duration;

use std::thread;
use std::net::{TcpListener, TcpStream};

//So it can use accounts and companies
mod companies;
mod account;
mod data;
mod id;
mod servers;


fn main() {
    //Makes the company manager
    let mut company_manager : CompanyManager = CompanyManager::new();
    let mut user_manager : UserManager = UserManager::new();

    //Make Jeffry
    user_manager.new_user(String::from("Jeffry Bezos"), 1000.0);
    user_manager.new_user(String::from("Jeffry Bezos's son Tim Cook"), 1000.0);

    //Create Jeff Bezos

    company_manager.new_company(String::from("Amazon"), 20.0);
    company_manager.new_company(String::from("Apple"), 20.0);

    for _ in 0..5 {
        company_manager.update();
    }
    
    println!("{}", company_manager.get_company(0));
    println!("{}", user_manager.get_user(0));

    //Web Listener testing
    let listener_result = TcpListener::bind("127.0.0.1:7878");

    let listener;
    match listener_result {
        Ok(connection) => listener = connection,
        Err(error) => panic!("{}", error),
    }

    let nonblock_result = listener.set_nonblocking(true);

    match nonblock_result {
        Err(error) => panic!("{}", error),
        _ => (),
    }
    
    for stream in listener.incoming() {
        //Checks for a stream
        let stream_result = stream;
        match stream_result {
            //Handles the streams connection
            Ok(stream) => {
                match server::handle_connection(stream) {
                    Err(error) => println!("{}", error),
                    _ => (),
                }
            },
            _ => (),
        }

        thread::sleep(Duration::from_secs(1));
        company_manager.update();
        let _ = save_to_file("html/data.txt", &company_manager.get_data());
    }
}

