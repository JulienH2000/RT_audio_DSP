use std::sync::{Arc, Mutex};
use crate::math_utils;

pub mod filter;
pub mod trim;
pub mod band;
pub mod dynamics;
pub mod metering;
pub mod eq;

#[derive(Clone, Debug)]
pub enum Dsp {
    LPF(filter::LPF),
    HPF(filter::HPF),
    BF(band::BF),
    Trim(trim::Trim),
    Meter(metering::Meter),
    EQ(eq::Eq)
}

/* !! Note that LPF, HPF and BF are legacy since the full EQ implementation */
pub fn dsp_handler (buffer: &mut [i32; crate::SYSTEM_DSP_BUFFER_LENGTH], dsp_slots: Arc<Mutex<Vec<Dsp>>>) {

    let mut dsp_slots = dsp_slots.lock().unwrap();

    // deinterleave the buffer, ready for processing
    let mut n_buffers = crate::audio_core::NonInterleavedBuffer::deinterleave(&buffer);

    for (i, buffer) in n_buffers.buf.iter_mut().enumerate() {
        for s in buffer.iter_mut() {
            *s = math_utils::s24_to_i32(s);
        }

        for dsp in dsp_slots.iter_mut() {
            match dsp {
                // Legacy DSP Model
                Dsp::LPF(lpf) => if i as u8 == lpf.index {
                    if lpf.bypass.inner_bypass() == false {
                        lpf.next_buffer(buffer);
                    }
                },
                // Legacy DSP Model

                Dsp::HPF(hpf) => if i as u8 == hpf.index {
                    if hpf.bypass.inner_bypass() == false {
                        hpf.next_buffer(buffer);
                    }
                },
                // Legacy DSP Model
                Dsp::BF(b) => if  i as u8 == b.index {
                    if b.bypass.inner_bypass() == false {
                        b.next_buffer(buffer);
                    }
                },
                Dsp::Trim(t) => if i as u8 == t.index {
                    t.next_buffer(buffer);
                },
                Dsp::Meter(m) => if i as u8 == m.index {
                    m.level_detect(buffer);
                },
                Dsp::EQ(e) => if i as u8 == e.index {
                    e.next_buffer(buffer);
                }
            }
        }

    }

    // interleave back
    *buffer = n_buffers.interleave();

}