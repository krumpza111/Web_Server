use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    fs::{File},
};
use std::thread;
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::form_urlencoded;

// Default HOST and PORT settings
const HOST: &str = "127.0.0.1";
const PORT: u16 = 7876;

// Struct representing JSON form data accepting a user name and email
#[derive(Serialize, Deserialize, Debug)] 
struct FormData {
    name: String,
    email: String,
}

// Thread to handle parrallel client requests
fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 2048]; // Buffer for reading data stream
    let mut request = String::new(); // String storing the request header
    let mut body = String::new(); // String storing the data of the request

    /* 
    Reads data from the stream, chunks that data and pushes it to the request string
    If a end of header line is encountered we read the rest of the data into the body
    This should parse the entire request message. A later check will read the body again if missed.
    */
    while let Ok(bytes_read) = stream.read(&mut buf) {
        if bytes_read == 0 {
            println!("Request data is empty");
            break;
        }
        let chunk = String::from_utf8_lossy(&buf[..bytes_read]);
        request.push_str(&chunk);

        if request.contains("\r\n\r\n") { // end of headers encountered
            let headers_end = request.find("\r\n\r\n").unwrap();
            body = request[headers_end + 4..].to_string();
            break;
        }
    }

    println!("Request: {}", request);
    // Start of code chunk handling the request
    if request.starts_with("POST") {
        // Parses the request and 
        if let Some(content_length) = request
                .lines()
                .find(|line| line.to_lowercase().starts_with("content-length"))
                .and_then(|line| line.split(':').nth(1).map(|s| s.trim().parse::<usize>().unwrap_or(0)))
        {
            while body.len() < content_length {
                // attempts to read reamaining data if any
                let bytes_read = stream.read(&mut buf).expect("Failed to read the body");
                body.push_str(&String::from_utf8_lossy(&buf[..bytes_read]));
            }
             
            // uses urlencoeded to get the form data (JSON) from the body
            let form_data: FormData = form_urlencoded::parse(body.as_bytes())
                .into_owned()
                .fold(FormData {
                    name: String::new(),
                    email: String::new(),
                }, |mut acc, (key, val)| { // matching JSON to form data
                    match key.as_ref() {
                        "name" => acc.name = val.to_string(),
                        "email" => acc.email = val.to_string(),
                        _ => (),
                    }
                    acc
                });
            println!("Received name: {}", form_data.name);
            println!("Received email: {}", form_data.email);

            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nForm received successfully!";
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            eprintln!("No Content-Length header found");
        }
    } else { // If not POST METHOD THEN SEARCH FOR FILE
        // Matching to see if requested file is stored on server
        let requested_file = request.split_whitespace().nth(1).unwrap_or("/");
        if requested_file == "/index.html" {
            match File::open("src/index.html") {
                Ok(mut file) => {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    // Formats HTTP message header followed by http data
                    let mut response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Lenngth: {}\r\n\r\n{}", contents.len(), contents);
                    response = response + "\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    println!("HTML Page sent");
                },
                Err(_) => {
                    // File failed to open
                    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    println!("Failed to open file");
                }
            }
        } else {
            // No file stored on the server matches requested file
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
   
}

// Main function for setting up connection socket and waiting for connection
fn main() {
    let listener = TcpListener::bind((HOST, PORT)).expect("Failed to connect");
    println!("Ready to serve... ");
    for stream in listener.incoming() {
        match stream {
            // Once connection is established begin spawning seperate threads for parralell client execution
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}