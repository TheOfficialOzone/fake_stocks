


use crate::companies::company::Company;
use crate::companies::company_manager::CompanyManager;
use crate::companies::stock::Stock;
use crate::account::user::User;

use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};

//So it can use accounts and companies
mod companies;
mod account;


fn main() {

    let file = fs::read("html/hello.html");
   
    match file {
        Err(_) => println!("Error opening file!"),
        Ok(_) => println!("File opened!"),
    }

    let mut company_manager : CompanyManager = CompanyManager::new();
    //Create amazon

    //Create Jeff Bezos
    let mut jeffy : User = User::new(String::from("Jeffry Bezos"), 1000.0);

    let _ = company_manager.add_company(Company::new(String::from("Not Amazon"), 10, 10.0));
    
    {
        let amazon = company_manager.get_company_mut(0).unwrap();

        println!("{}", jeffy);

        let purchase_result = amazon.purchase_stock(&mut jeffy);

        match purchase_result {
            Err(error) => println!("{}", error),
            Ok(success) => println!("{}", success),
        }
    }

    for _ in 0..8 {
        company_manager.update();
    }


    let stock = jeffy.get_stock(0);
    let stock_value = stock.value(&company_manager);

    match stock_value {
        None => (),
        Some(x) => println!("Price of Amazon is {}", x),
    }


    println!("{}", company_manager.get_company(0).unwrap());
    println!("{}", jeffy);



    //Web Listener testing
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //println!("Connection established!");
        handle_connection(stream);
    }
}


fn handle_connection(mut stream : TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}