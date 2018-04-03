extern crate rand;

use std::f64::consts::PI;
use rand::Rng;

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();

    let num = 150i32;
    let radius = 3 * num;
    //let rectangles: [Rectangle: num];
    for _n in 0..num {
        let (a, b) = point_in_circle_i32(radius as u32);
        let point1 = Point { x: a, y: b};

        println!("Point{{ x: {}, y: {} }}", point1.x, point1.y);
    }
}

// fn gen_rectangles(size: &str) -> Rectangle {
fn gen_rectangles() -> () {
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
