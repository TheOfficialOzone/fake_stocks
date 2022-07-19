

use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;


pub fn handle_connection(mut stream : TcpStream) -> Result<String, String> {
    //The Buffer
    let mut buffer = [0; 1024];

    //Reads the Stream
    let read_result = stream.read(&mut buffer);
    match read_result {
        Err(error) => return Err(error.to_string()),
        _ => (),
    }

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let load_page = b"GET / HTTP/1.1\r\n";
    let load_data = b"GET /html/data.txt HTTP/1.1";

    let (status_line, filename) = if buffer.starts_with(load_page) {
        ("HTTP/1.1 200 OK", "html/hello.html")
    } else if buffer.starts_with(load_data) {
        ("HTTP/1.1 200 OK", "html/data.txt")
    } else {
        println!("Could not understand request!");
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

    Ok(String::from("Connection processed successfully"))
}