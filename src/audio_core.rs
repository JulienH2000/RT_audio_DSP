use std::collections::BTreeMap;
use std::sync::{Arc,Mutex};

use alsa::Card;
use alsa::card::Iter;
use alsa::device_name::HintIter;
use alsa::ctl::{Ctl, DeviceIter};
use alsa::{Direction, Error};

use crate::control::control_utils::{self, ControlTarget};
use crate::dsp::{band, eq, filter, metering, trim};
use crate::dsp::Dsp;
use crate::errors::RunningError;
use crate::{errors, SYSTEM_DSP_BUFFER_LENGTH, SYSTEM_DSP_CHANNEL_BUFFER, SYSTEM_DSP_CHANNELS};
use crate::control;

pub fn init_io (dsp: &SystemDspSettings) -> Result<(Option<alsa::PCM>, Option<alsa::PCM>), errors::HardwareError> {

    let devname = "plughw:CARD=sndrpihifiberry,DEV=0";

    let ip_pcm = match alsa::PCM::new(&devname, alsa::Direction::Capture, false) {
        Ok(pcm) => Some(pcm),
        Err(_) => None
    };

    let op_pcm = match alsa::PCM::new(&devname, alsa::Direction::Playback, false) {
        Ok(pcm) => Some(pcm),
        Err(_) => None
    };

    let _ip_rate = match init_params(dsp, &ip_pcm) {
        Ok(rate) => Some(rate),
        Err(_) => None,
    };
    let _op_rate = match init_params(dsp, &op_pcm) {
        Ok(rate) => Some(rate),
        Err(_) => None
    };
 
    Ok((ip_pcm, op_pcm))

}

fn init_params (dsp: &SystemDspSettings, pcm: &Option<alsa::PCM>) -> Result<u32, errors::HardwareError>{



    let pcm = match pcm {
        Some(option_pcm) => option_pcm,
        None => return Err(errors::HardwareError::new("No PCM !!")),
    };

    {
        let hwp = alsa::pcm::HwParams::any(&pcm).unwrap();

        hwp.set_channels(dsp.channels as u32).unwrap();

        println!("{:?} {:?}", hwp.get_rate_min(), hwp.get_rate_max());
        hwp.set_rate(dsp.sample_rate, alsa::ValueOr::Nearest).unwrap();

        hwp.test_format(alsa::pcm::Format::S24LE).unwrap();
        hwp.set_format(alsa::pcm::Format::S24LE).unwrap();

        //hwp.get_access().unwrap();
        hwp.set_access(alsa::pcm::Access::RWInterleaved).unwrap();

        println!("{:?} {:?}", hwp.get_buffer_size_min(), hwp.get_buffer_size_max());
        hwp.set_buffer_size(dsp.buffer_size).unwrap();

        println!("{:?} {:?}", hwp.get_period_size_min(), hwp.get_period_size_max());
        hwp.set_period_size(dsp.buffer_size / 2, alsa::ValueOr::Nearest).unwrap();


        pcm.hw_params(&hwp).unwrap();
    }

    let rate = {
        let hwp = pcm.hw_params_current().unwrap();
        let swp = pcm.sw_params_current().unwrap();
        

        let (bufsize, periodsize) = (hwp.get_buffer_size().unwrap(), hwp.get_period_size().unwrap());
        swp.set_start_threshold(bufsize - periodsize).unwrap();
        swp.set_avail_min(periodsize).unwrap();
        pcm.sw_params(&swp).unwrap();

        match pcm.info().unwrap().get_stream() {
            alsa::Direction::Playback => {
                println!("Opened audio output with parameters: {:?}, \n{:?}", hwp, swp);
            },
            alsa::Direction::Capture => {
                println!("Opened audio input with parameters: {:?}, \n{:?}", hwp, swp);
            },
        }

        hwp.get_rate().unwrap()
    };

    return Ok(rate);
}

pub fn read_from_inputs (pcm: &alsa::PCM, io: &mut alsa::pcm::IO<i32>, buffer: &mut [i32; SYSTEM_DSP_BUFFER_LENGTH]) -> Result<(), RunningError>{

    let avail = match pcm.avail_update() {
        Ok(n) => n,
        Err(e) => {
            println!("\x1b[31mReading Recovering from {}\x1b[0m", e);
            pcm.recover(e.errno() as std::os::raw::c_int, true).unwrap();
            pcm.avail_update().unwrap()
        }
    } as usize;

    if avail > 0 {
        let _ = io.readi(buffer);
    }
    use alsa::pcm::State;
    match pcm.state() {
        State::Running => Ok(()),
        State::Prepared => { println!("\x1b[32mStarting audio input stream\x1b[0m"); 
        pcm.start().unwrap(); Err(RunningError::new()) },
        State::Suspended | State::XRun => Err(RunningError::new()),
        n @ _ => Err(format!("Unexpected pcm state {:?}", n)).unwrap(),
    }
}

pub fn feed_to_outputs (pcm: &alsa::PCM, io: &mut alsa::pcm::IO<i32>, buffer: &[i32; SYSTEM_DSP_BUFFER_LENGTH]) -> Result<bool, Error>{

    let avail = match pcm.avail_update() {
        Ok(n) => n,
        Err(e) => {
            println!("\x1b[31mWritting Recovering from {}\x1b[0m", e);
            pcm.recover(e.errno() as std::os::raw::c_int, true).unwrap();
            pcm.avail_update().unwrap()
        }
    } as usize;

    if avail > 0 {
        let _ = io.writei(buffer);
    }
    use alsa::pcm::State;
    match pcm.state() {
        State::Running => Ok(false),
        State::Prepared => { println!("\x1b[32mStarting audio output stream\x1b[0m"); 
        pcm.start()?; Ok(true) },
        State::Suspended | State::XRun => Ok(true),
        n @ _ => Err(format!("Unexpected pcm state {:?}", n)).unwrap(),
    }

}


#[derive(Copy, Debug, Clone)]
pub struct NonInterleavedBuffer {
    pub buf: [[i32; SYSTEM_DSP_CHANNEL_BUFFER]; SYSTEM_DSP_CHANNELS],
    pub index: usize,
}

impl NonInterleavedBuffer {
    pub fn deinterleave (i_buffer: &[i32; SYSTEM_DSP_BUFFER_LENGTH]) -> Self {
        /*
            [f32; buffer]---+
            [f32; buffer]   |
            [f32; buffer]   | xChannels
            [f32; buffer]   |
            [...]....    ---+
    
        */
        let mut n_buffers = [[0i32; SYSTEM_DSP_CHANNEL_BUFFER]; SYSTEM_DSP_CHANNELS];
    
        // (i, (Li, Ri)) 
        let iter_buffer = i_buffer.chunks(SYSTEM_DSP_CHANNELS).enumerate();
    
        for (i, samples) in iter_buffer {
            for (b, s) in samples.iter().enumerate() {
                n_buffers[b][i] = *s;
    
            }
        }
        return NonInterleavedBuffer { buf: n_buffers, index : 0 }
    }

    pub fn interleave (&mut self) -> [i32; SYSTEM_DSP_BUFFER_LENGTH] {

        let mut o_buffer = [0i32; SYSTEM_DSP_BUFFER_LENGTH];
        
        let mut i = 0;
        for buffer in self.buf {
            for j in 0..buffer.len() {
                o_buffer[i + 2*j] = buffer[j];
            }
            i = i +1;
        }

        return o_buffer;
    }
    
}

impl Iterator for NonInterleavedBuffer {
    type Item = [i32; SYSTEM_DSP_CHANNEL_BUFFER];

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let item = Some(self.buf[self.index]);
        if self.index < self.buf.len() - 1 {
            self.index = self.index + 1;
        } else {
            self.index = 0;
            return None;
        }

        return item
        
    }

}

//pub fn deinterleave_handle (n_buffers: &[[f32; SYSTEM_DSP_CHANNEL_BUFFER]; SYSTEM_DSP_CHANNELS], ch_idx: usize) -> [f32; SYSTEM_DSP_CHANNEL_BUFFER] { return n_buffers[ch_idx]; }

pub enum BusType {
    Master,
    Group,
    Aux,
    Track,
    DirectOut
}

pub struct SystemDspSettings {
    pub sample_rate: u32,
    pub format: alsa::pcm::Format,
    pub buffer_size: alsa::pcm::Frames,
    pub channels: usize,
}

impl SystemDspSettings {

    pub fn new (sr: u32, format: alsa::pcm::Format, bf: alsa::pcm::Frames, ch: usize) -> Self {
        return SystemDspSettings { sample_rate: sr, format: format, buffer_size: bf, channels: ch }
    }
}

pub struct DspIo {
    // I/O instances are only a reference to a buffer existing elsewhere
    pub buffer_index: u8,
}

pub struct Path {
    pub index: u8,
    pub label: Option<String>,
    pub leg_width: u8,
}

pub struct Bus {
    pub index: u8,
    pub label: Option<String>,
    pub leg_with: u8,
    pub bus_type: BusType,
}

pub fn invoke_from_init (init: String) -> Option<Arc<Mutex<Vec<Dsp>>>> {
    let mut slots: Vec<Dsp> = vec![];
    for slot in init.trim().split(",") {
        match slot {
            "lpf" => {
                for i in 0..SYSTEM_DSP_CHANNELS as u8 {
                    slots.push(Dsp::LPF(filter::LPF::new(i)));
                }
            },
            "hpf" => {
                for i in 0..SYSTEM_DSP_CHANNELS as u8 {
                    slots.push(Dsp::HPF(filter::HPF::new(i)));
                }
            },
            "trim" => {
                for i in 0..SYSTEM_DSP_CHANNELS as u8 {
                    slots.push(Dsp::Trim(trim::Trim::new(i)));
                }
            },
            "band" => {
                for i in 0..SYSTEM_DSP_CHANNELS as u8 {
                    slots.push(Dsp::BF(band::BF::new(i)));
                }
            },
            "meter" => {
                for i in 0..SYSTEM_DSP_CHANNELS as u8 {
                    slots.push(Dsp::Meter(metering::Meter::new(i)));
                }
            },
            "eq" => {
                for i in 0..SYSTEM_DSP_CHANNELS as u8 {
                    slots.push(Dsp::EQ(eq::Eq::new(i)));
                }
            }
            _ => return None
        }
    }
    return Some(Arc::new(Mutex::new(slots)))
}