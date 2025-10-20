use std::ops::Deref;
use std::error::Error;

const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

impl Deref for SnowKg {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

impl Deref for SnowLb {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl Deref for Snowball {
    type Target = i64;

    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl From<SnowKg> for Snowball {
    fn from(kg: SnowKg) -> Self {
        let snowballs = (*kg / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball(snowballs)
    }
}

impl From<SnowLb> for Snowball {
    fn from(lb: SnowLb) -> Self {
        let snowballs = (*lb / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball(snowballs)
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub area: f64,
    pub snow: Snowball,
}

impl Location {
    pub fn new<T: Into<Snowball>>(x: f64, y: f64, z: f64, area: f64, snow: T) -> Location {
        Location {
            x,
            y,
            z,
            area,
            snow: snow.into(),
        }
    }

    pub fn density(&self) -> f64 {
        if self.area == 0.0 {
            0.0
        } else {
            (*self.snow as f64) / self.area
        }
    }
}

pub fn find_best_location(locations: Vec<Location>) -> Result<Location, Box<dyn Error>> {
    if locations.is_empty() {
        return Err("No locations provided".into());
    }

    let mut best = locations[0].clone();

    for location in &locations[1..] {
        if location.density() > best.density() {
            best = location.clone();
        }
    }

    Ok(best)
}

fn main() {
    // Example usage
    let locations = vec![
        Location::new(1.0, 2.0, 3.0, 100.0, SnowKg(5.0)),
        Location::new(4.0, 5.0, 6.0, 50.0, SnowLb(11.0)),
        Location::new(7.0, 8.0, 9.0, 75.0, Snowball(25)),
    ];

    match find_best_location(locations) {
        Ok(best) => {
            println!("Best location found at ({}, {}, {})", best.x, best.y, best.z);
            println!("Density: {:.2} snowballs per unit area", best.density());
        }
        Err(e) => println!("Error: {}", e),
    }
}
