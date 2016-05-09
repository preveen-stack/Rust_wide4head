extern crate fuse;

use std::env;
use fuse::Filesystem;

struct JsonFilesystem;

impl Filesystem for JsonFilesystem {
}

fn main() {
    println!("Hello, world!");
}
