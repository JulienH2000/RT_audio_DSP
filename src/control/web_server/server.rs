/*
+-----------------------------------------------------------------------+
|
|   Web server API syntax 
|
|   !! channel parameter is ignored for now but is needed !!
|
|   host:6060/api/  dsp/channel/target/subtarget/param/
|                   ping/
|                   state/target
|
|   ex:  POST /api/dsp/1/eq/band6/freq/ require JSON payload
|        GET /api/ping/
|        GET /state/ch1rms
|
+-----------------------------------------------------------------------+
*/

use std::io::{BufRead, BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::string::ParseError;
use std::sync::{Arc, Mutex};
use std::thread;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Deserializer, Serialize};


use crate::{dsp, errors};
use dsp::Dsp;
use crate::control::control_utils::ControlTarget;

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST
}

pub fn start(dsp_slots: Arc<Mutex<Vec<dsp::Dsp>>>) {

    let _server = thread::spawn(move || {
        let listener = TcpListener::bind("10.67.0.24:6060").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // todo
            handle_con(stream, Arc::clone(&dsp_slots));
        }
    });
}

fn handle_con(mut stream: TcpStream, dsp_slots: Arc<Mutex<Vec<dsp::Dsp>>>) {
    let mut buf_reader = BufReader::new(&stream);

    let http_req: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("HTTP Request: {:?}", http_req);

    let payload_len = http_req
        .iter()
        .find(|line| line.starts_with("Content-Length:"))
        .and_then(|line| line.split(":").nth(1))
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);


    let path = http_req[0].split_whitespace().nth(1).unwrap();
    println!("PATH: {:?}", path);

    let method = get_req_type(&http_req).unwrap();
    println!("METHOD: {:?}", method);

    let payload = extract_payload(&mut buf_reader, payload_len);
    println!("PAYLOAD: {:?}", payload);

    if path.starts_with("/api/") {
        handle_api_request(path, method, payload, stream, dsp_slots);
        println!("API Req Received !!");
    } else {
        handle_file_request(path, stream);
    }


    
    
}

fn extract_payload(reader: &mut BufReader<&TcpStream>, len: usize) -> Option<String> {
    if len > 0 {
        let mut body = vec![0; len as usize];
        reader.read_exact(&mut body).unwrap();
        let json_payload = String::from_utf8_lossy(&body).to_string();
        Some(json_payload)
    } else {
        None
    }
}

fn get_req_type(req: &Vec<String>) -> Result<HttpMethod, errors::HttpError> {

    let req_type = req[0].split_whitespace().nth(0).unwrap();

    match req_type {
        "GET" => return Ok(HttpMethod::GET),
        "POST" => return Ok(HttpMethod::POST),
        _ => return Err(errors::HttpError::InvalidMethod)
    }
}

#[derive(Debug, serde::Deserialize)]
struct ApiRequest {
    target: String,
    #[serde(deserialize_with = "deserialize_f32")]
    value: f32,
}

fn parse_json(json_payload: &str) -> serde_json::Result<ApiRequest> {
    return serde_json::from_str(json_payload);
}

fn deserialize_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Deserialize};

    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(s) => s.parse().map_err(de::Error::custom),
        serde_json::Value::Number(n) => n.as_f64().map(|f| f as f32).ok_or_else(|| de::Error::custom("Invalid number")),
        _ => Err(de::Error::custom("Expected string or number")),
    }
}

fn handle_api_request(path: &str, method: HttpMethod, payload: Option<String>, stream: TcpStream, dsp_slots: Arc<Mutex<Vec<dsp::Dsp>>>) -> Result<ApiRequest, errors::HttpError> {

    let req = match payload {
        Some(p) => parse_json(&p).unwrap(),
        None => return Err(errors::HttpError::UnsupportedReq)
    };

    let api_path = path.trim_start_matches("/api");

    match run_dsp_command(req, api_path, dsp_slots) {
        Ok(_) => {
            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nCommand Success";
            // Send response
            let mut stream = stream;
            stream.write_all(response.as_bytes()).expect("Failed to write response headers");
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

    return Err(errors::HttpError::UnsupportedReq)
}

fn handle_file_request(path: &str, mut stream: TcpStream) {
    let file_path = get_full_path(path);
    println!("full path: {:?}", file_path);
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

fn run_dsp_command(req: ApiRequest, path: &str, dsp_slots: Arc<Mutex<Vec<dsp::Dsp>>>) -> Result<(), errors::CmdError> {

    let mut dsp_slots = dsp_slots.lock().unwrap();

    let path: Vec<&str> = path.trim().trim_start_matches('/').trim_end_matches('/').split("/").collect();

    println!("{:?}", path);

    let value = req.value;

    match path[0] {
        "dsp" => match path[2]  {
            "trim" => { 
                for dsp in dsp_slots.iter_mut() {
                    match dsp {
                        Dsp::Trim(trim) => match path[4] {

                                    "amp" => {
                                        trim.amp = ControlTarget::gain_from_f32(value);
                                    },
                                    _ => return Err(errors::CmdError{}),
                        },
                        _ => ()
                    }
                }
            },
            "eq" => { 
                for dsp in dsp_slots.iter_mut() {
                    match dsp {
                        Dsp::EQ(eq) => match path[3] {
                            "band1" => {
                                match path[4] {
                                    "freq" => {
                                        eq.band1.freq = ControlTarget::freq_from_i32(value as i32);
                                    },
                                    "amp" => {
                                        eq.band1.amp = ControlTarget::gain_from_f32(value);
                                    },
                                    "q" => {
                                        eq.band1.q = ControlTarget::q_from_f32(value);
                                    },
                                    "model" => {},
                                    _ => return Err(errors::CmdError{}),
                                }
                            },
                            "band2" => {
                                match path[4] {
                                    "freq" => {
                                        eq.band2.freq = ControlTarget::freq_from_i32(value as i32);
                                    },
                                    "amp" => {
                                        eq.band2.amp = ControlTarget::gain_from_f32(value);
                                    },
                                    "q" => {
                                        eq.band2.q = ControlTarget::q_from_f32(value);
                                    },
                                    "model" => {},
                                    _ => return Err(errors::CmdError{}),
                                }
                            },
                            "band3" => {
                                match path[4] {
                                    "freq" => {
                                        eq.band3.freq = ControlTarget::freq_from_i32(value as i32);
                                    },
                                    "amp" => {
                                        eq.band3.amp = ControlTarget::gain_from_f32(value);
                                    },
                                    "q" => {
                                        eq.band3.q = ControlTarget::q_from_f32(value);
                                    },
                                    "model" => {},
                                    _ => return Err(errors::CmdError{})
                                }
                            },
                            "band4" => {
                                match path[4] {
                                    "freq" => {
                                        eq.band4.freq = ControlTarget::freq_from_i32(value as i32);
                                    },
                                    "amp" => {
                                        eq.band4.amp = ControlTarget::gain_from_f32(value);
                                    },
                                    "q" => {
                                        eq.band4.q = ControlTarget::q_from_f32(value);
                                    },
                                    "model" => {},
                                    _ => return Err(errors::CmdError{})
                                }
                            },
                            "band5" => {
                                match path[4] {
                                    "freq" => {
                                        eq.band5.freq = ControlTarget::freq_from_i32(value as i32);
                                    },
                                    "amp" => {
                                        eq.band5.amp = ControlTarget::gain_from_f32(value);
                                    },
                                    "q" => {
                                        eq.band5.q = ControlTarget::q_from_f32(value);
                                    },
                                    "model" => {},
                                    _ => return Err(errors::CmdError{})
                                }
                            },
                            "band6" => {
                                match path[4] {
                                    "freq" => {
                                        eq.band6.freq = ControlTarget::freq_from_i32(value as i32);
                                    },
                                    "amp" => {
                                        eq.band6.amp = ControlTarget::gain_from_f32(value);
                                    },
                                    "q" => {
                                        eq.band6.q = ControlTarget::q_from_f32(value);
                                    },
                                    "model" => {},
                                    _ => return Err(errors::CmdError{})
                                }
                            },
                            _ => {
                                return Err(errors::CmdError{})
                            }
                        },
                        _ => ()
                    }
                }
            },
            "dyn" => return Err(errors::CmdError{}),
            _ => return Err(errors::CmdError{})
        },
        "ping" => {},
        "state" => {},
        _ => return Err(errors::CmdError{})
    }

    Ok(())
}