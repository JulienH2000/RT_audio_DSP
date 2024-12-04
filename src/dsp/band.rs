use std::f32::consts::PI;

use crate::control::control_utils::ControlTarget;
use crate::{dsp, math_utils, SYSTEM_DSP_CHANNEL_BUFFER, SYSTEM_DSP_SAMPLE_RATE, INV_SAMPLE_RATE};

#[derive(Clone, Debug, Copy)]
pub struct BF {
    pub index: u8,
    pub freq: ControlTarget,
    pub bypass: ControlTarget,
    pub amp: ControlTarget,
    pub q: ControlTarget,
    input_sample_minus_one: i32,
    input_sample_minus_two: i32,
    output_sample_minus_one: f32,
    output_sample_minus_two: f32,
}

impl BF {
    pub fn new (index: u8) -> Self {
        return BF {
            index: index,
            freq: ControlTarget::freq(500i32),
            bypass: ControlTarget::bypass(false),
            amp: ControlTarget::amp(1f32),
            q: ControlTarget::q(1f32),
            input_sample_minus_one: 0i32,
            input_sample_minus_two: 0i32,
            output_sample_minus_one: 0f32,
            output_sample_minus_two: 0f32,
        }
    }

    pub fn next_buffer (&mut self, buffer: &mut [i32; SYSTEM_DSP_CHANNEL_BUFFER]) {

        let freq;
        if let ControlTarget::freq(f) = self.freq {
            freq = f;
        } else {panic!()}

        let A: f32;
        if let ControlTarget::amp(a) = self.amp {
            A = a;
        } else {panic!()}

        let q: f32;
        if let ControlTarget::q(qf) = self.q {
            q = qf;
        } else {panic!()}

        let f0 = freq as f32 * INV_SAMPLE_RATE; //0.010416
        let w = 2f32 * PI * f0;
        let bw: f32 = w.sin() / (2f32 * q);


        let a0 = 1.0 + (bw / A);
        let a1 = (-2f32 * w.cos()) / a0;
        let a2 = (1.0 - (bw / A)) / a0;
        let b0 = (1.0 + (bw * A)) / a0;
        let b1 = a1;
        let b2 = (1.0 - (bw * A)) / a0;

        for s in buffer {

            let next_sample = 
                (b0 * *s as f32) + 
                (b1 * self.input_sample_minus_one as f32) + 
                (b2 * self.input_sample_minus_two as f32) - 
                (a1 * self.output_sample_minus_one) - 
                (a2 * self.output_sample_minus_two);

                self.input_sample_minus_two  = self.input_sample_minus_one;
                self.input_sample_minus_one  = *s as i32;
                self.output_sample_minus_two = self.output_sample_minus_one;
                self.output_sample_minus_one = next_sample;

            *s = next_sample as i32;
            math_utils::clamp_s24(s);

            };
    }
}