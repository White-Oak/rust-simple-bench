#![feature(test)]
extern crate test;
extern crate time;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::iter::*;

    #[bench]
    fn process_array(b: &mut Bencher) {
        const W: i32 = 25600;
        const H: i32 = 2048;
        const SIZE: usize = (W * H) as usize;
        let mut v: Vec<i32> = repeat(0).take(SIZE).collect();
        b.iter(|| {
            for i in 0..W {
                for j in 0..H{
                    v[(i + W * j) as usize] = i * j;
                } 
            }
        });
        println!("Element is {}", v[(W + 1) as usize]);
    }
}
