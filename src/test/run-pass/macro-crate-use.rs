// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn increment(x: uint) -> uint {
    x + 1
}

#[macro_export]
macro_rules! increment {
    ($x:expr) => ({
        use $crate::increment;
        increment($x)
    })
}

fn main() {
    assert_eq!(increment!(3), 4);
}
