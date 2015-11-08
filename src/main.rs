extern crate time;

use std::thread;
use std::mem::size_of;

fn main() {
    const W: i32 = 25600;
    const H: i32 = 2048;
    const SIZE: usize = (W * H) as usize;
    let child = thread::Builder::new()
        .stack_size(SIZE * size_of::<i32>() * 2)
        .spawn(move || {
            let mut v:[i32; SIZE] = [0; SIZE];
            let mills = time::precise_time_ns();
            for i in 0..W {
                for j in 0..H{
                    v[(i + W * j) as usize] = i * j;
                } 
            }
            let total = time::precise_time_ns() - mills;
            println!("Time is {}", total / 1000 / 1000);
            println!("Element is {}", v[(W + 1) as usize]);
            }).unwrap();
    child.join();
}
