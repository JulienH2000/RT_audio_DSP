use std::f32::consts::PI;
use crate::control::control_utils::ControlTarget;
use crate::{math_utils, SYSTEM_DSP_CHANNEL_BUFFER, INV_SAMPLE_RATE};

#[derive(Clone, Debug, Copy)]
pub struct LPF {
    pub index: u8,
    pub freq: ControlTarget,
    pub bypass: ControlTarget,
    input_sample_minus_one: i32,
    input_sample_minus_two: i32,
    output_sample_minus_one: f32,
    output_sample_minus_two: f32,
    a0: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
}

impl LPF {
    pub fn new (index: u8) -> Self {

        let mut lpf = LPF { 
            index: index,
            freq: ControlTarget::freq(20000i32),
            bypass: ControlTarget::bypass(false),
            input_sample_minus_one: 0i32,
            input_sample_minus_two: 0i32,
            output_sample_minus_one: 0f32,
            output_sample_minus_two: 0f32,
            a0: 0f32,
            a1: 0f32,
            a2: 0f32,
            b0: 0f32,
            b1: 0f32,
        };
        let r = 1.4;
        let f0 = 20000f32 * INV_SAMPLE_RATE; //0.010416
        let c: f32;
        if f0 < 0.1 {
            c = 1.0 / (f0 * PI); // 30.57814
        } else {
            c = ((0.5 - f0) * PI).tan(); //30.54683998694403
        }
        lpf.a0 = 1.0 / ( 1.0 + r * c + c * c); //1.0670489636350725e-3
        lpf.a1 = 2.0 * lpf.a0; // 2.134097927270145e-3
        lpf.a2 = lpf.a0; //1.0670489636350725e-3
        lpf.b0 = 2.0 * ( 1.0 - c*c) * lpf.a0; //-1.989212809355381
        lpf.b1 = ( 1.0 - r * c + c * c) * lpf.a0; //0.9934810052099211

        return lpf
    }

    pub fn next_buffer (&mut self, buffer: &mut [i32; SYSTEM_DSP_CHANNEL_BUFFER]) {

        let freq;
        if let ControlTarget::freq(f) = self.freq {
            freq = f;
        } else {panic!()}

        let r = 1.4;
        let f0 = freq as f32 * INV_SAMPLE_RATE; //0.010416
        let c: f32;
        if f0 < 0.1 {
            c = 1.0 / (f0 * PI); // 30.57814
        } else {
            c = ((0.5 - f0) * PI).tan(); //30.54683998694403
        }
        let a0 = 1.0 / ( 1.0 + r * c + c * c); //1.0670489636350725e-3
        let a1 = 2.0 * a0; // 2.134097927270145e-3
        let a2 = a0; //1.0670489636350725e-3
        let b0 = 2.0 * ( 1.0 - c*c) * a0; //-1.989212809355381
        let b1 = ( 1.0 - r * c + c * c) * a0; //0.9934810052099211

        for s in buffer {
            *s = math_utils::s24_to_i32(s);

            let next_sample = 
                (a0 * *s as f32) + 
                (a2 * self.input_sample_minus_two as f32) + 
                (a1 * self.input_sample_minus_one as f32) - 
                (b0 * self.output_sample_minus_one) - 
                (b1 * self.output_sample_minus_two);

                self.input_sample_minus_two  = self.input_sample_minus_one;
                self.input_sample_minus_one  = *s as i32;
                self.output_sample_minus_two = self.output_sample_minus_one;
                self.output_sample_minus_one = next_sample;

            *s = next_sample as i32;
            math_utils::clamp_s24(s);
            //*s = 0f32;

            };
    }

}

#[derive(Clone, Debug, Copy)]
pub struct HPF {
    pub index: u8,
    pub freq: ControlTarget,
    pub bypass: ControlTarget,
    input_sample_minus_one: i32,
    input_sample_minus_two: i32,
    output_sample_minus_one: f32,
    output_sample_minus_two: f32,
    a0: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
}

impl HPF {
    pub fn new (index: u8) -> Self {
        let mut hpf = HPF { 
            index: index,
            freq: ControlTarget::freq(20i32),
            bypass: ControlTarget::bypass(false),
            input_sample_minus_one: 0i32,
            input_sample_minus_two: 0i32,
            output_sample_minus_one: 0f32,
            output_sample_minus_two: 0f32,
            a0: 0f32,
            a1: 0f32,
            a2: 0f32,
            b0: 0f32,
            b1: 0f32,
        };
        let r = 1.4;
        let f0 = 20f32 * INV_SAMPLE_RATE; //0.010416
        let c: f32;
        if f0 < 0.05 {
            c = f0 * PI; // 30.57814
        } else {
            c = (f0 * PI).tan(); //30.54683998694403
        }

        hpf.a0 = 1.0 / ( 1.0 + r * c + c * c); //1.0670489636350725e-3
        hpf.a1 = -2.0 * hpf.a0; // 2.134097927270145e-3
        hpf.a2 = hpf.a0; //1.0670489636350725e-3
        hpf.b0 = 2.0 * ( c*c - 1.0) * hpf.a0; //-1.989212809355381
        hpf.b1 = ( 1.0 - r * c + c * c) * hpf.a0; //0.9934810052099211

        return hpf

    }

    pub fn next_buffer (&mut self, buffer: &mut [i32; SYSTEM_DSP_CHANNEL_BUFFER]){

        let freq;
        if let ControlTarget::freq(f) = self.freq {
            freq = f;
        } else {panic!()}

        let r = 1.4;
        let f0 = freq as f32 * INV_SAMPLE_RATE; //0.010416
        let c: f32;
        if f0 < 0.05 {
            c = f0 * PI; // 30.57814
        } else {
            c = (f0 * PI).tan(); //30.54683998694403
        }
        self.a0 = 1.0 / ( 1.0 + r * c + c * c); //1.0670489636350725e-3
        self.a1 = -2.0 * self.a0; // 2.134097927270145e-3
        self.a2 = self.a0; //1.0670489636350725e-3
        self.b0 = 2.0 * ( c*c - 1.0) * self.a0; //-1.989212809355381
        self.b1 = ( 1.0 - r * c + c * c) * self.a0; //0.9934810052099211

        for s in buffer{

            let next_sample = 
                (self.a0 * *s as f32) + 
                (self.a2 * self.input_sample_minus_two as f32) + 
                (self.a1 * self.input_sample_minus_one as f32) - 
                (self.b0 * self.output_sample_minus_one) - 
                (self.b1 * self.output_sample_minus_two);

                self.input_sample_minus_two  = self.input_sample_minus_one;
                self.input_sample_minus_one  = *s;
                self.output_sample_minus_two = self.output_sample_minus_one;
                self.output_sample_minus_one = next_sample;

            *s = next_sample as i32;

            };
    }

}