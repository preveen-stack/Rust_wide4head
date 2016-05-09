fn main() {
    println!("Hello, world!");


	fn plus_one(i : i32) -> i32 {
		i + 1
	}
	//without type inference
	let f: fn(i32) -> i32 = plus_one;

	//with type inference
	let f = plus_one;

	let six = f(5);

	let a = [1, 2, 3]; // a [i32 : 3]
	let mut m = [1, 2, 3]; // m [i32 ; 3]
}
