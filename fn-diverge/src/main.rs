fn main() {
    println!("Hello, world!");
	diverges();
}

fn diverges() -> ! {
	panic!("this function never returns!");
}
