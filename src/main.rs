extern crate rand;

use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
    let radius: i64 = 150;
    for _n in 1..2000 {
        let t: f64 = 2.0 * PI * rand::random::<f64>();
        let u: f64 = rand::random::<f64>() + rand::random::<f64>();
        let r: f64;
        if u > 1.0 {
            r = 2.0 - u
        } else {
            r = u
        }

        let a: f64 = (radius as f64) * r * t.cos();
        let b: f64 = (radius as f64) * r * t.sin();
        println!("{},{}", a.round() as i64, b.round() as i64);
    }

}
