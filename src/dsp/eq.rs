use std::f32::consts::PI;

use crate::control::control_utils::ControlTarget;
use crate::{dsp, math_utils, SYSTEM_DSP_CHANNEL_BUFFER, SYSTEM_DSP_SAMPLE_RATE, INV_SAMPLE_RATE};

#[derive(Clone, Debug, Copy)]
pub enum EqModel {
    //HighShelf,
    //LowShelf,
    HighPass,
    LowPass,
    Bell
}

#[derive(Clone, Debug, Copy)]
pub struct Band {
    pub model: ControlTarget,
    pub freq: ControlTarget,
    pub bypass: ControlTarget,
    pub amp: ControlTarget,
    pub q: ControlTarget,
}

impl Band {
    fn new (model: ControlTarget, freq: ControlTarget, bypass: ControlTarget, amp: ControlTarget, q: ControlTarget) -> Self {
        return Band {
            model: model,
            freq: freq,
            bypass: bypass,
            amp: amp,
            q: q
        }
    }
                                // model,  freq, amp, q, bypass
    fn extract_params (&self) -> (EqModel, i32, f32, f32, bool) {

        let model;
        if let ControlTarget::eqmodel(m) = self.model {
            model = m;
        } else {panic!()}

        let freq: i32;
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

        let b: bool;
        if let ControlTarget::bypass(bp) = self.bypass {
            b = bp;
        } else {panic!()}
        
        return (model, freq, A, q, b)
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Eq {
    pub index: u8,
    pub band1: Band,
    pub band2: Band,
    pub band3: Band,
    pub band4: Band,
    pub band5: Band,
    pub band6: Band,
    input_sample_minus_one: i32,
    input_sample_minus_two: i32,
    output_sample_minus_one: i32,
    output_sample_minus_two: i32,
}

impl Eq {
    pub fn new (index: u8) -> Self {
        return Eq {
            index: index,
            band1: Band::new(ControlTarget::eqmodel(EqModel::HighPass), ControlTarget::freq(120), ControlTarget::bypass(true), ControlTarget::amp(1f32), ControlTarget::q(1f32)),
            band2: Band::new(ControlTarget::eqmodel(EqModel::Bell), ControlTarget::freq(340), ControlTarget::bypass(true), ControlTarget::amp(1f32), ControlTarget::q(1f32)),
            band3: Band::new(ControlTarget::eqmodel(EqModel::Bell), ControlTarget::freq(600), ControlTarget::bypass(true), ControlTarget::amp(1f32), ControlTarget::q(1f32)),
            band4: Band::new(ControlTarget::eqmodel(EqModel::Bell), ControlTarget::freq(1500), ControlTarget::bypass(true), ControlTarget::amp(1f32), ControlTarget::q(1f32)),
            band5: Band::new(ControlTarget::eqmodel(EqModel::Bell), ControlTarget::freq(3000), ControlTarget::bypass(true), ControlTarget::amp(1f32), ControlTarget::q(1f32)),
            band6: Band::new(ControlTarget::eqmodel(EqModel::LowPass), ControlTarget::freq(20000), ControlTarget::bypass(true), ControlTarget::amp(1f32), ControlTarget::q(1f32)),
            input_sample_minus_one: 0i32,
            input_sample_minus_two: 0i32,
            output_sample_minus_one: 0i32,
            output_sample_minus_two: 0i32,
        }
    }

    pub fn next_buffer (&mut self, buffer: &mut [i32; SYSTEM_DSP_CHANNEL_BUFFER]) {

        let band1 = self.band1.extract_params();
        let band2 = self.band2.extract_params();
        let band3 = self.band3.extract_params();
        let band4 = self.band4.extract_params();
        let band5 = self.band5.extract_params();
        let band6 = self.band6.extract_params();

        let band1coefs = match band1.0 {
            EqModel::HighPass => hpf_coefs(band1.1),
            EqModel::LowPass => lpf_coefs(band1.1),
            EqModel::Bell => bell_coefs(band1.1, band1.2, band1.3)
        };
        let band2coefs = match band2.0 {
            EqModel::HighPass => hpf_coefs(band2.1),
            EqModel::LowPass => lpf_coefs(band2.1),
            EqModel::Bell => bell_coefs(band2.1, band2.2, band2.3)
        };
        let band3coefs = match band3.0 {
            EqModel::HighPass => hpf_coefs(band3.1),
            EqModel::LowPass => lpf_coefs(band3.1),
            EqModel::Bell => bell_coefs(band3.1, band3.2, band3.3)
        };
        let band4coefs = match band4.0 {
            EqModel::HighPass => hpf_coefs(band4.1),
            EqModel::LowPass => lpf_coefs(band4.1),
            EqModel::Bell => bell_coefs(band4.1, band4.2, band4.3)
        };
        let band5coefs = match band5.0 {
            EqModel::HighPass => hpf_coefs(band5.1),
            EqModel::LowPass => lpf_coefs(band5.1),
            EqModel::Bell => bell_coefs(band5.1, band5.2, band5.3)
        };
        let band6coefs = match band6.0 {
            EqModel::HighPass => hpf_coefs(band6.1),
            EqModel::LowPass => lpf_coefs(band6.1),
            EqModel::Bell => bell_coefs(band6.1, band6.2, band6.3)
        };

        for sample in buffer {

            self.input_sample_minus_one  = *sample;

            if band1.4 {self.next_sample(sample, band1coefs);}
            if band2.4 {self.next_sample(sample, band2coefs);}
            if band3.4 {self.next_sample(sample, band3coefs);}
            if band4.4 {self.next_sample(sample, band4coefs);}
            if band5.4 {self.next_sample(sample, band5coefs);}
            if band6.4 {self.next_sample(sample, band6coefs);}

            self.input_sample_minus_two  = self.input_sample_minus_one;
            self.output_sample_minus_two = self.output_sample_minus_one;
            self.output_sample_minus_one = *sample;
        }
    }

    fn next_sample (&mut self, sample: &mut i32, (a0, a1, a2, b0, b1): (f32, f32, f32, f32, f32)) {
        let next_sample = 
            ( a0 * *sample as f32) + 
            ( a2 * self.input_sample_minus_two as f32) + 
            ( a1 * self.input_sample_minus_one as f32) - 
            ( b0 * self.output_sample_minus_one as f32) - 
            ( b1 * self.output_sample_minus_two as f32);
        *sample = next_sample as i32;
    }
}

fn hpf_coefs (freq: i32) -> (f32, f32, f32, f32, f32) {
    let r = 1.4;
    let f0 = freq as f32 * INV_SAMPLE_RATE;
    let c: f32;
    if f0 < 0.05 {
        c = f0 * PI;
    } else {
        c = (f0 * PI).tan();
    }
    let a0 = 1.0 / ( 1.0 + r * c + c * c);
    let a1 = -2.0 * a0;
    let a2 = a0;
    let b0 = 2.0 * ( c*c - 1.0) * a0;
    let b1 = ( 1.0 - r * c + c * c) * a0;

    return (a0, a1, a2, b0, b1);
}

fn lpf_coefs (freq: i32) -> (f32, f32, f32, f32, f32) {
    let r = 1.4;
    let f0 = freq as f32 * INV_SAMPLE_RATE; 
    let c: f32;
    if f0 < 0.1 {
        c = 1.0 / (f0 * PI);
    } else {
        c = ((0.5 - f0) * PI).tan(); 
    }
    let a0 = 1.0 / ( 1.0 + r * c + c * c); 
    let a1 = 2.0 * a0; 
    let a2 = a0; 
    let b0 = 2.0 * ( 1.0 - c*c) * a0; 
    let b1 = ( 1.0 - r * c + c * c) * a0;

    return (a0, a1, a2, b0, b1);
}

fn bell_coefs (freq: i32, amp: f32, q: f32) -> (f32, f32, f32, f32, f32) {
    let f0 = freq as f32 * INV_SAMPLE_RATE; //0.010416
    let w = 2f32 * PI * f0;
    let bw: f32 = w.sin() / (2f32 * q);

    let a0 = 1.0 + (bw / amp);
    let a1 = (-2f32 * w.cos()) / a0;
    let a2 = (1.0 - (bw / amp)) / a0;
    let b0 = (1.0 + (bw * amp)) / a0;
    let b1 = a1;
    let b2 = (1.0 - (bw * amp)) / a0;

    return (b0, b1, b2, a1, a2);
}