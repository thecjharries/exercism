const CARS_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        1..=4 => CARS_PER_HOUR * (speed as f64),
        5..=8 => CARS_PER_HOUR * (speed as f64) * 0.9,
        9..=10 => CARS_PER_HOUR * (speed as f64) * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
