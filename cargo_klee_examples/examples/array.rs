// get_sign.rs
// Showcase how we automatically can interface Rust to KLEE
//

#![no_std]
#![no_main]

use klee_sys::klee_make_symbolic;
use panic_klee as _;

fn sum_first_elements(arr: &[u8], index: usize) -> u16 {
    let mut acc: u16 = 0;
    for i in 0..index {
        if index < arr.len() {
            acc += arr[i as usize] as u16;
         } else {
            break;
        }
    }
    acc
}

#[no_mangle]
fn main() {
    let mut arr = [0u8; 8];  
    for i in 0..arr.len() {
        let mut var: u8 = 0;
        klee_make_symbolic!(&mut var, "num");
        arr[i] = var;
    }

    let mut i: usize = 0;
    klee_make_symbolic!(&mut i, "i");
    let b = sum_first_elements(&arr, i);
}

// A) Array indexing is tricky to analyse at compile time.
// Thus Rust (rustc) will inject code for run-time verification
// `panic`ing on index out of range.
//
// (Compare to C/C++, where a "buffer overflow" might pass unnoticed
// causing all sorts of problems.)
//
// Compare the test generated in release `--release` (optimized) to
// test generated in debug/dev mode (un-optimized).
//
// Try to explain in your own words the difference and why?
// (Hint, even if we don't use the result `b`, Rust do not optimize out the call, why?)
//
// [your answer here]
// On debug the generated tests are 10 but on release there are only 2 generated tests.
// Debug checks the index value from 0..8 and then 255. Whereas release only checks the values
// 0..1. I get the feeling that since KLEE checks indices 0..8 because that is the length of the
// array. And perhaps the Rust compiler do some optimizations in release mode so that the array is
// smaller i.e. 2 bytes instead of 8.
//
// B) Fix the code so that you don't get an error.
// (It should still compute the sum of the n first elements
// and return the sum of the whole array if index larger than size/length).
// The fix should be in the function (not on the caller side).
//
// [Git commit "B"]
//
// C) In the example, the array is holding only zeroes.
// Figure out a way to make the content symbolic.
// (Hint, declare as mutable, iterate and set each element symbolic.)
// (Hint2, it seems that you can set the whole array symbolic directly
// without messing with an iterator, super!!!.)
//
// [Git commit "C"]
//
// D) Analyze the example using KLEE. Now a new (maybe unexpected) error should occur!
// Notice, the error occurs only in `debug/dev` builds.
//
// Explain what caused the error.
//
// [your answer here]
// In debug compilation: 
// The first and second value in the array are both 248. When adding them
// the accumulator value (u8) will overflow and cause a panic.
//
// E) Make a sensible fix to the code.
// Motivate your choice.
//
// [your answer here]
// Let the accumulator variable be an u16 instead. Thus we can accumulate a value that
// is 256 times larger than an u8. One would need an array of size larger than size
// 256 to be able to overflow that variable.
// This is more sensible than doing a wrapping add in my opinion as then you would get a value
// than you might not expect in debug. For release mode it will wrap anyway.
//
// [Git commit "D"]
//
// F) Learning outcome.
// 70% of Microsoft security updates over the last decade is directly related to
// memory safety.
//
// Explain in your own words what Microsoft would gain by using Rust.
// [your answer here]
// It would most likely lead to less issues with memory bugs. And since the
// Rust compiler is much more strict of what you can do, it would probably
// lead to less developer headaches in the long-run. 
//
//
// Explain in your own words what Microsoft would gain by using `cargo klee`
// on their Rust code.
// [your answer here]
// If they want to utilize KLEE it would make it easier for them to integrate integrate
// it into their worflows.
//
// And YES, Microsoft is rewriting core system functionality in Rust as we speak!
