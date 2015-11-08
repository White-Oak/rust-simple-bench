extern crate time;

use time::precise_time_ns;
use std::thread;
use std::mem::size_of;
use std::io::*;
use std::fs::OpenOptions;
use std::path::Path;

fn main() {
    const W: i32 = 25600;
    const H: i32 = 2048;
    const SIZE: usize = (W * H) as usize;
    let child = thread::Builder::new()
        .stack_size(SIZE * size_of::<i32>() * 2)
        .spawn(move || {
            let mut v: [i32; SIZE] = [0; SIZE];
            let start = precise_time_ns();
            for i in 0..W {
                for j in 0..H{
                    v[(i + W * j) as usize] = i * j;
                } 
            }
            let total = precise_time_ns() - start;
            println!("{}", total / 1000 / 1000);
            write_to_null(v[(W+1) as usize]);
        }).unwrap();
    child.join();
}

fn write_to_null(i :i32) {
    let path = Path::new("/dev/null");
    let mut options = OpenOptions::new();
    options.write(true).append(true);
            
    let file = match options.open(path) {
        Ok(file) => file,
        Err(..)  => panic!("room"),
    };
    let mut buffer = BufWriter::new(&file);
    write!(buffer, "{}", i);
}
