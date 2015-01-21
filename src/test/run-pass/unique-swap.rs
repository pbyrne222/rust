// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unknown_features)]
#![feature(box_syntax)]

use std::mem::swap;

pub fn main() {
    let mut i = box 100i;
    let mut j = box 200i;
    swap(&mut i, &mut j);
    assert_eq!(i, box 200i);
    assert_eq!(j, box 100i);
}
