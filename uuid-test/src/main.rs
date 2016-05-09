extern crate uuid;

use uuid::Uuid;


fn main() {
	for  _ in 0..10 {
		println!("{}", Uuid::new_v4().hyphenated().to_string());
	}
    println!("Hello, world!");
}
