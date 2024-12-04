use std::collections::VecDeque;

use crate::{dsp, math_utils, INV_RMS_INTEGRATION_TIME, PPM_HOLD_TIME, RMS_INTEGRATION_TIME, SYSTEM_DSP_CHANNEL_BUFFER};

const INV_I24_MAX: f32 = 1f32 / 8388607f32 ;

#[derive(Clone, Debug)]
pub struct Meter {
    pub index: u8,
    pub peak: f32,
    cycle_count: u32,
    pub rms: f32,
    buf: VecDeque<f32>,
    sum: f32
}

impl Meter {
    pub fn new (index: u8) -> Self {
        return Meter {
            index: index,
            peak: 0f32,
            cycle_count: 1u32,
            rms: 0f32,
            buf: VecDeque::from([0f32; RMS_INTEGRATION_TIME as usize]),
            sum: 0f32
        }
    }
    pub fn level_detect (&mut self, buffer: &[i32; SYSTEM_DSP_CHANNEL_BUFFER]) {
        // dBFS peak level
        let mut instant_level:f32;
        for s in buffer {
            instant_level = 20.0 * math_utils::fast_log10((*s as f32).abs() * INV_I24_MAX); // Sample to dBFS
        
            if instant_level > self.peak {
                self.peak = instant_level;
            } else if self.cycle_count == PPM_HOLD_TIME {
                self.peak = instant_level;
                self.cycle_count = 1;
            }
            self.buf.pop_front().unwrap();
            self.buf.push_back(instant_level);
            self.sum -= self.buf.front().unwrap();
            self.sum += self.buf.back().unwrap();
            self.rms = self.sum * INV_RMS_INTEGRATION_TIME;
            self.cycle_count += 1;
        }
    }
}