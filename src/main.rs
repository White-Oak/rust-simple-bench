extern crate time;

    use std::iter::*;
    use time::*;

    fn main() {
    const W: i32 = 25600;
    const H: i32 = 2048;
    const SIZE: usize = (W * H) as usize;
    let mut v: Vec<i32> = repeat(0).take(SIZE).collect();
    let start = precise_time_ns();
    for i in 0..W {
        for j in 0..H{
            v[(i + W * j) as usize] = i * j;
        } 
    }
    let total = precise_time_ns() - start;
    println!("{}", total / 1000 / 1000);
}
