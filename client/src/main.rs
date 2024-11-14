use std::{
    io::{prelude::*, BufReader, BufRead},
    net::{TcpStream},
    process::{Command},
    fs,
    env,
};

// Default HOST and PORT settings
const HOST: &str = "127.0.0.1";
const PORT: u16 = 7876;

fn main() {
    let address = format!("{}:{}", HOST, PORT);
    match TcpStream::connect(&address){
        Ok(mut stream) => {
            println!("Connection Established");
            // Sends a generic request and waits for reponse
            let request = "GET /index.html HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";
            stream.write_all(request.as_bytes()).expect("Failed to send to server");
            let mut reader = BufReader::new(&mut stream);

            // Reading the response line (html header) from stream
            let mut response_line = String::new();
            reader.read_line(&mut response_line).expect("Failed to read response line");

            // http header responses are added to a vector to be printed out
            let mut http_response = Vec::new();
            for line in reader.by_ref().lines() {
                let temp = line.expect("Couldn't read line");
                if temp.is_empty() {
                    break;
                }
                http_response.push(temp)
            }
            println!("Response headers: {http_response:#?}");

            // Gets file path using PATH env
            let mut file_path = env::current_dir().expect("Failed getting current directory");
            file_path.push("src/temp.html");

            // Grabs html content and writes it to the temporary file
            let mut html_content = String::new();
            reader.read_to_string(&mut html_content).expect("Failed to read html content");
            //println!("Content: {}", html_content);
            fs::write(&file_path, html_content).expect("Failed to write content to the file");

            // Makes the PATH into a string and opens in browser
            let path_str = file_path.to_str().expect("Failed to convert to string");
            open_in_browser(path_str);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

// A function which takes a path as a string and opens the html page in a browser
fn open_in_browser(path: &str) {
    println!("Opening in browser... ");
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", path]).spawn().expect("Failed to open html page");
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn().expect("Failed to open HTML file");
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(path).spawn().expect("Failed to open HTML file");
    } else {
        eprintln!("Platform not supported for this server");
    }
}
