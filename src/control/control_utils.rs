use crate::dsp;

#[derive(Clone, Debug, Copy)]
pub enum ControlTarget {
    bypass(bool),
    freq(i32),
    amp(f32),
    q(f32),
    ratio(f32),
    thsh(i32),
    atk(i32),
    rel(i32),
    eqmodel(dsp::eq::EqModel)
}

impl ControlTarget {
    pub fn bypass_from_bool (bool: bool) -> ControlTarget {
        return ControlTarget::bypass(bool);
    }
    pub fn freq_from_i32 (freq: i32) -> ControlTarget {
        return ControlTarget::freq(freq);
    }

    pub fn inner_bypass (&self) -> bool {
        if let ControlTarget::bypass(b) = self {
            return *b
        } else { panic!() }
    }

    pub fn gain_from_f32 (amp: f32) -> ControlTarget {
        return ControlTarget::amp(amp)
    }

    pub fn q_from_f32 (q: f32) -> ControlTarget {
        return ControlTarget::q(q)
    }

    pub fn eqmodel_from_string (s: &str) -> ControlTarget {
        return match s {
            "hpf" => ControlTarget::eqmodel(dsp::eq::EqModel::HighPass),
            "lpf" => ControlTarget::eqmodel(dsp::eq::EqModel::LowPass),
            "bell" => ControlTarget::eqmodel(dsp::eq::EqModel::Bell),
            _ => ControlTarget::eqmodel(dsp::eq::EqModel::Bell)
        }
    }
}