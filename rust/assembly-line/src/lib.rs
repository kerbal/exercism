// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn success_rate(speed: u8) -> f64 {
    if (1 <= speed && speed <= 4) { return 1.0 }
    if (5 <= speed && speed <= 8) { return 0.9 }
    return 0.77;
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    return success_rate(speed) * 221.0 * (speed as f64);
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0) as u32;
}
