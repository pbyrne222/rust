// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate dylib;
extern crate both;

use std::mem;

fn main() {
    assert_eq!(unsafe { mem::transmute::<&int, uint>(&both::foo) },
               dylib::addr());
}
