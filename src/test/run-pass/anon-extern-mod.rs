// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

#[link(name = "rust_test_helpers")]
extern {
    fn rust_get_test_int() -> libc::intptr_t;
}

pub fn main() {
    unsafe {
        let _ = rust_get_test_int();
    }
}
