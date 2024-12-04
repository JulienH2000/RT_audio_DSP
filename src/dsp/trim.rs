use crate::control::control_utils::ControlTarget;
use crate::{dsp, math_utils, SYSTEM_DSP_CHANNEL_BUFFER, SYSTEM_DSP_SAMPLE_RATE, INV_SAMPLE_RATE};

#[derive(Clone, Debug, Copy)]
pub struct Trim {
    pub index: u8,
    pub amp: ControlTarget
}

impl Trim {
    pub fn new (index: u8) -> Self {
        return Trim {
            index: index,
            amp: ControlTarget::amp(1f32),
        }
    }

    pub fn next_buffer (&mut self, buffer: &mut [i32; SYSTEM_DSP_CHANNEL_BUFFER]) {

        let amp;
        if let ControlTarget::amp(a) = self.amp {
            amp = a;
        } else {panic!()}

        for s in buffer {

            *s = (*s as f32 * amp) as i32;
            math_utils::clamp_s24(s);

            };
    }
}