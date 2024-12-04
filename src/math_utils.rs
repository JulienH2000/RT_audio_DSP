pub fn s24_to_i32 (s: &i32) -> i32 {

    if (s & (1 << 23 )) != 0 {
        return s | 0xFF << size_of::<i32>() * 8 - 8 
    } else {
        return *s
    }
}

pub fn clamp_s24 (s: &mut i32) {
    *s = (*s).clamp(-8388608, 8388607);
}

pub fn fast_log10(x: f32) -> f32 {
    const SCALE_FACTOR: f32 = 0.30102999566; // 1 / log2(10)
    let bits: u32 = x.to_bits();
    let log2 = ((bits >> 23) & 0xff) as i32 - 127;
    let mantissa = (bits & 0x7fffff) | 0x800000;
    let log2_frac = (mantissa as f32) / 8388608.0;
    (log2 as f32 + log2_frac.log2()) * SCALE_FACTOR
}
