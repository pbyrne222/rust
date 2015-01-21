// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::num::Int;

trait BrokenAdd: Int {
    fn broken_add<T>(&self, rhs: T) -> Self {
        *self + rhs //~  ERROR mismatched types
                    //~| expected `Self`
                    //~| found `T`
                    //~| expected Self
                    //~| found type parameter
    }
}

impl<T: Int> BrokenAdd for T {}

pub fn main() {
    let foo: u8 = 0u8;
    let x: u8 = foo.broken_add("hello darkness my old friend".to_string());
    println!("{}", x);
}
