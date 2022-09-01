



//For storing IDs
use crate::id::ID;

use crate::companies::company::Company;
use crate::companies::company_manager::CompanyManager;
use crate::companies::stock::Stock;
use crate::users::ranking::{Ranker, RankerHistory};
use crate::users::user::User;
use crate::users::password::Password;
use crate::users::user_manager::UserManager;
use crate::data::data_saving::SaveData;
use crate::servers::server;
use crate::servers::client_tracker::ClientTracker;

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

/// Resets the company manager
fn reset_company_manager(company_manager : &Arc<RwLock<CompanyManager>>) -> Result<(), String> {
    //Start the company manager
    let mut company_man;
    match company_manager.write() {
        Ok(company_manager) => company_man = company_manager,
        Err(error) => return Err(error.to_string()),
    };

    //Resets Apple
    match company_man.get_company_by_name_mut(&String::from("Apple")) {
        Ok(company) => { company.reset_company(200.0).unwrap(); company.id()},
        Err(_error) => company_man.new_company(String::from("Apple"), 200.0),
    };

    //Resets Amazon
    match company_man.get_company_by_name_mut(&String::from("Amazon")) {
        Ok(company) => { company.reset_company(200.0).unwrap(); company.id()},
        Err(_error) => company_man.new_company(String::from("Amazon"), 200.0),
    };

    for _ in 0..50 {
        company_man.update();
    }

    Ok(())
}

fn main() -> Result<(), String> {
    //Read / Write locks
    let company_manager_rw : Arc<RwLock<CompanyManager>> = Arc::new(RwLock::new(CompanyManager::new()));
    let user_manager_rw : Arc<RwLock<UserManager>> = Arc::new(RwLock::new(UserManager::new()));
    let ranker_rw : Arc<RwLock<Ranker>> = Arc::new(RwLock::new(Ranker::new()));
    let ranker_history_rw : Arc<RwLock<RankerHistory>> = Arc::new(RwLock::new(RankerHistory::new()));
    let client_tracker_rw : Arc<RwLock<ClientTracker>> = Arc::new(RwLock::new(ClientTracker::new()));

    //Resets the company manager
    _ = reset_company_manager(&company_manager_rw);

    //Web Listener testing
    let listener = match TcpListener::bind("127.0.0.1:8000") {
        Ok(listener) => listener,
        Err(error) => return Err(error.to_string()),
    };

    //The company manager / user manager shared across threads!
    let thread_company_manager : Arc<RwLock<CompanyManager>> = Arc::clone(&company_manager_rw);
    let thread_user_manager : Arc<RwLock<UserManager>> = Arc::clone(&user_manager_rw);
    let thread_ranker : Arc<RwLock<Ranker>> = Arc::clone(&ranker_rw);
    let thread_ranker_history : Arc<RwLock<RankerHistory>> = Arc::clone(&ranker_history_rw);
    let thread_client_tracker : Arc<RwLock<ClientTracker>> = Arc::clone(&client_tracker_rw);

    // Spawns a thread to listen to web requests!
    thread::spawn(move || {
        for stream in listener.incoming() {
            //Checks for a stream
            match stream {
                Ok(stream) => {
                    //Handles a request from a client
                    match server::handle_connection(stream, &thread_client_tracker, &thread_company_manager, &thread_user_manager, &thread_ranker, &thread_ranker_history) {
                        Err(error) => println!("Error: {}", error),
                        _ => (),
                    };
                },
                Err(error) => println!("{}", error),
            }
        }
    });
    
    //Gets the time of start-up
    let mut time = Instant::now();
    let mut reset_time = time;

    const LOOP_DELAY : u64 = 5;
    const RESET_DELAY : u64 = 600;

    //Forever loops as this will hopefully never crash :)
    loop {
        // Resets the company manager after 1 day
        if reset_time.elapsed().as_secs() >= RESET_DELAY {
            // Resets in [RESET_DELAY] seconds
            reset_time += Duration::new(RESET_DELAY, 0);

            println!("Resetting!");
            // Reset the user manager
            match user_manager_rw.write() {
                Ok(mut user_man) => user_man.reset_users(),
                Err(error) => println!("{}", error),
            }

            // Resets the stock history / prices of all the companies
            match reset_company_manager(&company_manager_rw) {
                Err(error) => println!("{}", error),
                _ => (),
            }

            //Clears the client tracker (Everyone must re-login)
            match client_tracker_rw.write() {
                Ok(mut client_tracker) => client_tracker.clear(),
                Err(error) => println!("{}", error),
            }

            // Writes the old ranker to the ranker
            // And clears the new ranker
            match ranker_history_rw.write() {
                Ok(mut history) => {
                    match ranker_rw.write() {
                        Ok(mut ranker) => { history.add(ranker.clone()); ranker.clear()},
                        Err(error) => println!("{}", error),
                    }
                },
                Err(error) => println!("{}", error),
            }
        }
        
        //Updates the company manager every 20 seconds
        if time.elapsed().as_secs() >= LOOP_DELAY {
            //Adds 20 seconds to the time
            time += Duration::new(LOOP_DELAY,  0);

            // Gets the company manager
            let mut company_manager = match company_manager_rw.write() {
                Ok(company_manager) => company_manager,
                Err(error) => { println!("{}", error); break; },
            };

            // Update the company manager
            company_manager.update();
            
            // Reads the user manager
            let user_manager = match user_manager_rw.read() {
                Ok(user_manager) => user_manager,
                Err(error) => { println!("{}", error); break; },
            };

            // Updates the leaderboards
            match ranker_rw.write() {
                Ok(mut ranker) => {
                    match ranker.rank_users(&user_manager, &company_manager) {
                        Err(error) => println!("{}", error),
                        _ => (),
                    };
                },
                Err(error) => println!("{}", error.to_string()),
            };
        }
    }

    Ok(())
}

