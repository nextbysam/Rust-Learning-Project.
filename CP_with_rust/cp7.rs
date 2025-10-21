// CP7: Understanding Vectors vs Structs
// This example demonstrates the difference between:
// 1. A struct (single object with fields)
// 2. A vector (collection of multiple objects)

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

// This is ONE Location (a struct with fields)
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

// This function receives MULTIPLE Locations (a vector)
pub fn find_best_location(locations: Vec<Location>) -> Result<Location, Box<dyn Error>> {
    // locations = a vector/array containing many Location objects
    // locations[0] = get the first Location from the vector
    // locations[1] = get the second Location from the vector

    if locations.is_empty() {
        return Err("No locations provided".into());
    }

    // Get the FIRST Location from the vector
    let mut best = locations[0].clone();
    //             ^^^^^^^^^^^^
    //             Index into the vector to get Location at index 0

    // Loop through the REST of the locations (from index 1 onwards)
    for location in &locations[1..] {
    //  ^^^^^^^^    ^^^^^^^^^^^^^^
    //  Each item   Slice from index 1 to end
        if location.density() > best.density() {
            best = location.clone();
        }
    }

    Ok(best)
}

fn main() {
    println!("=== Understanding Vectors vs Structs ===\n");

    // Create ONE Location (a struct)
    let single_location = Location::new(1.0, 2.0, 3.0, 100.0, SnowKg(5.0));
    println!("Single Location:");
    println!("  x: {}", single_location.x);  // Access field with .x
    println!("  y: {}", single_location.y);  // Access field with .y
    println!("  density: {:.2}\n", single_location.density());

    // Create MULTIPLE Locations (a vector)
    let locations = vec![
        Location::new(1.0, 2.0, 3.0, 100.0, SnowKg(5.0)),    // locations[0]
        Location::new(4.0, 5.0, 6.0, 50.0, SnowLb(11.0)),     // locations[1]
        Location::new(7.0, 8.0, 9.0, 75.0, Snowball(25)),     // locations[2]
    ];

    println!("Vector of Locations:");
    println!("  Number of locations: {}", locations.len());
    println!("\n  Location at index 0:");
    println!("    x: {}", locations[0].x);  // Index into vector, then access field
    println!("    density: {:.2}", locations[0].density());

    println!("\n  Location at index 1:");
    println!("    x: {}", locations[1].x);
    println!("    density: {:.2}", locations[1].density());

    println!("\n  Location at index 2:");
    println!("    x: {}", locations[2].x);
    println!("    density: {:.2}", locations[2].density());

    // Find the best location
    println!("\n=== Finding Best Location ===");
    match find_best_location(locations) {
        Ok(best) => {
            println!("Best location found at ({}, {}, {})", best.x, best.y, best.z);
            println!("Density: {:.2} snowballs per unit area", best.density());
        }
        Err(e) => println!("Error: {}", e),
    }
}
