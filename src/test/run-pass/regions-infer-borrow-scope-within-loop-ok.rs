// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unknown_features)]
#![feature(box_syntax)]

fn borrow<T>(x: &T) -> &T {x}

pub fn main() {
    let x = box 3i;
    loop {
        let y = borrow(&*x);
        assert_eq!(*x, *y);
        break;
    }
}
