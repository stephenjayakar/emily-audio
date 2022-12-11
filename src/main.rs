// 48k
// 250
// 192 chunks
// 8 threads

#![feature(exclusive_range_pattern)]

mod input;

use std::time::SystemTime;
use rayon::prelude::*;

fn validate_array(arr: [i32; 48000]) -> bool {
    for i in 0..48000 {
        let flag: bool = match i % 250 {
            0..50 => {
                arr[i] == 1
            },
            50..100 => {
                arr[i] == -1
            },
            _ => {
                arr[i] == 0
            }
        };
        if !flag {
            return false;
        }
    }
    return true;
}
// thread 0 does 0
// thread 1 does 1
// thread 0 does 2

fn validate_array_parallel(arr: [i32; 48000]) -> bool {
    (0..48000).into_par_iter().all(|i| {
        match i % 250 {
            0..50 => {
                arr[i] == 1
            },
            50..100 => {
                arr[i] == -1
            },
            _ => {
                arr[i] == 0
            }
        }
    })
}


fn main() {
    let x = timeit(|| validate_array(input::MY_ARRAY));
    println!("serial result {}", x);
    let y = timeit(|| validate_array_parallel(input::MY_ARRAY));
    println!("parallel result {}", y);
}

fn timeit<F: Fn() -> T, T>(f: F) -> T {
  let start = SystemTime::now();
  let result = f();
  let end = SystemTime::now();
  let duration = end.duration_since(start).unwrap();
  println!("it took {}ms", duration.as_secs_f64() * 1000.0);
  result
}
