// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(linkage)]

#[no_mangle]
#[linkage = "external"]
static BAZ: i32 = 21;

extern {
    fn what() -> i32;
}

fn main() {
    unsafe {
        assert_eq!(what(), BAZ);
    }
}
