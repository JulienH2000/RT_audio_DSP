use std::io::{BufRead, BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::path::{Path, PathBuf};
use std::fs;

use crate::dsp;

pub fn start(dsp_slots: Arc<Mutex<Vec<dsp::Dsp>>>) {

    let _server = thread::spawn(move || {
        let listener = TcpListener::bind("10.67.0.24:6060").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // todo
            handle_con(stream);
        }
    });
}

fn handle_con(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let path = http_req[0].split_whitespace().nth(1).map(|path| path.trim_start_matches('/')).unwrap();

    println!("{:?}", path);

    let file_path = get_full_path(path);

    println!("{:?}", file_path);

    match fs::File::open(&file_path) {
        Ok(mut file) => {
            let mime_type = get_mime_type(&file_path);
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).expect("failed to read file");

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                mime_type,
                contents.len()
            );
            
            // Send response
            let mut stream = stream;
            stream.write_all(response.as_bytes()).expect("Failed to write response headers");
            stream.write_all(&contents).expect("Failed to write file contents");
            stream.flush().expect("Failed to flush stream");
        },
        Err(_) => {
            // 404 Not Found response
            let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\n\r\n404 Not Found";
            let mut stream = stream;
            stream.write_all(response.as_bytes()).expect("Failed to write 404 response");
            stream.flush().expect("Failed to flush stream");
        }
    }
}

fn get_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        _ => "application/octet-stream",
    }
}

// Prefix path with the static files directory
fn get_full_path(relative_path: &str) -> PathBuf {
    let base_path = Path::new("src/control/web_server/html");
    let safe_path = relative_path.trim_start_matches('/');
    
    // Default to index.html if root is requested
    let file_path = base_path.join(safe_path);
    if file_path.is_dir() {
        file_path.join("index.html")
    } else {
        file_path
    }
}