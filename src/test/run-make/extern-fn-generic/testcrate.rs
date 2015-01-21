// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "lib"]

#[repr(C)]
pub struct TestStruct<T> {
    pub x: u8,
    pub y: T
}

pub extern "C" fn foo<T>(ts: TestStruct<T>) -> T { ts.y }

#[link(name = "test")]
extern {
    pub fn call(c: extern "C" fn(TestStruct<i32>) -> i32) -> i32;
}
