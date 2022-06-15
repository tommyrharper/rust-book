pub fn production_rate_per_hour(speed: u8) -> f64 {
    f64::from(speed) * f64::from(221)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    match speed {
        1..=4 => (production_rate_per_hour(speed) / f64::from(60)) as u32,
        5..=8 => (production_rate_per_hour(speed) * 0.9 / f64::from(60)) as u32,
        _ => (production_rate_per_hour(speed) * 0.77 / f64::from(60)) as u32
    }
}

fn main() {
    let res = production_rate_per_hour(7);
    println!("res: {}", res);
}