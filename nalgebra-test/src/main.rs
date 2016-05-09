extern crate nalgebra;

use nalgebra::{Mat2, Rot2, Vec2};

fn main() {
	let v =  Vec2::new(0.0f64, 1.0);
	//90 degrees clockwise
	// 0, 1
	// -1, 0
	let rot = Mat2::new(0.0f64, -1.0, 1.0, 0.0);
	println!("{:?}", rot * v);
    println!("Hello, world!");
}
