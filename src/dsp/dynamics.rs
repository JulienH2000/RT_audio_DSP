use crate::control::control_utils::ControlTarget;
use crate::{dsp, math_utils, SYSTEM_DSP_CHANNEL_BUFFER, SYSTEM_DSP_SAMPLE_RATE, INV_SAMPLE_RATE};


#[derive(Clone, Debug, Copy)]
pub struct Dyn {
    pub index: u8,
    pub model: DynModel,
    pub ratio: ControlTarget,
    pub threshhold: ControlTarget,
    pub attack: ControlTarget,
    pub release: ControlTarget,
    pub makeup: ControlTarget,
    pub bypass: ControlTarget,
    input_sample_minus_one: i32,
    input_sample_minus_two: i32,
    output_sample_minus_one: f32,
    output_sample_minus_two: f32,
}

#[derive(Clone, Debug, Copy)]
pub enum DynModel {
    compressor,
    limiter,
    brickwall
}

impl Dyn {
    pub fn new_compressor (index: u8) -> Self {
        return Dyn {
            index: index,
            model: DynModel::compressor,
            ratio: ControlTarget::ratio(1f32),
            threshhold: ControlTarget::thsh(0),
            attack: ControlTarget::atk(40),
            release: ControlTarget::rel(200),
            makeup: ControlTarget::amp(1f32),
            bypass: ControlTarget::bypass(false),
            input_sample_minus_one: 0i32,
            input_sample_minus_two: 0i32,
            output_sample_minus_one: 0f32,
            output_sample_minus_two: 0f32,
        }
    }

    pub fn compressor (&mut self, buffer: &mut [i32; SYSTEM_DSP_CHANNEL_BUFFER]) {

        let makeup;
        if let ControlTarget::amp(m) = self.makeup {
            makeup = m;
        }

        let ratio;
        if let ControlTarget::ratio(r) = self.ratio {
            ratio = r;
        }

        let thresh;
        if let ControlTarget::thsh(t) = self.threshhold {
            thresh = t;
        }       
        
        let atk;
        if let ControlTarget::atk(at) = self.attack {
            atk = at;
        }       
        
        let rel;
        if let ControlTarget::rel(rl) = self.release {
            rel = rl;
        }       
        
        let bypass;
        if let ControlTarget::bypass(b) = self.bypass {
            bypass = b;
        }

        for s in buffer {


        };
    }
}