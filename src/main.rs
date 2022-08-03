


//For storing IDs
use crate::id::ID;

use crate::companies::company::Company;
use crate::companies::company_manager::CompanyManager;
use crate::companies::stock::Stock;
use crate::users::user::User;
use crate::users::password::Password;
use crate::users::user_manager::UserManager;
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
mod users;
mod id;

fn main() {
    //Read / Write locks
    let company_manager_rw : Arc<RwLock<CompanyManager>> = Arc::new(RwLock::new(CompanyManager::new()));
    let user_manager_rw : Arc<RwLock<UserManager>> = Arc::new(RwLock::new(UserManager::new()));

    let mut user_man = user_manager_rw.write().unwrap();
    let mut company_man = company_manager_rw.write().unwrap();

    let _jef = user_man.new_user(String::from("jeffy_b"), String::from("Jeffy Bezos"), Password::new([20, 21, 23, 21]));
    let _tim = user_man.new_user(String::from("tim_c"), String::from("Tim Cook"), Password::new([20, 20, 23, 21]));

    //Create Jeff Bezos
    let _amazon = company_man.new_company(String::from("Amazon"), 200.0);
    let _apple = company_man.new_company(String::from("Apple"), 200.0);


    for _ in 0..40 {
        company_man.update();
    }

    drop(user_man);
    drop(company_man);

    //Web Listener testing
    let listener_result = TcpListener::bind("127.0.0.1:800");

    let listener;
    match listener_result {
        Ok(connection) => listener = connection,
        Err(error) => panic!("{}", error),
    }

    //The company manager / user manager shared across threads!
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

    //Forever loops as this will hopefully never crash :)
    loop {
        const LOOP_DELAY : u64 = 20;
        //Updates the company manager every 20 seconds
        if time.elapsed().as_secs() > LOOP_DELAY {
            //Adds 20 seconds to the time
            time += Duration::new(LOOP_DELAY,  0);

            let mut company_man = company_manager_rw.write().unwrap();
            //Update the company manager
            company_man.update();
        }
    }
}

