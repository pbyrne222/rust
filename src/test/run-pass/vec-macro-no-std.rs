// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(lang_items, start)]
#![no_std]

extern crate "std" as other;

#[macro_use]
extern crate core;
extern crate libc;

#[macro_use]
extern crate collections;

use core::option::Option::Some;
use core::slice::SliceExt;
use collections::vec::Vec;

// Issue #16806

#[start]
fn start(_argc: int, _argv: *const *const u8) -> int {
    let x: Vec<u8> = vec![0, 1, 2];
    match x.last() {
        Some(&2) => (),
        _ => panic!(),
    }
    0
}
