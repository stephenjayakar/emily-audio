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

fn validate_array_branch_free(arr: [i16; 48000]) -> bool {
    let lookup = [1, -1, 0, 0, 0];
    for i in 0..48000 {
        let expected = lookup[(i % 250) / 50];
        if arr[i] != expected {
            return false;
        }
    }
    return true;
}

fn validate_array_simd_branch_free_1(arr: [i16; 48000]) -> bool {
    let lookup = [1, -1, 0, 0, 0];
    for i in 0..12000 {
        let x = i16x4::from_array([
            arr[i],
            arr[i + 12000],
            arr[i + 24000],
            arr[i + 36000],
        ]);
        let should_equal_val = lookup[(i % 250) / 50];
        let should_equal = i16x4::from_array([should_equal_val, should_equal_val, should_equal_val, should_equal_val]);
        if x != should_equal {
            return false;
        }
    }
    return true;
}

fn validate_array_simd_branch_free_2(arr: [i16; 48000]) -> bool {
    let a = i16x4::from_array([1, 1, 1, 1]);
    let b = i16x4::from_array([-1, -1, -1, -1]);
    let c = i16x4::from_array([0, 0, 0, 0]);
    let lookup = [a, b, c, c, c];
    for i in 0..12000 {
        let x = i16x4::from_array([
            arr[i],
            arr[i + 12000],
            arr[i + 24000],
            arr[i + 36000],
        ]);
        let should_equal_index = (i % 250) / 50;
        let should_equal = lookup[should_equal_index];
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
    let c = timeit(|| validate_array_branch_free(input::MY_ARRAY));
    println!("serial branch free {}", c);
    let d = timeit(|| validate_array_simd_branch_free_1(input::MY_ARRAY));
    println!("simd branch free lookup small {}", d);
    let e = timeit(|| validate_array_simd_branch_free_2(input::MY_ARRAY));
    println!("simd branch free lookup large {}", e);
}

fn timeit<F: Fn() -> T, T>(f: F) -> T {
  let start = SystemTime::now();
  let result = f();
  let end = SystemTime::now();
  let duration = end.duration_since(start).unwrap();
  println!("it took {}ms", duration.as_secs_f64() * 1000.0);
  result
}
