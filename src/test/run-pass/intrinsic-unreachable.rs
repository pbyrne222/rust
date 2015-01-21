// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::intrinsics;

// See also src/test/run-make/intrinsic-unreachable.

unsafe fn f(x: uint) -> uint {
    match x {
        17 => 23,
        _ => intrinsics::unreachable(),
    }
}

fn main() {
    assert_eq!(unsafe { f(17) }, 23);
}
