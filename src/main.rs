// 48k
// 250
// 192 chunks
// 8 threads

// 50 1s, 50 -1s, 150 0s.
// 48000

#![feature(exclusive_range_pattern)]
#![feature(portable_simd)]

use std::simd::{i16x4, i16x8};

mod input;

use std::time::SystemTime;
use rayon::prelude::*;

fn validate_array(arr: [i16; 48000]) -> bool {
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

fn validate_array_parallel(arr: [i16; 48000]) -> bool {
    // try slices.chunks https://doc.rust-lang.org/std/primitive.slice.html#method.chunks
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

// change to 128b
fn validate_array_simd(arr: [i16; 48000]) -> bool {
    let a = i16x4::from_array([1, 1, 1, 1]);
    let b = i16x4::from_array([-1, -1, -1, -1]);
    let c = i16x4::from_array([0, 0, 0, 0]);
    for i in 0..12000 {
        let x = i16x4::from_array([
            arr[i],
            arr[i + 12000],
            arr[i + 24000],
            arr[i + 36000],
        ]);
        let should_equal = match i % 250 {
            0..50 => {
                a
            },
            50..100 => {
                b
            },
            _ => {
                c
            }
        };
        if x != should_equal {
            return false;
        }
    }
    return true;
}

// fn i_to_val(i: usize) -> i16 {
//     match i % 250 {
//             0..50 => {
//                 1
//             },
//             50..100 => {
//                 -1
//             },
//             _ => {
//                 0
//             }
//     }
// }

fn validate_array_simd_cache(arr: [i16; 48000]) -> bool {
    let a = i16x4::from_array([1, 1, 1, 1]);
    let b = i16x4::from_array([-1, -1, -1, -1]);
    let c = i16x4::from_array([0, 0, 0, 0]);
    let d = i16x4::from_array([1, 1, -1, -1]);
    let e = i16x4::from_array([0, 0, 1, 1]);
    let f = i16x4::from_array([-1, -1, 0, 0]);
    for i in (0..48000).step_by(4) {
        unsafe {
            let x = i16x4::from_slice(&arr.get_unchecked(i..i+4));
            let y = match i % 500 {
                0..48 => {
                    a
                },
                48 => {
                    d
                },
                49..100 => {
                    b
                },
                100..248 => {
                    c
                },
                248 => {
                    e
                },
                252..300 => {
                    a
                },
                300..348 => {
                    b
                },
                348 => {
                    f
                },
                352..500 => {
                    c
                }
                _ => {
                    std::hint::unreachable_unchecked()
                }
            };
            if x != y {
                return false;
            }
        }
    };
    return true;
}

fn validate_array_simd_bigger(arr: [i16; 48000]) -> bool {
    let a = i16x8::from_array([1, 1, 1, 1, 1, 1, 1, 1]);
    let b = i16x8::from_array([-1, -1, -1, -1, -1, -1, -1, -1]);
    let c = i16x8::from_array([0, 0, 0, 0, 0, 0, 0, 0]);
    for i in 0..6000 {
        let x = i16x8::from_array([
            arr[i],
            arr[i + 6000],
            arr[i + 12000],
            arr[i + 18000],
            arr[i + 24000],
            arr[i + 32000],
            arr[i + 36000],
            arr[i + 42000],
        ]);
        let should_equal = match i % 250 {
            0..50 => {
                a
            },
            50..100 => {
                b
            },
            _ => {
                c
            }
        };
        if x != should_equal {
            return false;
        }
    }
    return true;
}

fn main() {
    let x = timeit(|| validate_array(input::MY_ARRAY));
    println!("serial result {}", x);
    let y = timeit(|| validate_array_parallel(input::MY_ARRAY));
    println!("parallel result {}", y);
    let z = timeit(|| validate_array_simd(input::MY_ARRAY));
    println!("simd result {}", z);
    let a = timeit(|| validate_array_simd_cache(input::MY_ARRAY));
    println!("simd result cached {}", a);
    let b = timeit(|| validate_array_simd_bigger(input::MY_ARRAY));
    println!("simd result bigger width {}", b);
    ////////////////////////////////
    // Create two vectors of i32 values
    // let a = i16x4::from_array([1, 2, 3, 4]);
    // let b = i16x4::from_array([5, 6, 7, 8]);

    // // Perform SIMD operations on the vectors
    // let c = a + b; // => i16x4::new(6, 8, 10, 12)
    // let d = a * b; // => i16x4::new(5, 12, 21, 32)

    // // You can also use the u32x4 and i32x8/u32x8 types for
    // // unsigned 32-bit integers and longer vectors, respectively

    // // The values in the vector are stored in registers, which you
    // // can access using the `.0`, `.1`, `.2`, and `.3` fields:
    // println!("{:?}", c.as_array());
}

fn timeit<F: Fn() -> T, T>(f: F) -> T {
  let start = SystemTime::now();
  let result = f();
  let end = SystemTime::now();
  let duration = end.duration_since(start).unwrap();
  println!("it took {}ms", duration.as_secs_f64() * 1000.0);
  result
}
