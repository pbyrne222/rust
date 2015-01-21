// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(asm)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn main() {
    let x: int;
    unsafe {
        // Treat the output as initialization.
        asm!("mov $1, $0" : "=r"(x) : "r"(5u));
    }
    assert_eq!(x, 5);

    let mut x = x + 1;
    assert_eq!(x, 6);

    unsafe {
        // Assignment to mutable.
        asm!("mov $1, $0" : "=r"(x) : "r"(x + 7));
    }
    assert_eq!(x, 13);
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn main() {}
