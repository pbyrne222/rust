// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_assignment)]
#![allow(unknown_features)]
#![feature(box_syntax)]

pub fn main() {
    let i = box 1i;
    let mut j = box 2i;
    // Should drop the previous value of j
    j = i;
    assert_eq!(*j, 1);
}
