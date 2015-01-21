// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::thread::Thread;

pub fn main() {
    Thread::scoped(move|| child(10)).join().ok().unwrap();
}

fn child(i: int) { println!("{}", i); assert!((i == 10)); }
