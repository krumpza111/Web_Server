use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    fs::{File},
};
use std::thread;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 7876;

fn handle_client(mut stream: TcpStream) {
    // Reads data stream into buffer
    let mut buf = [0; 1024];
    if let Ok(bytes_read) = stream.read(&mut buf) {
        if bytes_read == 0 {
            println!("Empty message request");
            return;
        }

    let request = String::from_utf8_lossy(&buf[..bytes_read]);
    let requested_file = request.split_whitespace().nth(1).unwrap_or("/");

    // Matching to see if requested file is stored on server
    if requested_file == "/index.html" {
        match File::open("index.html") {
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
        println!("Invalid page requested");
    }
    }
}

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