// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem::swap;

pub fn main() {
    let mut x = 3i; let mut y = 7i;
    swap(&mut x, &mut y);
    assert!((x == 7)); assert!((y == 3));
}
