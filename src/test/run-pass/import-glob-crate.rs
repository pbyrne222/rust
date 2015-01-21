// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_assignment)]

use std::mem::*;

pub fn main() {
    assert_eq!(size_of::<u8>(), 1);
    let (mut x, mut y) = (1i, 2i);
    swap(&mut x, &mut y);
    assert_eq!(x, 2);
    assert_eq!(y, 1);
}
