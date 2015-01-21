// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os;
use std::thread::Thread;

fn f(n: uint) {
    let mut i = 0u;
    while i < n {
        let _ = Thread::scoped(move|| g()).join();
        i += 1u;
    }
}

fn g() { }

fn main() {
    let args = os::args();
    let args = if os::getenv("RUST_BENCH").is_some() {
        vec!("".to_string(), "400".to_string())
    } else if args.len() <= 1u {
        vec!("".to_string(), "10".to_string())
    } else {
        args.into_iter().collect()
    };
    let n = args[1].parse().unwrap();
    let mut i = 0u;
    while i < n { Thread::spawn(move|| f(n) ); i += 1u; }
}
