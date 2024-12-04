use std::collections::{BTreeMap, HashMap};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, Thread};

use alsa::direct::pcm::Control;
use zmq::{self, Socket};
use crate::dsp::{self, Dsp};

use crate::{control, SYSTEM_DSP_CHANNELS};

use super::control_utils::ControlTarget;


pub fn launch_socket (dsp_slots: Arc<Mutex<Vec<Dsp>>>) {

    let crtl_slots = Arc::clone(&dsp_slots);
    let state_slots = Arc::clone(&dsp_slots);

    let _ctrl_socket = thread::spawn(move || {

        let context = zmq::Context::new();
        let socket = context.socket(zmq::REP).unwrap();
        socket.bind("tcp://127.0.0.1:6061").expect("msg");

        let mut msg = zmq::Message::new();

          loop {
            let recv = socket.recv(&mut msg, 0);
            match recv {
                Ok(_) => {
                    //let parcel = json::parse(msg.as_str().unwrap()).unwrap();
                    let string = msg.as_str().unwrap();
                    //println!("{}", string);
                    if string == "exit" {
                        println!("Publisher is shutting down..."); // Show a loading screen (eg)
                    } else if string == "connected" {
                        println!("Publisher is connected."); // Hide a loading screen (eg)
                    } else {
                        //println!("NEW MESSAGE: {}, at socket:{:?}", string, socket.get_last_endpoint().unwrap());    
                        parse_command(string, Arc::clone(&crtl_slots), &socket); 
                        socket.send("message received", 0).unwrap(); 
                    }
                }
                Err(e) => {
                    //println!("Error From reccv: {}", e);
                }
            }

        }

    });

    let _ping = thread::spawn(move || {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::REP).unwrap();
        socket.bind("tcp://127.0.0.1:6062").expect("msg");
        let mut msg = zmq::Message::new();

        loop {
            let recv = socket.recv(&mut msg, 0);
            match recv {
                Ok(_) => {
                        socket.send("Ping received", 0).unwrap();
                        //println!("Ping received !");
                }
                Err(e) => {
                    println!("Error From reccv: {}", e);
                }
            }
        std::thread::sleep(std::time::Duration::from_millis(750));
        }
    });

    let _state_socket = thread::spawn(move || {
        let context = zmq::Context::new();
        let publisher = context.socket(zmq::PUB).unwrap();
        publisher.bind("tcp://127.0.0.1:6063").expect("msg");

        let dsp_slots = Arc::clone(&state_slots);

        loop {
            let dsp_slots = dsp_slots.lock().unwrap();
            let dsps = dsp_slots.clone();
            drop(dsp_slots);
            let mut peaks = [0f32; SYSTEM_DSP_CHANNELS];
            let mut rms = [0f32; SYSTEM_DSP_CHANNELS];
            for dsp in dsps.iter() {
                match dsp {
                    Dsp::Meter(m) => {
                            peaks[m.index as usize] = m.peak;
                            rms[m.index as usize] = m.rms;
                        },
                    _ => (),
                }
            }

        
            publisher.send(&format!("{{ \"peakchannel1\": {}, \"peakchannel2\": {},  \"rmschannel1\": {},  \"rmschannel2\": {}}}", peaks[0], peaks[1], rms[0], rms[1]), 0).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

}

fn parse_command(string: &str, dsp_slots: Arc<Mutex<Vec<Dsp>>>, socket: &Socket)
 {
    let parcel: Vec<&str> = string.split(':').collect();
    let target = parcel[0].trim();
    let param = parcel[1].trim();
    let value = parcel[2].trim();

    let mut dsp_slots = dsp_slots.lock().unwrap();

    match target {
        "lpf" => {
            for dsp in dsp_slots.iter_mut() {
                match dsp {
                    Dsp::LPF(lpf) => {
                        match param {
                            "bypass" => lpf.bypass = ControlTarget::bypass_from_bool(value.parse::<bool>().unwrap()),
                            "freq" => lpf.freq = ControlTarget::freq_from_i32(value.parse::<i32>().unwrap()),
                            _ => println!("Unknown parameters for lpf filter !!")
                        }
                    },
                    _ => (),
                }
            }
        }
        "hpf" => {
            for dsp in dsp_slots.iter_mut() {
                match dsp {
                    Dsp::HPF(hpf) => {
                        match param {
                            "bypass" => hpf.bypass = ControlTarget::bypass_from_bool(value.parse::<bool>().unwrap()),
                            "freq" => hpf.freq = ControlTarget::freq_from_i32(value.parse::<i32>().unwrap()),
                            _ => println!("Unknown parameters for hpf filter !!")
                        }
                    },
                    _ => (),
                }
            }
        }
        "trim" => {
            for dsp in dsp_slots.iter_mut() {
                match dsp {
                    Dsp::Trim(t) => {
                        match param {
                            "amp" => t.amp = ControlTarget::gain_from_f32(value.parse::<f32>().unwrap()),
                            _ => println!("Unknown parameters for trim !!")
                        }
                    },
                    _ => (),
                }
            }
        }
        "band" => {
            for dsp in dsp_slots.iter_mut() {
                match dsp {
                    Dsp::BF(band) => {
                        match param {
                            "bypass" => band.bypass = ControlTarget::bypass_from_bool(value.parse::<bool>().unwrap()),
                            "freq" => band.freq = ControlTarget::freq_from_i32(value.parse::<i32>().unwrap()),
                            "amp" => band.amp = ControlTarget::gain_from_f32(value.parse::<f32>().unwrap()),
                            "q" => band.q = ControlTarget::q_from_f32(value.parse::<f32>().unwrap()),
                            _ => println!("Unknown parameters for band filter !!")
                        }
                    },
                    _ => (),
                }
            } 
        }
        _ => return,
    }



}