// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct S {
    x: u64,
    y: u64,
    z: u64,
}

impl Copy for S {}

#[link(name = "rust_test_helpers")]
extern {
    pub fn get_x(x: S) -> u64;
    pub fn get_y(x: S) -> u64;
    pub fn get_z(x: S) -> u64;
}

#[inline(never)]
fn indirect_call(func: unsafe extern fn(s: S) -> u64, s: S) -> u64 {
    unsafe {
        func(s)
    }
}

fn main() {
    let s = S { x: 1, y: 2, z: 3 };
    assert_eq!(s.x, indirect_call(get_x, s));
    assert_eq!(s.y, indirect_call(get_y, s));
    assert_eq!(s.z, indirect_call(get_z, s));
}
