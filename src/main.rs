extern crate rand;

use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
    let (a, b) = point_in_circle_i32(2_000u32);
    println!("Round -> X: {}, Y: {}", a, b);

    let (c, d) = point_in_cricle_f64(200.12461324);
    println!("Non Round -> X: {}, Y: {}", c, d);
}

// Function that returns a point within a circle with the defined radius
// Returned values are i32
fn point_in_circle_i32(radius: u32) -> (i32, i32) {
        let (a, b) = point_in_cricle_f64(radius as f64);
        return (a.round() as i32, b.round() as i32);
}

// Function that returns a point within a circle with the defined radius
// Returned values are f64
fn point_in_cricle_f64(radius: f64) -> (f64, f64) {
    let t: f64 = 2.0 * PI * rand::random::<f64>();
    let u: f64 = rand::random::<f64>() + rand::random::<f64>();
    let r: f64;
    if u > 1.0 {
        r = 2.0 - u;
    } else {
        r = u;
    }

    let a: f64 = (radius as f64) * r * t.cos();
    let b: f64 = (radius as f64) * r * t.sin();
    return (a, b);
}
