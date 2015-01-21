// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(asm)]
#![crate_type="lib"]

pub fn exit(n: uint) {
    unsafe {
        // Pretend this asm is an exit() syscall.
        asm!("" :: "r"(n) :: "volatile");
        // Can't actually reach this point, but rustc doesn't know that.
    }
}
