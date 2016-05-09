extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};


fn main() {
	let client = Client::open("redis://127.0.0.1").unwrap();
	let conn = client.get_connection().unwrap();
	let _: () = conn.set("answer", 42).unwrap();
	let answer: i32 = conn.get("answer").unwrap();
	println!("Answer:{}", answer);
}
