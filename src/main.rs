


//For storing IDs
use crate::id::ID;

use crate::companies::company::Company;
use crate::companies::company_manager::CompanyManager;
use crate::companies::stock::Stock;
use crate::account::user::User;
use crate::account::user_manager::UserManager;
use crate::data::data_saving::SaveData;
use crate::servers::server;

use std::time::{Instant, Duration};
use std::sync::{Arc, RwLock};
use std::thread;
use std::net::TcpListener;

//So it can use accounts and companies
mod servers;
mod data;
mod companies;
mod account;
mod id;

fn main() {
    //Makes the company manager
    //let mut company_manager : CompanyManager = CompanyManager::new();
    //let mut user_manager : UserManager = UserManager::new();

    //Read / Write locks
    let company_manager_rw : Arc<RwLock<CompanyManager>> = Arc::new(RwLock::new(CompanyManager::new()));
    let user_manager_rw : Arc<RwLock<UserManager>> = Arc::new(RwLock::new(UserManager::new()));

    let mut user_man = user_manager_rw.write().unwrap();
    let mut company_man = company_manager_rw.write().unwrap();

    let _jef = user_man.new_user(String::from("Jeffry Bezos"), 1000.0);
    let _tim = user_man.new_user(String::from("Jeffry Bezos's son Tim Cook"), 1000.0);

    //Make Jeffry
    // let _jef = user_manager.new_user(String::from("Jeffry Bezos"), 1000.0);
    // let _tim = user_manager.new_user(String::from("Jeffry Bezos's son Tim Cook"), 1000.0);

    //Create Jeff Bezos

    let _amazon = company_man.new_company(String::from("Amazon"), 20.0);
    let _apple = company_man.new_company(String::from("Apple"), 20.0);

    { // Testing
        let amazon_co = company_man.get_company_by_id(_amazon).unwrap();
        let _purchase_result = amazon_co.purchase_stock(user_man.get_user_by_id_mut(_jef).unwrap());
    }

    for _ in 0..5 {
        company_man.update();
    }
    
    println!("{}", company_man.get_company_by_id(_amazon).unwrap().get_data());
    println!("{}", company_man.get_company_by_id(_apple).unwrap().get_data());
    println!("{}", user_man.get_user_by_id(_jef).unwrap().get_data());
    println!("{}", user_man.get_user_by_id(_tim).unwrap().get_data());
    println!("{}", company_man);

    drop(user_man);
    drop(company_man);
    


    //Web Listener testing
    let listener_result = TcpListener::bind("127.0.0.1:7878");

    let listener;
    match listener_result {
        Ok(connection) => listener = connection,
        Err(error) => panic!("{}", error),
    }

    //This is the stock price data for the main function
    //let stock_price_data : Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    
    //This is the stock price data for the thread
    let thread_company_manager : Arc<RwLock<CompanyManager>> = Arc::clone(&company_manager_rw);
    let thread_user_manager : Arc<RwLock<UserManager>> = Arc::clone(&user_manager_rw); 


    // Spawns a thread to listen to web requests!
    thread::spawn(move || {
        for stream in listener.incoming() {
            //Checks for a stream
            let stream_result = stream;
    
            match stream_result {
                //Handles the streams connection
                Ok(stream) => {
                    //Handles a connection
                    match server::handle_connection(stream, &thread_company_manager, &thread_user_manager) {
                        Err(error) => println!("Error: {}", error),
                        _ => (),
                    }
                },
                Err(error) => println!("{}", error),
            }
        }
    });
    
    //Gets the time of start-up
    let mut time = Instant::now();
    loop {
        //Updates the company manager every two seconds
        if time.elapsed().as_secs() > 1 {
            //Adds two seconds to the time
            time += Duration::new(1,  0);

            let mut company_man = company_manager_rw.write().unwrap();
            //Update the company manager
            company_man.update();
        }
    }
}

