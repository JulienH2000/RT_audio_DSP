/*

+-----------------------------------------------------------------------+
|
|
|
|
|
|
|
|
|
+-----------------------------------------------------------------------+

*/

use std::{collections::BTreeMap, sync::{mpsc::channel, Arc, Mutex}, vec};

use control::{socket_handler, web_server};
use errors::RunningError;
use dsp::filter;

pub mod audio_core;
pub mod dsp;
pub mod errors;
pub mod math_utils;
pub mod control;

const UI_SERVER: &'static str = "ws://localhost:6060";

const SYSTEM_DSP_CHANNELS: usize = 2;
const SYSTEM_DSP_CHANNEL_BUFFER: usize = 128;
const SYSTEM_DSP_BUFFER_LENGTH: usize = SYSTEM_DSP_CHANNEL_BUFFER * SYSTEM_DSP_CHANNELS;
const SYSTEM_DSP_SAMPLE_RATE: u32 = 48000;
const INV_SAMPLE_RATE: f32 = 1.0 / SYSTEM_DSP_SAMPLE_RATE as f32 * 0.957; //0.00002083
const PPM_HOLD_TIME: u32 = 600 * 48; // samples = ms * samples per ms 
const RMS_INTEGRATION_TIME: u32 = 300 * 48;
const INV_RMS_INTEGRATION_TIME: f32 = 1f32 / ( 300f32 * 48f32 );


fn main() {


    let dsp = audio_core::SystemDspSettings::new(SYSTEM_DSP_SAMPLE_RATE, alsa::pcm::Format::s24(), (SYSTEM_DSP_BUFFER_LENGTH*2) as i64, SYSTEM_DSP_CHANNELS);

    let (ip_pcm, op_pcm) = match audio_core::init_io(&dsp) {
        Ok((ip,op)) => (ip,op),
        Err(errors::HardwareError {message}) => {
            panic!("Hardware error occured trying to init audio device(s) !! : {}", message);
        },
    };
    let ip_pcm = match ip_pcm {
        Some(pcm) => pcm,
        None => panic!("No Input PCM provided !!"),
    };
    let mut ip_io = ip_pcm.io_i32_s24().unwrap();

    let op_pcm = match op_pcm {
        Some(pcm) => pcm,
        None => panic!("No Output PCM provided !!")
    };
    let mut op_io = op_pcm.io_i32_s24().unwrap();

    let system_init = "trim,lpf,hpf,band,meter".to_string();

    let dsp_slots = match audio_core::invoke_from_init(system_init) {
        Some(slots) => slots,
        None => {println!("System DSP Slots Init failed ! ");
            panic!();
        }
    };

    //println!("DSP Slots initialized :{:?}", dsp_slots);

    {
    let mut dsps = dsp_slots.lock().unwrap();
    for dsp in dsps.iter_mut() {
    //println!("\x1b[33mDSP Slot initialized :{:?}", dsp);
    }
    }

    //socket_handler::launch_socket(Arc::clone(&dsp_slots));
    web_server::server::start(Arc::clone(&dsp_slots));

    let mut buffer = [0i32; SYSTEM_DSP_BUFFER_LENGTH];
    
    'audio: loop {

        // Feed all inputs into buffer
        match audio_core::read_from_inputs(&ip_pcm, &mut ip_io, &mut buffer) {
            Ok(r) => r,
            Err(e) => match e {
                errors::RunningError {}=> continue 'audio,
                _ => panic!()
            }
        }


        /* DSP happens here */
        dsp::dsp_handler(&mut buffer, Arc::clone(&dsp_slots));

        // Feed all buffers to outputs
        audio_core::feed_to_outputs(&op_pcm, &mut op_io, &mut buffer).unwrap();
    }

}
